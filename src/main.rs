use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str;
use std::thread;
pub mod models;
use clap::{App, Arg};
use models::*;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};

enum ThreadSignal<T: Model> {
    Write(T),
    Stop,
}

#[derive(Clone)]
pub struct Arguments {
    pub input: &'static str,
    pub output: &'static str,
    pub maximum_rows: i32,
    pub no_ignore: bool,
}

pub fn new_thread<'a, T: Model + Send + 'static>(arguments: Arguments) -> Sender<ThreadSignal<T>> {
    let (snd, rcv) = channel::<ThreadSignal<T>>();
    thread::spawn(move || {
        let mut count = 0;
        let file_name = format!("{}.sql", T::get_table_name());
        let file_path = PathBuf::from(arguments.output).join(file_name);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create_new(true)
            .open(file_path)
            .expect(&format!(
                "Can not open the file {}. Already exist or permission denied",
                T::get_table_name()
            ));

        write!(&file, "\n{}", T::get_create_table_query());

        loop {
            match rcv.recv() {
                Ok(result) => match result {
                    ThreadSignal::Write(entry) => {
                        count += 1;
                        let data_set = entry.get_data_set();
                        if count >= arguments.maximum_rows {
                            let columns = data_set.keys().fold(String::new(), |a, b| a + b + ",");
                            let columns = columns.trim_end_matches(",");
                            write!(
                                &file,
                                ";\n\nINSERT {} INTO {} ({}) VALUES ",
                                if arguments.no_ignore { "" } else { "IGNORE" },
                                T::get_table_name(),
                                columns
                            );
                        } else {
                            write!(&file, ",");
                        }

                        let values = data_set.values().fold(String::new(), |a, b| {
                            a + &match b {
                                SqlType::BigInt(big_int) => big_int.to_string(),
                                SqlType::Int(int) => int.to_string(),
                                SqlType::Decimal(dec) => dec.to_string(),
                                SqlType::Varchar(varchar) => String::from(varchar.clone()),
                                SqlType::Bool(b) => {
                                    String::from(if b.clone() {
                                        "1"
                                    } else {
                                        "0"
                                    })
                                }
                                SqlType::Null => String::from("NULL"),
                            } + ","
                        });
                        let values = values.trim_end_matches(",");
                        write!(&file, "({})", values);
                    }
                    ThreadSignal::Stop => {
                        break;
                    }
                },
                Err(_) => {}
            }
        }
    });

    snd
}

