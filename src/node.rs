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
            lat: 0.0,
            lng: 0.0
        }
    }
}