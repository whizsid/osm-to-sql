use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::path::PathBuf;

use crate::node::Node;
use crate::main_info::Tag;
use crate::main_info::UsedTag;
use crate::main_info::MainInfo;
use crate::way::WayNode;
use crate::relation::RelationMember;

pub struct SqlFile {
    pub table_name: String,
    pub rows: i32,
    pub sets: i32,
    pub maximum_rows_per_query: i32,
    pub last_set_rows: i32,
    pub file: File,
}

fn create_file(output_dir: String, name: &str)->File {
    let file_name = String::from(name);

    let file_path = PathBuf::from(output_dir).join(file_name);

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create_new(true)
        .open(file_path)
        .unwrap();
    
    return file;
}

impl SqlFile {
    //Node Tables
    pub fn new_node_file(output_dir: String, maximum_rows_per_query: i32) -> SqlFile {
        let file = create_file(output_dir, "nodes.sql");

        if let Err(e) = write!(&file, "DROP TABLE IF EXISTS nodes;") {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        };

        if let Err(e) = write!(
            &file,
            "\nCREATE TABLE nodes (\
             id INTEGER,\
             lat DECIMAL(10,8),\
             lng DECIMAL(10,8),\
             version INTEGER,\
             changeset INTEGER,\
             user VARCHAR(150),\
             uid INTEGER,\
             visible TINYINT(2),\
             date_time VARCHAR(150),\
             CONSTRAINT nodes_pk PRIMARY KEY(id)\
             )"
        ) {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        }

        return SqlFile {
            table_name: String::from("nodes"),
            rows: 0,
            sets: 0,
            maximum_rows_per_query: maximum_rows_per_query,
            last_set_rows: 0,
            file: file,
        };
    }

