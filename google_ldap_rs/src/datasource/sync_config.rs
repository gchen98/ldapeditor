use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LdapConfig {
    pub host: String,
    pub admin_dn: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct User{
    pub username: String,
    pub subscription: String,
    pub filter: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct GoogleConfig{
    pub users: Vec<User>,
}