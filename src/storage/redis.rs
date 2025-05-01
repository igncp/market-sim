use redis::{Client, Connection};

use crate::storage_interface::StorageRedis;

pub struct StorageRedisImpl {
    connection: Connection,
    url: String,
}

impl StorageRedisImpl {
    pub fn new(url: &str) -> Self {
        let client = Client::open(url).expect("Invalid Redis URL");
        let connection = client.get_connection().expect("Failed to connect to Redis");

        StorageRedisImpl {
            connection,
            url: url.to_string(),
        }
    }
}

impl StorageRedis for StorageRedisImpl {
    fn append_sorted_set(&mut self, key: &str, score: u64, value: &str) -> Result<(), String> {
        redis::cmd("ZADD")
            .arg(key)
            .arg(score)
            .arg(value)
            .exec(&mut self.connection)
            .map_err(|e| format!("Failed to append to sorted set: {}", e))?;

        Ok(())
    }

    fn save_key(&mut self, key: &str, value: &str) -> Result<(), String> {
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .exec(&mut self.connection)
            .map_err(|e| format!("Failed to save key: {}", e))?;

        Ok(())
    }

    fn load_key(&mut self, key: &str) -> Result<String, String> {
        let value: String = redis::cmd("GET")
            .arg(key)
            .query(&mut self.connection)
            .map_err(|e| format!("Failed to load key: {}", e))?;

        Ok(value)
    }

    fn flush_data(&mut self) -> Result<(), String> {
        redis::cmd("FLUSHALL")
            .exec(&mut self.connection)
            .map_err(|e| format!("Failed to flush data: {}", e))?;

        Ok(())
    }
}