    pub fn insert_to_node_file(&mut self, node: Node) {
        if self.last_set_rows >= self.maximum_rows_per_query || self.rows == 0 {
            if let Err(e) = write!(
                self.file,
                ";\nINSERT INTO nodes (\
                 id,lat,lng, version, changeset, user, uid, visible, date_time\
                 ) VALUES (\
                 \"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\
                 ) ",
                node.main_info.id,
                node.lat,
                node.lng,
                node.main_info.version,
                node.main_info.changeset,
                node.main_info.user,
                node.main_info.uid,
                if node.main_info.visible { 1 } else { 0 },
                node.main_info.timestamp
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows = 1;
        } else {
            if let Err(e) = write!(
                self.file,
                ", (\
                 \"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\
                 ) ",
                node.main_info.id,
                node.lat,
                node.lng,
                node.main_info.version,
                node.main_info.changeset,
                node.main_info.user,
                node.main_info.uid,
                if node.main_info.visible { 1 } else { 0 },
                node.main_info.timestamp
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows += 1;
        }

        self.rows += 1;
    }

    //Way/Relation Tables
    pub fn new_main_file(table_name:&str,output_dir: String, maximum_rows_per_query: i32) -> SqlFile {
        let file = create_file(output_dir, &format!("{}{}",table_name,".sql"));

        if let Err(e) = write!(&file, "DROP TABLE IF EXISTS {};",table_name) {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        };

        if let Err(e) = write!(
            &file,
            "\nCREATE TABLE {} (\
             id INTEGER,\
             version INTEGER,\
             changeset INTEGER,\
             user VARCHAR(150),\
             uid INTEGER,\
             visible TINYINT(2),\
             date_time VARCHAR(150),\
             CONSTRAINT {}_pk PRIMARY KEY(id)\
             )",
             table_name,
             table_name
        ) {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        }

        return SqlFile {
            table_name: String::from(table_name),
            rows: 0,
            sets: 0,
            maximum_rows_per_query: maximum_rows_per_query,
            last_set_rows: 0,
            file: file,
        };
    }

    pub fn insert_to_main_file(&mut self, table_name: &str,main_info: MainInfo) {
        if self.last_set_rows >= self.maximum_rows_per_query || self.rows == 0 {
            if let Err(e) = write!(
                self.file,
                ";\nINSERT INTO {} (\
                 id, version, changeset, user, uid, visible, date_time\
                 ) VALUES (\
                 \"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\
                 ) ",
                table_name,
                main_info.id,
                main_info.version,
                main_info.changeset,
                main_info.user,
                main_info.uid,
                if main_info.visible { 1 } else { 0 },
                main_info.timestamp
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows = 1;
        } else {
            if let Err(e) = write!(
                self.file,
                ", (\
                 \"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\
                 ) ",
                main_info.id,
                main_info.version,
                main_info.changeset,
                main_info.user,
                main_info.uid,
                if main_info.visible { 1 } else { 0 },
                main_info.timestamp
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows += 1;
        }

        self.rows += 1;
    }

    // Tags Table
    pub fn new_tag_file(output_dir: String, maximum_rows_per_query: i32) -> SqlFile {
        let file = create_file(output_dir,"tags.sql");

        if let Err(e) = write!(&file, "DROP TABLE IF EXISTS tags;") {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        };

        if let Err(e) = write!(
            &file,
            "\nCREATE TABLE tags (\
                id INTEGER,\
                name VARCHAR(150),\
                CONSTRAINT tags_pk PRIMARY KEY(id)\
             )"
        ) {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        }

        return SqlFile {
            table_name: String::from("tags"),
            rows: 0,
            sets: 0,
            maximum_rows_per_query: maximum_rows_per_query,
            last_set_rows: 0,
            file: file,
        };
    }

    pub fn insert_to_tag_file(&mut self,tag:&Tag){
        if self.last_set_rows >= self.maximum_rows_per_query || self.rows == 0 {
            if let Err(e) = write!(
                self.file,
                ";\nINSERT INTO tags (\
                 id,name\
                 ) VALUES (\
                 \"{}\",\"{}\"\
                 ) ",
                tag.id,
                tag.name
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows = 1;
        } else {
            if let Err(e) = write!(
                self.file,
                ", (\
                 \"{}\",\"{}\"\
                 ) ",
                tag.id,
                tag.name
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows += 1;
        }

        self.rows += 1;
    }

    // Node, Members, Relations has tags
    pub fn new_ref_tags_file(output_dir: String, maximum_rows_per_query: i32) -> SqlFile {
        let file = create_file(output_dir,"ref_tags.sql");

        if let Err(e) = write!(&file, "DROP TABLE IF EXISTS ref_tags;") {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        };

        if let Err(e) = write!(
            &file,
            "\nCREATE TABLE ref_tags (\
                rt_id INTEGER AUTO_INCREMENT,
                tag_id INTEGER,\
                node_id INTEGER DEFAULT NULL,\
                relation_id INTEGER DEFAULT NULL,\
                way_id INTEGER DEFAULT NULL,\
                value VARCHAR(250),\
                CONSTRAINT ref_tags_pk PRIMARY KEY(rt_id),\
                CONSTRAINT  ref_tags_tags_fk FOREIGN KEY(tag_id) REFERENCES tags(id),\
                CONSTRAINT  ref_tags_nodes_fk FOREIGN KEY(node_id) REFERENCES nodes(id),\
                CONSTRAINT  ref_tags_relations_fk FOREIGN KEY(relation_id) REFERENCES relations(id),\
                CONSTRAINT  ref_tags_ways_fk FOREIGN KEY(way_id) REFERENCES ways(id)\
             )"
        ) {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        }

        return SqlFile {
            table_name: String::from("tags"),
            rows: 0,
            sets: 0,
            maximum_rows_per_query: maximum_rows_per_query,
            last_set_rows: 0,
            file: file,
        };
    }

    pub fn insert_to_ref_tags_file(&mut self,used_tag:UsedTag){
        if self.last_set_rows >= self.maximum_rows_per_query || self.rows == 0 {
            if let Err(e) = write!(
                self.file,
                ";\nINSERT INTO ref_tags (\
                 tag_id,node_id,way_id,relation_id,value\
                 ) VALUES (\
                 \"{}\",{},{},{},\"{}\"\
                 ) ",
                used_tag.tag.id,
                if used_tag.ref_type=="node" { used_tag.ref_id.to_string()} else {String::from("NULL")},
                if used_tag.ref_type=="way" { used_tag.ref_id.to_string()} else {String::from("NULL")},
                if used_tag.ref_type=="relation" { used_tag.ref_id.to_string()} else {String::from("NULL")},
                used_tag.value
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows = 1;
        } else {
            if let Err(e) = write!(
                self.file,
                ", (\
                 \"{}\",{},{},{},\"{}\"\
                 ) ",
                used_tag.tag.id,
                if used_tag.ref_type=="node" { used_tag.ref_id.to_string()} else {String::from("NULL")},
                if used_tag.ref_type=="way" { used_tag.ref_id.to_string()} else {String::from("NULL")},
                if used_tag.ref_type=="relation" { used_tag.ref_id.to_string()} else {String::from("NULL")},
                used_tag.value
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows += 1;
        }

        self.rows += 1;
    }

    // Ways has multiple nodes
    pub fn new_way_nodes_file(output_dir: String, maximum_rows_per_query: i32) -> SqlFile {
        let file = create_file(output_dir,"way_nodes.sql");

        if let Err(e) = write!(&file, "DROP TABLE IF EXISTS way_nodes;") {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        };

        if let Err(e) = write!(
            &file,
            "\nCREATE TABLE way_nodes (\
                way_id INTEGER,\
                node_id INTEGER,\
                CONSTRAINT way_nodes_pk PRIMARY KEY(way_id,node_id),\
                CONSTRAINT  way_nodes_nodes_fk FOREIGN KEY(node_id) REFERENCES nodes(id),\
                CONSTRAINT  way_nodes_ways_fk FOREIGN KEY(way_id) REFERENCES ways(id)\
             )"
        ) {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        }

        return SqlFile {
            table_name: String::from("way_nodes"),
            rows: 0,
            sets: 0,
            maximum_rows_per_query: maximum_rows_per_query,
            last_set_rows: 0,
            file: file,
        };
    }

    pub fn insert_to_way_nodes_file(&mut self,way_node:WayNode){
        if self.last_set_rows >= self.maximum_rows_per_query || self.rows == 0 {
            if let Err(e) = write!(
                self.file,
                ";\nINSERT INTO way_nodes (\
                 node_id,way_id\
                 ) VALUES (\
                 \"{}\",\"{}\"\
                 ) ",
                way_node.node_id,
                way_node.way_id
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows = 1;
        } else {
            if let Err(e) = write!(
                self.file,
                ", (\
                 \"{}\",\"{}\"\
                 ) ",
                way_node.node_id,
                way_node.way_id
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows += 1;
        }

        self.rows += 1;
    }

    // Ways has multiple nodes
    pub fn new_relation_members_file(output_dir: String, maximum_rows_per_query: i32) -> SqlFile {
        let file = create_file(output_dir,"relation_members.sql");

        if let Err(e) = write!(&file, "DROP TABLE IF EXISTS relation_members;") {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        };

        if let Err(e) = write!(
            &file,
            "\nCREATE TABLE relation_members (\
                rm_id INTEGER AUTO_INCREMENT,
                relation_id INTEGER,\
                node_id INTEGER DEFAULT NULL,\
                way_id INTEGER DEFAULT NULL,\
                role VARCHAR (150),\
                CONSTRAINT relation_members_pk PRIMARY KEY(rm_id),\
                CONSTRAINT  relation_members_nodes_fk FOREIGN KEY(node_id) REFERENCES nodes(id),\
                CONSTRAINT  relation_members_ways_fk FOREIGN KEY(way_id) REFERENCES ways(id),\
                CONSTRAINT  relation_members_relations_fk FOREIGN KEY(relation_id) REFERENCES relations(id)\
             )"
        ) {
            drop(file);
            panic!("Could not to write to the file: {:?}", e);
        }

        return SqlFile {
            table_name: String::from("relation_members"),
            rows: 0,
            sets: 0,
            maximum_rows_per_query: maximum_rows_per_query,
            last_set_rows: 0,
            file: file,
        };
    }

    pub fn insert_to_relation_members_file(&mut self,relation_member:RelationMember){
        if self.last_set_rows >= self.maximum_rows_per_query || self.rows == 0 {
            if let Err(e) = write!(
                self.file,
                ";\nINSERT INTO relation_members (\
                 relation_id,node_id,way_id,role\
                 ) VALUES (\
                 \"{}\",{},{},\"{}\"\
                 ) ",
                relation_member.relation_id,
                if relation_member.ref_type=="node" {relation_member.ref_id.to_string()} else {String::from("NULL")},
                if relation_member.ref_type=="way" {relation_member.ref_id.to_string()} else {String::from("NULL")},
                relation_member.role
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows = 1;
        } else {
            if let Err(e) = write!(
                self.file,
                ", (\
                 \"{}\",{},{},\"{}\"\
                 ) ",
                relation_member.relation_id,
                if relation_member.ref_type=="node" {relation_member.ref_id.to_string()} else {String::from("NULL")},
                if relation_member.ref_type=="way" {relation_member.ref_id.to_string()} else {String::from("NULL")},
                relation_member.role
            ) {
                drop(&self.file);
                panic!("Could not to write to the file: {:?}", e);
            }

            self.last_set_rows += 1;
        }

        self.rows += 1;
    }
}
