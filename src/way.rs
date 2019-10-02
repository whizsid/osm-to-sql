pub struct Way {
    pub nodes: Vec<i8>,
    pub main_info: crate::main_info::MainInfo,
}

impl Default for Way {
    fn default()->Way {
        Way {
            main_info: crate::main_info::MainInfo {.. Default::default()},
            nodes: vec![]
        }
    }
}