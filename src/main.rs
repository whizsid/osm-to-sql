pub mod node;
use node::Node;

pub mod way;
use way::Way;
use way::WayNode;

pub mod relation;
use relation::Relation;
use relation::RelationMember;

pub mod main_info;
use main_info::Attr;
use main_info::Tag;
use main_info::UsedTag;

pub mod argument;
use argument::Arguments;

use quick_xml::events::Event;
use quick_xml::Reader;

use std::env;
use std::path::Path;
use std::str;

pub mod sql_file;
use sql_file::SqlFile;

fn main() {

    let args: Vec<_> = env::args().collect();

    let formated_arg = Arguments::parse_args(args);

    let result = Reader::from_file(&Path::new(&formated_arg.input_file));

    let mut nodes_table = SqlFile {..SqlFile::new_node_file(formated_arg.clone())};
    let mut tags_table = SqlFile {..SqlFile::new_tag_file(formated_arg.clone())};
    let mut ways_table = SqlFile {..SqlFile::new_main_file(formated_arg.clone(),"ways")};
    let mut way_nodes_table = SqlFile {..SqlFile::new_way_nodes_file(formated_arg.clone())};
    let mut relations_table = SqlFile {..SqlFile::new_main_file(formated_arg.clone(),"relations")};
    let mut relation_members_table = SqlFile {..SqlFile::new_relation_members_file(formated_arg.clone())};
    let mut ref_tags_table = SqlFile {..SqlFile::new_ref_tags_file(formated_arg.clone())};

    // let mut count = 0;
    let mut buf = Vec::new();
    let mut used_tags: Vec<String> = vec![];
    let mut last_ref_id: i64 =0;
    let mut last_ref_type: &str ="node";

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

                            last_ref_id = node.main_info.id;
                            last_ref_type = "node";

                            nodes_table.insert_to_node_file(node);
                        },
                        b"way"=>{
                            let mut way:Way = Way {..Default::default()};

                            e.attributes().for_each(|a| match a {
                                Ok(attr)=>{
                                    way.main_info.set_attribute(Attr::from_quick_xml(attr));
                                },
                                Err(e)=>panic!("{:?}",e)
                            });

                            last_ref_id = way.main_info.id;
                            last_ref_type = "way";

                            ways_table.insert_to_main_file("ways", way.main_info);
                        },
                        b"relation"=>{
                            let mut relation:Relation = Relation {..Default::default()};

                            e.attributes().for_each(|a| match a {
                                Ok(attr)=>{
                                    relation.main_info.set_attribute(Attr::from_quick_xml(attr));
                                },
                                Err(e)=>panic!("{:?}",e)
                            });

                            last_ref_id = relation.main_info.id;
                            last_ref_type = "relation";

                            relations_table.insert_to_main_file("relations", relation.main_info);
                        },
                        b"tag"=>{
                            let tag:Tag;

                            let key_attribute_result = e.attributes().find(|a| match a {
                                Ok(attr)=>{
                                    String::from_utf8_lossy(attr.key)  =="k"
                                },
                                Err(e)=>panic!("{:?}",e)
                            });

                            let value_attribute_result = e.attributes().find(|a| match a {
                                Ok(attr)=>{
                                    String::from_utf8_lossy(attr.key)  =="v"
                                },
                                Err(e)=>panic!("{:?}",e)
                            });

                            match key_attribute_result {
                                Some(result) => {
                                    match result {
                                        Ok(key_attribute)=>{
                                            
                                            match value_attribute_result {
                                                Some(v_result)=>{
                                                    match v_result {
                                                        Ok(value_attribute)=>{
                                                            let k_attr = Attr::from_quick_xml(key_attribute);
                                                            let v_attr = Attr::from_quick_xml(value_attribute);
                                                            

                                                            tag = match used_tags.iter().position(|t| &t==&&k_attr.value) {
                                                                None =>{

                                                                    let inner_tag = Tag {
                                                                        id: used_tags.len() as i16,
                                                                        name: k_attr.value.clone()
                                                                    };

                                                                    used_tags.push(k_attr.value.clone());

                                                                    tags_table.insert_to_tag_file(&inner_tag);

                                                                    inner_tag
                                                                },
                                                                Some(num)=>{
                                                                    let inner_tag = Tag {
                                                                        id: num as i16,
                                                                        name: k_attr.value.clone()
                                                                    };

                                                                    inner_tag
                                                                }
                                                            };

                                                            ref_tags_table.insert_to_ref_tags_file(UsedTag {
                                                                ref_id: last_ref_id,
                                                                tag: tag,
                                                                value: v_attr.value,
                                                                ref_type: last_ref_type
                                                            });

                                                        },
                                                        Err(e)=>panic!("Can not read tag value attribute:{:?}",e)
                                                    }
                                                },
                                                None=>()
                                            }

                                        },
                                        Err(e)=>panic!("Can not read tag key attribute: {:?}",e)
                                    };
                                },
                                None => (),
                            };
                        },
                        b"nd"=>{
                            let ref_attribute_option = e.attributes().find(|a| match a {
                                Ok(attr)=>{
                                    String::from_utf8_lossy(attr.key)=="ref"
                                },
                                Err(e)=>panic!("Can not read ref attribute of nd tag:{:?}",e)
                            });

                            match ref_attribute_option {
                                Some(ref_attribute_result)=>{
                                    match ref_attribute_result {
                                        Ok(ref_attribute)=>{
                                            let attr = Attr::from_quick_xml(ref_attribute);

                                            way_nodes_table.insert_to_way_nodes_file(WayNode {
                                                way_id:last_ref_id,
                                                node_id: attr.value.parse::<i64>().unwrap()
                                            })
                                        },
                                        Err(e)=>panic!("Error reading ref attribute in nd tag: {:?}",e)
                                    }
                                },
                                None=>panic!("Error reading ref attribute in nd tag!"),
                            };



                        },
                        b"member"=>{
                            let ref_attribute_option = e.attributes().find(|a| match a {
                                Ok(attr)=>{
                                    String::from_utf8_lossy(attr.key)=="ref"
                                },
                                Err(e)=>panic!("Can not read ref attribute of nd tag:{:?}",e)
                            });

                            let type_attribute_option = e.attributes().find(|a| match a {
                                Ok(attr)=>{
                                    String::from_utf8_lossy(attr.key)=="type"
                                },
                                Err(e)=>panic!("Can not read ref attribute of nd tag:{:?}",e)
                            });

                            let role_attribute_option = e.attributes().find(|a| match a {
                                Ok(attr)=>{
                                    String::from_utf8_lossy(attr.key)=="role"
                                },
                                Err(e)=>panic!("Can not read ref attribute of nd tag:{:?}",e)
                            });

                            match ref_attribute_option {
                                Some(ref_attribute_result)=>match ref_attribute_result {
                                    Ok(ref_attribute)=>match type_attribute_option {
                                        Some(type_attribute_result)=>match type_attribute_result {
                                            Ok(type_attribute)=>{
                                                let role = match role_attribute_option {
                                                    Some(role_attribute_result)=>match role_attribute_result {
                                                        Ok(role_attribute)=>{
                                                            Attr::from_quick_xml(role_attribute).value
                                                        },
                                                        Err(e)=>panic!("Can not read role attribute in member tag:- {:?}",e)
                                                    },
                                                    None=>String::from("")
                                                };

                                                let ref_attr = Attr::from_quick_xml(type_attribute);

                                                relation_members_table.insert_to_relation_members_file(RelationMember {
                                                    role:role,
                                                    ref_type: ref_attr.value,
                                                    ref_id: Attr::from_quick_xml(ref_attribute).value.parse::<i64>().unwrap(),
                                                    relation_id: last_ref_id
                                                })
                                            },
                                            Err(e)=>panic!("Can not read type attribute in member tag.,{:?}",e)
                                        },
                                        None=>panic!("Can not read type attribute in member tag.")
                                    },
                                    Err(e)=>panic!("Can not read ref attribute in member tag.,{:?}",e)
                                },
                                None=>panic!("Can not read ref attribute in member tag.")
                            }
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

}
