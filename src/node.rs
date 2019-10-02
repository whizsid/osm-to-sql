pub struct Node {
    pub main_info: crate::main_info::MainInfo,
    pub lat: f32,
    pub lng: f32
}

impl Default for Node {
    fn default()->Node {
        Node {
            main_info: crate::main_info::MainInfo {
                ..Default::default()
            },
            lat: 3232.3232323,
            lng: 232.232323232
        }
    }
}