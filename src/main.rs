pub mod node;
use node::Node;

pub mod main_info;
use main_info::MainInfo;

pub mod argument;
use argument::Arguments;

use std::env;
// use quick_xml::Reader;

fn main() {

    let args : Vec<_> = env::args().collect();

    let formated_arg = Arguments::parse_args(args);

    // let xml = Reader::from_file(formated_arg.input_file);

    let node1 = Node{
        lat:23232.434,
        lng:23.43434,
        main_info:MainInfo {
            changeset: 32323,
            id: 343,
            tags: vec!(),
            timestamp: String::from("kmkmkmkk"),
            uid: 3234,
            user: String::from("dffcsdcds"),
            version: 33,
            visible: true
        }
    };

    println!("{:?}",formated_arg.input_file);
    println!("{:?}", node1.lat);

}
