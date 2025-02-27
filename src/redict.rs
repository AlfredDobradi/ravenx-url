use redis::{Commands, Connection as RedictConnection, ErrorKind, FromRedisValue, RedisError, RedisResult, Value};
use redis::ErrorKind::TypeError;
use redis::Value::Map;
use tracing::info;
use crate::api;

pub type UrlList = Vec<String>;

#[derive(Debug, Default)]
pub struct UrlItem {
    pub url: String,
    pub hits: u64,
}

impl From<(&Value, &Value)> for UrlItem {
    fn from((raw_url, raw_hits): (&Value, &Value)) -> Self {
        let url: String;
        let hits: u64;

        match raw_url {
            Value::BulkString(u) => url = String::from_utf8(u.clone()).unwrap_or_default(),
            _ => url = "".to_string(),
        }

        match raw_hits {
            Value::BulkString(h) => hits = String::from_utf8(h.clone()).unwrap_or_default().parse::<u64>().unwrap_or_default(),
            _ => hits = 0,
        }

        Self { url, hits }
    }
}

impl FromRedisValue for UrlItem {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Array(array) => {
                let mut url_raw = &Value::BulkString(Vec::new());
                let mut hits_raw = &Value::BulkString(Vec::new());

                let mut index = 0;
                for v in array {
                    if let Value::BulkString(val) = v {
                        let value = String::from_utf8(val.to_vec())?;
                        match value.as_str() {
                            "url" => url_raw = array.get(index+1).unwrap_or(&Value::Nil),
                            "hits" => hits_raw = array.get(index+1).unwrap_or(&Value::Nil),
                            _ => {}
                        }
                    };

                    index += 1;
                }

                Ok(Self::from((url_raw, hits_raw)))
            }
            _ => Err(RedisError::from((TypeError, "Failed to parse redict value")))
        }
    }
}


pub struct Connection {
    c: RedictConnection,
    version: String,
}

impl From<(RedictConnection, String)> for Connection {
    fn from((con, version): (RedictConnection, String)) -> Self {
        Self { c: con, version }
    }
}

impl Connection {
    pub fn get_list(&mut self) -> Result<Vec<String>, api::error::ApiError> {
        let list_key = format!("url:{}:list", self.version);
        let list: Vec<String> = self.c.smembers(list_key)?;

        Ok(list)
    }

    pub fn get_item(&mut self, key: &String) -> Result<UrlItem, api::error::ApiError> {
        let item_key = format!("url:{}:{}", self.version, key);

        let item: UrlItem = self.c.hgetall(item_key)?;

        Ok(item)
    }

    pub fn add_item(&mut self, key: String, value: String) -> Result<UrlItem, api::error::ApiError> {
        let list_key = format!("url:{}:list", self.version);
        let item = UrlItem{url: value, hits: 0};

        let item_key = format!("url:{}:{}", self.version, key);

        self.c.sadd(list_key, item_key)?;


        Ok(item)
    }
}


