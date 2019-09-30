pub struct MainInfo {
    pub changeset:i16,
    pub id: i16,
    pub version: i8,
    pub timestamp: String,
    pub user: String,
    pub uid: i16,
    pub visible: bool,
    pub tags: Vec<UsedTag>,
}

pub struct Tag {
    pub id: i16,
    pub name: String,
}

pub struct UsedTag {
    pub tag: Tag,
    pub value: String,
}
