mod store;
mod helpers;

use crate::store::Store;

use serde::{Deserialize, Serialize};

struct Keyv<T: Store> {
    store: T,
}

struct KeyvAsync<T: Store> {
    store: T,
}

impl<T: Store> Keyv<T> {
    pub fn new(store: T) -> Self {
        Keyv { store }
    }

    pub fn get_many<'a, K, V>(&self, keys: Vec<&str>) -> Result<Vec<V>, String>
    where 
        V: for<'de> Deserialize<'de> {
        let mut result: Vec<V> = vec![];
    for key in keys {
            result.push(self.get(key).unwrap());
        };
        Ok(result)
    }

    pub fn get<'a, V>(&self, key: &str) -> Result<V, String>
    where
        V: for<'de> Deserialize<'de>
    {
        let deserialized: V = serde_json::from_str(self.store.get(key).unwrap()).unwrap();
        Ok(deserialized)
    }


    pub fn set<V>(&self, key: &str, value: V)
    where V: Serialize
    {
        self.store.set(key, &serde_json::to_string(&value).unwrap())
    }

}

impl<T: Store> KeyvAsync<T> {
    pub fn new(store: T) -> Self {
        KeyvAsync {store}
    }

    pub async fn get<V>(&self, key: &str) -> Result<V, String>
    where
        V: for<'de> Deserialize<'de> + Send + Sync,
    {
        let deserialized: V = serde_json::from_str(self.store.get_async(key).await.unwrap()).unwrap();
        Ok(deserialized)
    }

    pub async fn get_many<'a, K, V>(&self, keys: Vec<&str>) -> Result<Vec<V>, String>
    where 
        V: for<'de> Deserialize<'de> + Send + Sync {
        let mut result: Vec<V> = vec![];
    for key in keys {
            result.push(self.get(key).await.unwrap());
        };
        Ok(result)
    }

    pub async fn set<V>(&self, key:&str, value:V) -> Result<bool, String> 
    where V: Serialize + Send
    {
        self.store.set_async(key, &serde_json::to_string(&value).unwrap()).await
    }

}
