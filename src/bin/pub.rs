use simple_pub_sub::publisher::{RedisPub, Publisher};
use simple_pub_sub::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut publisher = RedisPub::new("redis://localhost:6379")?
        .channel("test".into());
    
    publisher.publish("321".into())?;

    Ok(())
}