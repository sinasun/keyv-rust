use async_trait::async_trait;

#[async_trait]
pub trait Store {

    fn get(&self, key: &str) -> Result<&str, String>;
    fn set(&self, key: &str, value: &str);

    fn clear();


    // Async Functions
    async fn get_async(&self, key: &str) -> Result<&str, String>;
    async fn set_async(&self, key: &str, value: &str) -> Result<bool, String>;

    async fn clear_async();
}
