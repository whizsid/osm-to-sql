
pub struct Member {
    pub member_type: String,
    pub member_ref: i8,
    pub role: String,
}

pub struct Relation {
    pub members: Vec<Member>,
    pub main_info: crate::main_info::MainInfo
}

impl Default for Relation {
    fn default()->Relation {
        Relation {
            members: vec![],
            main_info: crate::main_info::MainInfo { .. Default::default()}
        }
    }
}
