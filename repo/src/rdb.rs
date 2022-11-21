use redis::{Client, Connection, Commands};

pub struct Redis {
    #[allow(dead_code)]
    params: String,
    #[allow(dead_code)]
    client: Client,
    connection: Connection
}

pub fn open(params: &str) -> Result<Redis, &'static str> {
    let client = redis::Client::open(params).expect("Get redis client error");
    let connection = client.get_connection().expect("Get redis connection error");
    Ok(Redis {
        params: String::from(params),
        client,
        connection
    })
}

impl Redis {
    pub fn set_value(&mut self, key: &str, value: &str) {
        self.connection.set::<&str, &str, String>(key, value).expect("Set value error");
    }
    
    pub fn del_key(&mut self, key: &str) {
        self.connection.del::<&str, String>(key).expect("Delete key error");
    }
    
}
