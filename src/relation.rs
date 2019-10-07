
pub struct Relation {
    pub main_info: crate::main_info::MainInfo
}

impl Default for Relation {
    fn default()->Relation {
        Relation {
            main_info: crate::main_info::MainInfo { .. Default::default()}
        }
    }
}

pub struct RelationMember {
    pub ref_type: String,
    pub ref_id: i64,
    pub relation_id: i64,
    pub role: String
}
