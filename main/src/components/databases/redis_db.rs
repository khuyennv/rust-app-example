use crate::errors::ApiError;
use core::result::Result as CoreResult;
use r2d2_redis::redis::{FromRedisValue, RedisError, Value};
use r2d2_redis::{
    r2d2::Pool,
    redis,
    redis::{parse_redis_url, Commands},
    RedisConnectionManager,
};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::str::from_utf8;

#[derive(Clone, Debug)]
pub struct RedisDB {
    pub conn: Pool<RedisConnectionManager>,
}

#[allow(unused)]
impl RedisDB {
    /**
     * Connect redis server
     **/
    pub fn connect(addr: String) -> Self {
        let host = parse_redis_url(addr.as_str()).expect("Could not parse redis url");
        let manage = RedisConnectionManager::new(host).expect("Could not connect redis with host");
        let pool_manager = Pool::builder().build(manage).expect("Could not pool connection redis");

        info!("Redis Connected");

        Self { conn: pool_manager }
    }

    /**
     * Get the value of key
     **/
    pub fn get<T: FromRedisValue>(&self, key: String) -> Result<T, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        match conn.get::<String, T>(key) {
            Ok(val) => Ok(val),
            Err(e) => return Err(ApiError::new(400, e.to_string(), 900, None, None)),
        }
    }

    /**
     * Get multi value of keys
     **/
    pub fn mget(&self, keys: Vec<String>) -> Result<Vec<String>, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        match conn.get::<Vec<String>, Value>(keys) {
            Ok(all_value) => {
                let mut list = Vec::<String>::new();
                match all_value {
                    Value::Bulk(vals) => {
                        for val in vals.iter() {
                            let v = match val {
                                Value::Nil => "nil",
                                Value::Data(ref val) => match from_utf8(val) {
                                    Ok(x) => x,
                                    Err(_) => "",
                                },
                                _ => "",
                            };

                            list.push(v.to_string());
                        }
                    }
                    Value::Data(v) => {
                        list.push(from_utf8(v.as_ref()).unwrap().to_string());
                    }
                    Value::Nil => list.push("nil".to_string()),
                    _ => {
                        return Ok(Vec::<String>::new());
                    }
                };

                Ok(list)
            }
            Err(err) => Err(ApiError::new(400, err.to_string(), 900, None, None)),
        }
    }

    /**
     * Set key to hold the string value in expire_time seconds
     **/
    pub fn set(&self, key: String, value: String, expire_time: usize) -> bool {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return false,
        };

        let set_value = conn.set(&key, value);
        if expire_time > 0 {
            conn.expire(&key, expire_time).unwrap_or(0);
        }

        self::RedisDB::redis_result_bool(set_value)
    }

    /**
     * Delete a key
     **/
    pub fn del(&self, key: String) -> bool {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return false,
        };

        self::RedisDB::redis_result_bool(conn.del(key))
    }

    /**
     * Get hash key
     **/
    pub fn hget<T: FromRedisValue>(&self, key: String, field: String) -> Result<T, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        match conn.hget::<String, String, T>(key, field) {
            Ok(value) => Ok(value),
            Err(err) => Err(ApiError::new(400, err.to_string(), 900, None, None)),
        }
    }

    /**
     * Set hash key to hold the string value in expire_time seconds
     **/
    pub fn hset(&self, key: String, field: String, value: String) -> bool {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return false,
        };

        self::RedisDB::redis_result_bool(conn.hset(&key, field, value))
    }

    /**
     * Delete a key
     **/
    pub fn hdel(&self, key: String, field: String) -> bool {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return false,
        };

        self::RedisDB::redis_result_bool(conn.hdel(key, field))
    }

    /**
     * Add elements to Set
     **/
    pub fn sadd(&self, key: String, value: String) -> bool {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return false,
        };

        self::RedisDB::redis_result_bool(conn.sadd(&key, value))
    }

    /*
     * Get hash all
     **/
    pub fn hgetall(&self, key_name: &String) -> Result<HashMap<String, String>, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        let map: HashMap<String, String> = conn.hgetall(key_name).unwrap();

        Ok(map)
    }

    /**
     * Get number item in list
     */
    pub fn llen(&self, key: &String) -> Result<usize, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        match conn.llen(key) {
            Ok(value) => {
                return Ok(value);
            }
            Err(err) => Err(ApiError::new(500, err.to_string(), 900, Some(err.to_string()), None)),
        }
    }

    /**
     * Push a item to end of list
     */
    pub fn rpush(&self, key: &String, items: &String) -> Result<usize, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        match conn.rpush(key, items) {
            Ok(value) => {
                return Ok(value);
            }
            Err(err) => Err(ApiError::new(500, err.to_string(), 900, Some(err.to_string()), None)),
        }
    }

    /**
     * Get number item in list and set expired for list
     */
    pub fn rpush_and_set_expire(&self, key: &String, item: &String, expire_rime: usize) -> Result<usize, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        let (k1, k2): (usize, usize) = redis::pipe()
            .cmd("RPUSH")
            .arg(key)
            .arg(item)
            .cmd("EXPIRE")
            .arg(key)
            .arg(expire_rime)
            .query(conn.deref_mut())
            .unwrap();

        Ok(k1)
    }

    /**
     * Redis result
     **/
    pub fn redis_result(redis_result: CoreResult<Value, RedisError>) -> Result<Value, ApiError> {
        // println!("redis_result: {:?}", redis_result);
        match redis_result {
            Ok(value) => Ok(value),
            Err(err) => Err(ApiError::new(400, err.to_string(), 900, None, None)),
        }
    }

    /**
     * Redis result boolean
     **/
    pub fn redis_result_bool(redis_result: CoreResult<Value, RedisError>) -> bool {
        match redis_result {
            Ok(value) => match value {
                Value::Nil => false,
                _ => true,
            },
            Err(err) => false,
        }
    }

    /**
     * Set new value if key not exist
     **/
    pub fn set_nx(&self, key: String, value: String, expire_time: usize) -> bool {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return false,
        };

        let set_value = conn.set_nx::<&String, String, bool>(&key, value);

        if set_value.is_ok() && set_value.unwrap() == true && expire_time > 0 {
            conn.expire(&key, expire_time).unwrap_or(0);

            return true;
        }

        return false;
    }

    /**
     * Set a key's time to live in seconds.
     **/
    pub fn expire(&self, key: String, expire_time: usize) -> Result<usize, ApiError> {
        let mut conn = match self.conn.get() {
            Ok(conn) => conn,
            Err(err) => return Err(ApiError::new(400, err.to_string(), 900, None, None)),
        };

        match conn.expire(key, expire_time) {
            Ok(value) => {
                return Ok(value);
            }
            Err(err) => Err(ApiError::new(400, err.to_string(), 900, None, None)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn connect() {}
}
