use simple_pub_sub::subscriber::{RedisSub, Subscriber};
use tokio::signal;

#[tokio::main]
async fn main() {
    let _redis_subscriber = RedisSub::new()
        .addr("redis://localhost:6379".into())
        .channel("test".into())
        .subscribe(|msg| {
            println!("get {}", msg);
        });
        
    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }
}