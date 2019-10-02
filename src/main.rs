pub mod node;
use node::Node;

pub mod way;
use way::Way;

pub mod relation;
use relation::Relation;

pub mod main_info;
use main_info::MainInfo;
use main_info::Attr;
// use main_info::Tag;

pub mod argument;
use argument::Arguments;

use quick_xml::events::Event;
use quick_xml::Reader;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    let formated_arg = Arguments::parse_args(args);

    let result = Reader::from_file(&Path::new(&formated_arg.input_file));

    // let mut count = 0;
    let mut buf = Vec::new();
    match result {
        Ok(mut reader) =>{ 
            // Self closing tags
            reader.expand_empty_elements(true);

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) => match e.name() {
                        b"node" => {
                            let mut node:Node = Node {..Default::default()};

                            e.attributes().for_each(|a| match a {
                                Ok(attr)=>{
                                    
                                    let formated_attr = Attr::from_quick_xml(attr);
                                    let value = formated_attr.value.clone();
                                    let name = formated_attr.name.clone();

                                    if !node.main_info.set_attribute(formated_attr) {
                                        match name.as_ref() {
                                            "lat"=> node.lat = value.parse::<f32>().unwrap(),
                                            "lon"=> node.lng = value.parse::<f32>().unwrap(),
                                            _=>(),
                                        }
                                    }
                                },
                                Err(e)=>panic!("{:?}",e)
                            });
                        },
                        b"way"=>{
                            let mut way:Way = Way {..Default::default()};

                            e.attributes().for_each(|a| match a {
                                Ok(attr)=>{
                                    way.main_info.set_attribute(Attr::from_quick_xml(attr));
                                },
                                Err(e)=>panic!("{:?}",e)
                            });
                        },
                        b"relation"=>{
                            let mut relation:Relation = Relation {..Default::default()};

                            e.attributes().for_each(|a| match a {
                                Ok(attr)=>{
                                    relation.main_info.set_attribute(Attr::from_quick_xml(attr));
                                },
                                Err(e)=>panic!("{:?}",e)
                            });

                        },
                        _ => (),
                    },
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    _ => (),
                }
                buf.clear();
            }
        },
        Err(e) => panic!("Invalid file :- {:?}", e),
    }

    let node1 = Node {
        lat: 23232.434,
        lng: 23.43434,
        main_info: MainInfo {
            changeset: 32323,
            id: 343,
            tags: vec![],
            timestamp: String::from("kmkmkmkk"),
            uid: 3234,
            user: String::from("dffcsdcds"),
            version: 33,
            visible: true,
        },
    };

    println!("{:?}", formated_arg.input_file);
    println!("{:?}", node1.lat);
}