fn main() {
    let config = App::new("OSM-to-SQL")
        .version("0.1.3")
        .author("WhizSid <whizsid@aol.com>")
        .about("Converting open street map files to SQL files with relations.")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .required(true)
                .takes_value(true)
                .help("Sets the input OSM XML file."),
        )
        .arg(
            Arg::with_name("output")
                .short("d")
                .long("directory")
                .value_name("DIRECTORY")
                .required(true)
                .takes_value(true)
                .help("Output directory. This directory should be an empty directory."),
        )
        .arg(
            Arg::with_name("rows")
                .short("r")
                .long("rows")
                .value_name("NUMBER")
                .takes_value(true)
                .default_value("400")
                .help("Maximum rows per one SQL insert query. Default is 400"),
        )
        .arg(
            Arg::with_name("no-ignore")
                .short("g")
                .long("no-ignore")
                .help("Do not use INSERT IGNORE queries."),
        )
        .get_matches();

    let arguments = Arguments {
        input: config.value_of("input").unwrap(),
        output: config.value_of("output").unwrap(),
        maximum_rows: config.value_of("rows").unwrap().parse().unwrap(),
        no_ignore: if let Some(_) = config.value_of("no-ignore") {
            true
        } else {
            false
        },
    };

    let result = Reader::from_file(&Path::new(&arguments.input));
    let nodes = new_thread::<Node>(arguments.clone());
    let tags = new_thread::<Tag>(arguments.clone());
    let ways = new_thread::<Way>(arguments.clone());
    let way_nodes = new_thread::<WayNode>(arguments.clone());
    let relations = new_thread::<Relation>(arguments.clone());
    let relation_members = new_thread::<RelationMember>(arguments.clone());
    let ref_tags = new_thread::<UsedTag>(arguments.clone());

    let buf = Vec::new();
    let mut used_tags: Vec<&str> = vec![];
    let mut last_ref_id: i64 = 0;
    let mut last_ref_type: &str = "node";

    match result {
        Ok(mut reader) => {
            // Self closing tags
            reader.expand_empty_elements(true);

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) => {
                        let mut attrs: HashMap<&str, &str> = HashMap::new();
                        let name = e.name();
                        for attribute in e.attributes() {
                            let attribute = attribute.unwrap();
                            let key = attribute.key;
                            let value = attribute.value;
                            attrs.insert(
                                unsafe { str::from_utf8_unchecked(key.as_ref()) },
                                unsafe { str::from_utf8_unchecked(value.as_ref()) },
                            );
                        }
                        match name {
                            b"node" => {
                                let mut node: Node = Node {
                                    ..Default::default()
                                };

                                for (key, value) in attrs {
                                    if !node.main_info.set_attribute(key, value) {
                                        match key {
                                            "lat" => {
                                                node.lat = value.parse::<f32>().unwrap();
                                            }
                                            "lon" => {
                                                node.lng = value.parse::<f32>().unwrap();
                                            }
                                            _=>{}
                                        }
                                    }
                                }

                                last_ref_id = node.main_info.id;
                                last_ref_type = "node";

                                nodes.send(ThreadSignal::Write(node));
                            }
                            b"way" => {
                                let mut way: Way = Way {
                                    ..Default::default()
                                };

                                for (k, v) in attrs {
                                    way.main_info.set_attribute(k, v);
                                }

                                last_ref_id = way.main_info.id;
                                last_ref_type = "way";

                                ways.send(ThreadSignal::Write(way));
                            }
                            b"relation" => {
                                let mut relation: Relation = Relation {
                                    ..Default::default()
                                };

                                for (k, v) in attrs {
                                    relation.main_info.set_attribute(k, v);
                                }

                                last_ref_id = relation.main_info.id;
                                last_ref_type = "relation";

                                relations.send(ThreadSignal::Write(relation));
                            }
                            b"tag" => {
                                let k = attrs.get("k").expect("Can not read key attribute.");
                                let v = attrs.get("v").expect("Can not read value attribute.");

                                let tag_index = used_tags.iter().position(|t| &t == &k);

                                let tag_id = match tag_index {
                                    None => {
                                        let id = used_tags.len() as i16;
                                        let in_tag = Tag {
                                            id,
                                            name: k,
                                        };

                                        used_tags.push(k);
                                        tags.send(ThreadSignal::Write(in_tag));
                                        id
                                    }
                                    Some(index) => {index as i16}
                                };

                                ref_tags.send(ThreadSignal::Write(UsedTag {
                                    tag_id,
                                    value: v,
                                    ref_id: last_ref_id,
                                    ref_type: last_ref_type,
                                }));
                            }
                            b"nd" => {
                                let ref_attr = attrs
                                    .get("ref")
                                    .expect("Can not read the ref attribute from nd tag.");

                                way_nodes.send(ThreadSignal::Write(WayNode {
                                    way_id: last_ref_id,
                                    node_id: ref_attr.parse::<i64>().unwrap(),
                                }));
                            }
                            b"member" => {
                                let ref_attr = attrs.get("ref").expect("Can not read ref attr from member tag.");
                                let type_attr = attrs.get("type").expect("Can not read type attr from member tag.");
                                let role_attr = attrs.get("role").unwrap_or(&"");

                                relation_members.send(ThreadSignal::Write(RelationMember {
                                    ref_id: ref_attr.parse::<i64>().unwrap(),
                                    ref_type: type_attr,
                                    role: role_attr,
                                    relation_id: last_ref_id
                                }));
                                
                            }
                            _ => (),
                        }
                    }
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    _ => (),
                }
                buf.clear();
            }
        }
        Err(e) => panic!("Invalid file :- {:?}", e),
    }
}
