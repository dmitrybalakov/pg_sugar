use postgres::{Client, NoTls};

pub fn connect() -> Client {
  Client::connect(CONNECTION_STRING, NoTls).unwrap()
}

pub const CONNECTION_STRING: &str = "postgresql://postgres:123456@localhost:25432/main";