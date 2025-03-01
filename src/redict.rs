use std::collections::HashMap;
use crate::api::error::ApiError;
use axum::http::StatusCode;
use redis::{
    Commands, Connection as RedictConnection, ErrorKind, FromRedisValue, RedisError, RedisResult,
    Value,
};
use serde::Serialize;
use tracing::{error, info};

pub type UrlList = Vec<String>;

#[derive(Debug, Default, Serialize)]
pub struct UrlItem {
    pub key: String,
    pub url: String,
    pub hits: u64,
}

impl From<(&Value, &Value, &Value)> for UrlItem {
    fn from((raw_key, raw_url, raw_hits): (&Value, &Value, &Value)) -> Self {
        let url: String = match raw_url {
            Value::BulkString(u) => String::from_utf8(u.clone()).unwrap_or_default(),
            _ => "".to_string(),
        };

        let key: String = match raw_key {
            Value::BulkString(u) => String::from_utf8(u.clone()).unwrap_or_default(),
            _ => "".to_string(),
        };

        let hits = match raw_hits {
            Value::BulkString(h) => String::from_utf8(h.clone())
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or_default(),
            _ => 0,
        };

        Self { key, url, hits }
    }
}

impl FromRedisValue for UrlItem {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match v {
            Value::Array(array) => {
                let mut key_raw = &Value::BulkString(Vec::new());
                let mut url_raw = &Value::BulkString(Vec::new());
                let mut hits_raw = &Value::BulkString(Vec::new());

                for (index, v) in array.iter().enumerate() {
                    if let Value::BulkString(val) = v {
                        let value = String::from_utf8(val.to_vec())?;
                        match value.as_str() {
                            "key" => key_raw = array.get(index + 1).unwrap_or(&Value::Nil),
                            "url" => url_raw = array.get(index + 1).unwrap_or(&Value::Nil),
                            "hits" => hits_raw = array.get(index + 1).unwrap_or(&Value::Nil),
                            _ => {}
                        }
                    };
                }

                Ok(Self::from((key_raw, url_raw, hits_raw)))
            }
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Failed to parse redict value",
            ))),
        }
    }
}

pub struct Connection {
    c: RedictConnection,
    version: String,
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection{{key_version: {}}}", self.version)
    }
}

impl From<(RedictConnection, String)> for Connection {
    fn from((con, version): (RedictConnection, String)) -> Self {
        Self { c: con, version }
    }
}

impl Connection {
    #[tracing::instrument]
    pub fn get_list(&mut self) -> Result<Vec<String>, ApiError> {
        let list_key = format!("url:{}:list", self.version);
        let list: Vec<String> = self.c.smembers(list_key)?;

        Ok(list)
    }

    #[tracing::instrument]
    pub fn get_item(&mut self, key: &String) -> Result<UrlItem, ApiError> {
        let item_key = format!("url:{}:{}", self.version, key);

        let item: UrlItem = self.c.hgetall(item_key)?;

        Ok(item)
    }

    #[tracing::instrument]
    pub fn get_items(&mut self) -> Result<HashMap<String, UrlItem>, ApiError> {
        let list_key = format!("url:{}:list", self.version);
        let list: Vec<String> = self.c.smembers(&list_key)?;

        let mut items: HashMap<String, UrlItem> = HashMap::new();

        for key in list {
            let item: UrlItem = self.c.hgetall(&key)?;

            items.insert(key, item);
        }

        Ok(items)
    }

    #[tracing::instrument]
    pub fn add_item(&mut self, key: &String, value: String, force: bool) -> Result<(), ApiError> {
        let list_key = format!("url:{}:list", self.version);
        let item_key = format!("url:{}:{}", self.version, key);

        self.c
            .sadd::<String, String, ()>(list_key.clone(), item_key.clone())?;

        if !force
            && self
                .c
                .hget::<String, String, UrlItem>(item_key.clone(), "url".to_string())
                .is_ok()
        {
            error!("key {} already exists", item_key);
            return Err(ApiError::StatusCode(StatusCode::CONFLICT));
        }

        self.c.hset_multiple::<&String, String, &String, ()>(
            &item_key,
            &[
                ("key".to_string(), &key),
                ("url".to_string(), &value),
                ("hits".to_string(), &"0".to_string()),
            ],
        )?;

        info!("added key {} with value {}", item_key, value);
        Ok(())
    }
}
