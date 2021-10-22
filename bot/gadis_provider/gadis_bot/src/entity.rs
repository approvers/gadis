pub struct User {
    pub name: String,
    pub nickname: String,
    pub pic_url: String,
    pub roles: Vec<Role>
}

pub struct Role {
    pub name: String,
    pub color_code: u16
}
