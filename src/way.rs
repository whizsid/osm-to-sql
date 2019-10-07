pub struct Way {
    pub main_info: crate::main_info::MainInfo,
}

impl Default for Way {
    fn default()->Way {
        Way {
            main_info: crate::main_info::MainInfo {.. Default::default()}
        }
    }
}

pub struct WayNode {
    pub way_id:i64,
    pub node_id:i64
}