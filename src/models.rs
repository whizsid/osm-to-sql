use std::collections::HashMap;

#[derive(Default)]
pub struct MainInfo {
    pub changeset: i32,
    pub id: i64,
    pub version: i16,
    pub timestamp: String,
    pub user: String,
    pub uid: i32,
    pub visible: bool,
}

impl MainInfo {
    fn get_data_set(&self) -> HashMap<&str, SqlType> {
        let mut data_set: HashMap<&str, SqlType> = HashMap::new();
        data_set.insert("changeset", SqlType::Int(self.changeset));
        data_set.insert("id", SqlType::BigInt(self.id));
        data_set.insert("version", SqlType::Int(self.version as i32));
        data_set.insert("timestamp", SqlType::Varchar(&self.timestamp));
        data_set.insert("user", SqlType::Varchar(&self.user));
        data_set.insert("uid", SqlType::Int(self.uid));
        data_set.insert("visible", SqlType::Bool(self.visible));

        data_set
    }

    pub fn set_attribute(&mut self, name: String, value: String) -> bool {
        match name.as_str() {
            "id" => {
                self.id = value.parse::<i64>().unwrap();
                true
            }
            "changeset" => {
                self.changeset = value.parse::<i32>().unwrap();
                true
            }
            "version" => {
                self.version = value.parse::<i16>().unwrap();
                true
            }
            "timestamp" => {
                self.timestamp = String::from(value);
                true
            }
            "user" => {
                self.user = String::from(value);
                true
            }
            "uid" => {
                self.uid = value.parse::<i32>().unwrap();
                true
            }
            "visible" => {
                self.visible = if value == "true" { true } else { false };
                true
            }
            _ => false,
        }
    }
}

#[derive(Default)]
pub struct Tag {
    pub id: i16,
    pub name: String,
}

impl Model for Tag {
    fn get_data_set<'a>(&'a self) -> HashMap<&'a str, SqlType> {
        let mut hash_map: HashMap<&str, SqlType> = HashMap::new();
        hash_map.insert("id", SqlType::Int(self.id as i32));
        hash_map.insert("name", SqlType::Varchar(self.name.as_str()));

        hash_map
    }

    fn get_table_name() -> &'static str {
        "tags"
    }

    fn get_create_table_query() -> &'static str {
        "CREATE TABLE tags(\
            id INETGER,\
            name VARCHAR(256),
            CONSTRAINT tags_pk PRIMARY KEY(id)\
        )"
    }

    fn get_columns() -> Vec<&'static str> {
        vec!["id", "name"]
    }
}

#[derive(Default)]
pub struct UsedTag {
    pub tag_id: i16,
    pub value: String,
    pub ref_id: i64,
    pub ref_type: String,
}

impl Model for UsedTag {
    fn get_data_set<'a>(&'a self) -> HashMap<&str, SqlType<'a>> {
        let mut hash_map: HashMap<&str, SqlType> = HashMap::new();
        hash_map.insert("tag_id", SqlType::Int(self.tag_id as i32));
        hash_map.insert(
            match self.ref_type.as_str() {
                "relation" => "relation_id",
                "way" => "way_id",
                "node" => "node_id",
                _ => panic!("Wrong type of tag."),
            },
            SqlType::BigInt(self.ref_id),
        );
        hash_map
    }

    fn get_table_name() -> &'static str {
        "ref_tags"
    }

    fn get_create_table_query() -> &'static str {
        "CREATE TABLE ref_tags (\
            rt_id BIGINT AUTO_INCREMENT,\
            tag_id INTEGER,\
            node_id BIGINT DEFAULT NULL,\
            relation_id BIGINT DEFAULT NULL,\
            way_id BIGINT DEFAULT NULL, \
            value VARCHAR(256),
            CONSTRAINT ref_tags_pk PRIMARY KEY(rt_id),\
            CONSTRAINT  ref_tags_tags_fk FOREIGN KEY(tag_id) REFERENCES tags(id),\
            CONSTRAINT  ref_tags_nodes_fk FOREIGN KEY(node_id) REFERENCES nodes(id),\
            CONSTRAINT  ref_tags_relations_fk FOREIGN KEY(relation_id) REFERENCES relations(id),\
            CONSTRAINT  ref_tags_ways_fk FOREIGN KEY(way_id) REFERENCES ways(id)\
        )"
    }

    fn get_columns() -> Vec<&'static str> {
        vec![
            "tag_id",
            "node_id",
            "relation_id",
            "way_id",
            "value",
        ]
    }
}

#[derive(Default)]
pub struct Node {
    pub main_info: MainInfo,
    pub lat: f32,
    pub lng: f32,
}

impl Model for Node {
    fn get_data_set<'a>(&'a self) -> HashMap<&str, SqlType<'a>> {
        let mut data_set = self.main_info.get_data_set();
        data_set.insert("lat", SqlType::Decimal(self.lat));
        data_set.insert("lng", SqlType::Decimal(self.lat));
        data_set
    }

    fn get_table_name() -> &'static str {
        "nodes"
    }

    fn get_create_table_query() -> &'static str {
        "CREATE TABLE nodes (\
             id BIGINT,\
             lat DECIMAL(10,8),\
             lng DECIMAL(10,8),\
             version INTEGER,\
             changeset INTEGER,\
             user VARCHAR(256),\
             uid INTEGER,\
             visible TINYINT(2),\
             timestamp VARCHAR(256),\
             CONSTRAINT nodes_pk PRIMARY KEY(id)\
        )"
    }

    fn get_columns() -> Vec<&'static str> {
        vec![
            "id",
            "lat",
            "lng",
            "version",
            "changeset",
            "user",
            "uid",
            "visible",
            "timestamp",
        ]
    }
}

