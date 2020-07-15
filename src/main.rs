use clap::{App, Arg};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str;
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;
use std::thread::JoinHandle;
use std::time::Duration;

pub mod models;
use models::*;

enum ThreadSignal<T: Model> {
    Write(T),
    Stop,
}

#[derive(Clone, Debug)]
pub struct Arguments {
    pub input: String,
    pub output: String,
    pub maximum_rows: i32,
    pub no_ignore: bool,
}

fn new_thread<'a, T: Model + Send + 'static>(arguments: Arguments) ->( JoinHandle<()>,Sender<ThreadSignal<T>>) {
    let (snd, rcv) = channel::<ThreadSignal<T>>();
    let handle = spawn(move || {
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

        let w = write!(&file, "{}", T::get_create_table_query());
        w.unwrap();

        loop {
            match rcv.recv_timeout(Duration::from_secs(1)) {
                Ok(result) => match result {
                    ThreadSignal::Write(entry) => {
                        count += 1;

                        let data_set = entry.get_data_set();
                        let columns = T::get_columns();
                        if count > arguments.maximum_rows || count == 1 {
                            let columns_str =
                                columns.iter().fold(String::new(), |a, b| a + b + ",");
                            let columns_str = columns_str.trim_end_matches(",");
                            let w = write!(
                                &file,
                                ";\nINSERT {} INTO {} ({}) VALUES ",
                                if arguments.no_ignore { "" } else { "IGNORE" },
                                T::get_table_name(),
                                columns_str
                            );
                            w.unwrap();

                            if count>arguments.maximum_rows {
                                count = 1;
                            }

                        } else {
                            let w = write!(&file, ",");
                            w.unwrap()
                        }

                        let mut values = String::new();
                        for (i, column) in columns.iter().enumerate() {
                            if i != 0 {
                                values += ","
                            }

                            let value = data_set.get(column).unwrap_or(&SqlType::Null);
                            values += &match value {
                                SqlType::BigInt(big_int) => big_int.to_string(),
                                SqlType::Int(int) => int.to_string(),
                                SqlType::Decimal(dec) => dec.to_string(),
                                SqlType::Varchar(varchar) => {
                                    let new_varchar = String::from("\"") + varchar;
                                    let new_varchar = new_varchar + &String::from("\"");
                                    new_varchar
                                }
                                SqlType::Bool(b) => String::from(if b.clone() { "1" } else { "0" }),
                                SqlType::Null => String::from("NULL"),
                            };
                        }

                        let w = write!(&file, "({})", values);
                        w.unwrap()
                    }
                    ThreadSignal::Stop => {
                        break;
                    }
                },
                Err(_) => {}
            }
        }
    });

    (handle,snd)
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
        input: String::from(config.value_of("input").unwrap()),
        output: String::from(config.value_of("output").unwrap()),
        maximum_rows: config.value_of("rows").unwrap().parse().unwrap(),
        no_ignore: if let Some(_) = config.value_of("no-ignore") {
            true
        } else {
            false
        },
    };

    let result = Reader::from_file(&Path::new(&arguments.input));
    let (nodes_handle,nodes) = new_thread::<Node>(arguments.clone());
    let (tags_handle,tags) = new_thread::<Tag>(arguments.clone());
    let (ways_handle,ways) = new_thread::<Way>(arguments.clone());
    let (way_nodes_handle,way_nodes) = new_thread::<WayNode>(arguments.clone());
    let (relations_handle,relations) = new_thread::<Relation>(arguments.clone());
    let (relation_members_handle, relation_members) = new_thread::<RelationMember>(arguments.clone());
    let (ref_tags_handle, ref_tags) = new_thread::<UsedTag>(arguments.clone());

    let mut used_tags: Vec<String> = vec![];
    let mut last_ref_id: i64 = 0;
    let mut last_ref_type: &str = "node";
    let mut buf = vec![];

    match result {
        Ok(mut reader) => {
            // Self closing tags
            reader.expand_empty_elements(true);

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) => {
                        let name = e.name();
                        let attrs: HashMap<String, String> = e
                            .attributes()
                            .map(|a| {
                                let attr = a.unwrap();
                                let value = String::from_utf8(attr.value.to_vec()).unwrap();

                                (str::from_utf8(attr.key).unwrap().to_string(), value)
                            })
                            .collect();

                        match name {
                            b"node" => {
                                let mut node: Node = Node {
                                    ..Default::default()
                                };

                                for (key, value) in attrs {
                                    if !node.main_info.set_attribute(key.clone(), value.clone()) {
                                        match key.as_str() {
                                            "lat" => {
                                                node.lat = value.parse::<f32>().unwrap();
                                            }
                                            "lon" => {
                                                node.lng = value.parse::<f32>().unwrap();
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                last_ref_id = node.main_info.id;
                                last_ref_type = "node";

                                nodes.send(ThreadSignal::Write(node)).unwrap();
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

                                ways.send(ThreadSignal::Write(way)).unwrap();
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

                                relations.send(ThreadSignal::Write(relation)).unwrap();
                            }
                            b"tag" => {
                                let k = String::from(attrs.get("k").unwrap());
                                let v = String::from(attrs.get("v").unwrap());

                                let tag_index = used_tags.iter().position(|t| t == &k);

                                let tag_id = match tag_index {
                                    None => {
                                        let id = used_tags.len() as i16;
                                        let in_tag = Tag {
                                            id,
                                            name: k.clone(),
                                        };

                                        used_tags.push(k.clone());
                                        tags.send(ThreadSignal::Write(in_tag)).unwrap();
                                        id
                                    }
                                    Some(index) => index as i16,
                                };

                                ref_tags
                                    .send(ThreadSignal::Write(UsedTag {
                                        tag_id,
                                        value: v,
                                        ref_id: last_ref_id,
                                        ref_type: String::from(last_ref_type),
                                    }))
                                    .unwrap();
                            }
                            b"nd" => {
                                let ref_attr = attrs
                                    .get("ref")
                                    .expect("Can not read the ref attribute from nd tag.");

                                way_nodes
                                    .send(ThreadSignal::Write(WayNode {
                                        way_id: last_ref_id,
                                        node_id: ref_attr.parse::<i64>().unwrap(),
                                    }))
                                    .unwrap();
                            }
                            b"member" => {
                                let ref_attr = attrs
                                    .get("ref")
                                    .expect("Can not read ref attr from member tag.");
                                let type_attr = attrs
                                    .get("type")
                                    .expect("Can not read type attr from member tag.");
                                let role_attr = attrs.get("role").unwrap();

                                relation_members
                                    .send(ThreadSignal::Write(RelationMember {
                                        ref_id: ref_attr.parse::<i64>().unwrap(),
                                        ref_type: type_attr.clone(),
                                        role: role_attr.clone(),
                                        relation_id: last_ref_id,
                                    }))
                                    .unwrap();
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

    nodes.send(ThreadSignal::Stop).unwrap();
    tags.send(ThreadSignal::Stop).unwrap();
    ways.send(ThreadSignal::Stop).unwrap();
    way_nodes.send(ThreadSignal::Stop).unwrap();
    relations.send(ThreadSignal::Stop).unwrap();
    relation_members.send(ThreadSignal::Stop).unwrap();
    ref_tags.send(ThreadSignal::Stop).unwrap();

    nodes_handle.join().unwrap();
    tags_handle.join().unwrap();
    ways_handle.join().unwrap();
    way_nodes_handle.join().unwrap();
    relations_handle.join().unwrap();
    relation_members_handle.join().unwrap();
    ref_tags_handle.join().unwrap();
}
