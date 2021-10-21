pub struct User {
    pub name: String,
    pub nickname: String,
    pub pic_url: String,
    pub top_role: Role
}

pub struct Role {
    pub name: String,
    pub color_code: u16
}