#[derive(Default)]
pub struct Relation {
    pub main_info: MainInfo,
}

impl Model for Relation {
    fn get_data_set<'a>(&'a self) -> HashMap<&str, SqlType<'a>> {
        self.main_info.get_data_set()
    }

    fn get_table_name() -> &'static str {
        "relations"
    }

    fn get_create_table_query() -> &'static str {
        "CREATE TABLE relations (\
             id BIGINT,\
             version INTEGER,\
             changeset INTEGER,\
             user VARCHAR(256),\
             uid INTEGER,\
             visible TINYINT(2),\
             timestamp VARCHAR(256),\
             CONSTRAINT relations_pk PRIMARY KEY(id)\
             )"
    }

    fn get_columns() -> Vec<&'static str> {
        vec![
            "id",
            "version",
            "changeset",
            "user",
            "uid",
            "visible",
            "timestamp",
        ]
    }
}

#[derive(Default)]
pub struct RelationMember {
    pub ref_type: String,
    pub ref_id: i64,
    pub relation_id: i64,
    pub role: String,
}

impl Model for RelationMember {
    fn get_data_set<'a>(&'a self) -> HashMap<&str, SqlType<'a>> {
        let mut hash_map: HashMap<&str, SqlType> = HashMap::new();

        hash_map.insert("relation_id", SqlType::BigInt(self.relation_id));
        hash_map.insert("role", SqlType::Varchar(self.role.as_str()));
        hash_map.insert(
            match self.ref_type.as_str() {
                "node" => "node_id",
                "way" => "way_id",
                "relation" => "sub_relation_id",
                _ => panic!("Wrong type of relation member."),
            },
            SqlType::BigInt(self.ref_id),
        );

        hash_map
    }

    fn get_table_name() -> &'static str {
        "relation_members"
    }

    fn get_create_table_query() -> &'static str {
        "CREATE TABLE relation_members (\
            rm_id BIGINT AUTO_INCREMENT,\
            relation_id BIGINT,\
            node_id BIGINT DEFAULT NULL,\
            way_id BIGINT DEFAULT NULL,\
            sub_relation_id BIGINT DEFAULT NULL,\
            role VARCHAR (256),\
            CONSTRAINT relation_members_pk PRIMARY KEY(rm_id),\
            CONSTRAINT  relation_members_nodes_fk FOREIGN KEY(node_id) REFERENCES nodes(id),\
            CONSTRAINT  relation_members_ways_fk FOREIGN KEY(way_id) REFERENCES ways(id),\
            CONSTRAINT  relation_members_relations_fk FOREIGN KEY(relation_id) REFERENCES relations(id),\
            CONSTRAINT  relation_members_sub_relations_fk FOREIGN KEY(sub_relation_id) REFERENCES relations(id)\
        )"
    }

    fn get_columns() -> Vec<&'static str> {
        vec![
            "relation_id",
            "node_id",
            "way_id",
            "sub_relation_id",
            "role",
        ]
    }
}

#[derive(Default)]
pub struct Way {
    pub main_info: MainInfo,
}

impl Model for Way {
    fn get_data_set<'a>(&'a self) -> HashMap<&str, SqlType<'a>> {
        self.main_info.get_data_set()
    }

    fn get_table_name() -> &'static str {
        "ways"
    }

    fn get_create_table_query() -> &'static str {
        "CREATE TABLE ways (\
             id BIGINT,\
             version INTEGER,\
             changeset INTEGER,\
             user VARCHAR(256),\
             uid INTEGER,\
             visible TINYINT(2),\
             timestamp VARCHAR(256),\
             CONSTRAINT ways_pk PRIMARY KEY(id)\
        )"
    }

    fn get_columns() -> Vec<&'static str> {
        vec![
            "id",
            "version",
            "changeset",
            "user",
            "uid",
            "visible",
            "timestamp",
        ]
    }
}

#[derive(Default)]
pub struct WayNode {
    pub way_id: i64,
    pub node_id: i64,
}

impl Model for WayNode {
    fn get_data_set<'a>(&'a self) -> HashMap<&str, SqlType<'a>> {
        let mut hash_map: HashMap<&str, SqlType> = HashMap::new();

        hash_map.insert("way_id", SqlType::BigInt(self.way_id));
        hash_map.insert("node_id", SqlType::BigInt(self.node_id));

        hash_map
    }

    fn get_table_name() -> &'static str {
        "way_nodes"
    }

    fn get_create_table_query() -> &'static str {
        "CREATE TABLE way_nodes (\
            way_id BIGINT,\
            node_id BIGINT,\
            CONSTRAINT way_nodes_pk PRIMARY KEY(way_id,node_id),\
            CONSTRAINT  way_nodes_nodes_fk FOREIGN KEY(node_id) REFERENCES nodes(id),\
            CONSTRAINT  way_nodes_ways_fk FOREIGN KEY(way_id) REFERENCES ways(id)\
        )"
    }

    fn get_columns() -> Vec<&'static str> {
        vec!["way_id", "node_id"]
    }
}

#[derive(Debug,Copy,Clone)]
pub enum SqlType<'a> {
    BigInt(i64),
    Int(i32),
    Decimal(f32),
    Varchar(&'a str),
    Bool(bool),
    Null,
}

pub trait Model {
    fn get_data_set<'a>(&'a self) -> HashMap<&str, SqlType<'a>>;

    fn get_table_name() -> &'static str;

    fn get_create_table_query() -> &'static str;

    fn get_columns() -> Vec<&'static str>;
}
