pub mod node;
use node::Node;

pub mod main_info;
use main_info::MainInfo;

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
    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    match result {
        Ok(mut reader) => loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) => match e.name() {
                    b"node" => println!(
                        "attributes values: {:?}",
                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                    ),
                    _ => (),
                },
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
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
