pub struct ServerAddress {
    pub host: String,
    pub port: u16,
}

impl From<[&str; 2]> for ServerAddress {
    fn from(info: [&str; 2]) -> Self {
        Self {
            host: info[0].to_string(),
            port: info[1].parse::<u16>().unwrap(),
        }
    }
}