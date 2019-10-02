use quick_xml::events::attributes::Attribute;
use std::str;

pub struct MainInfo {
    pub changeset:i32,
    pub id: i32,
    pub version: i8,
    pub timestamp: String,
    pub user: String,
    pub uid: i32,
    pub visible: bool,
    pub tags: Vec<UsedTag>,
}

impl Default for MainInfo {
    fn default()->MainInfo{
        MainInfo {
            id: 00000,
            version: 8,
            changeset: 000000,
            timestamp: String::from("2011-01-12T14:23:49Z"),
            user: String::from("anonymous"),
            uid: 0000,
            visible: true,
            tags: vec!()
        }
    }
}

impl MainInfo {
    pub fn set_attribute(&mut self, attr:crate::main_info::Attr)->bool{
        match attr.name.as_ref() {
            "id"=>{
                self.id = attr.value.parse::<i32>().unwrap();
                return true;
            } ,
            "version" => {
                self.version = attr.value.parse::<i8>().unwrap();
                return true;
            },
            "changeset"=>{
                self.changeset = attr.value.parse::<i32>().unwrap();
                return true;
            },
            "timestamp"=>{
                self.timestamp = attr.value;
                return true;
            },
            "user"=>{
                self.user = attr.value;
                return true;
            },
            "uid"=>{
                self.uid = attr.value.parse::<i32>().unwrap();
                return true;
            },
            "visible"=>{
                self.visible = attr.value=="true";
                return true;
            },
            _=>false
        }
    }
}

pub struct Attr {
    pub name: String,
    pub value: String
}

impl Attr {
    pub fn from_quick_xml (attr:Attribute)->Attr{
        let value = match str::from_utf8(& attr.value){
            Ok(ret_str)=>ret_str,
            Err(e)=>panic!("Attribute value error: {:?}",e)
        };

        let name = match str::from_utf8(attr.key){
            Ok(ret_str)=>ret_str,
            Err(e)=>panic!("Attribute name error: {:?}",e)
        };

        return Attr {
            value: value.to_owned(),
            name: name.to_owned()
        }
    }
}

pub struct Tag {
    pub id: i16,
    pub name: String,
}

pub struct UsedTag {
    pub tag: Tag,
    pub value: String,
}
