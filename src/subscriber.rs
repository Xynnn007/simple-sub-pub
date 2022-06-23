//! subscriber mod for simple pub-sub

use redis::PubSubCommands;

use crate::Result;

/// The standard trait for a `Subscriber`
pub trait Subscriber {
    /// publish a message to all the subscribers.
    /// 
    /// The parameter f is the closure to handle the received 
    /// message.
    fn subscribe<F>(&mut self, f: F) -> Result<()> 
        where F: FnMut(String) + Send + 'static;
}

/// An subscriber using redis.
pub struct RedisSub{
    channel: String,
    addr: String,
}

impl RedisSub {
    /// Instantiate a new RedisPub.
    /// 
    /// # Example
    /// ```no_run
    /// let pub = RedisSub::new("redis://localhost")?;
    /// ```
    pub fn new() -> Self {
        Self {
            channel: String::new(),
            addr: String::new(),
        }
    }

    /// Set the redis channel for the RedisSub
    /// # Example
    /// 
    /// ```no_run
    /// let pub = RedisSub::new()?
    ///     .channel("test".into());
    /// ```
    pub fn channel(mut self, channel: String) -> Self {
        self.channel = channel;
        self
    }

    /// Set the redis addr for the RedisSub
    /// # Example
    /// 
    /// ```no_run
    /// let pub = RedisSub::new()?
    ///     .addr("test".into());
    /// ```
    pub fn addr(mut self, addr: String) -> Self {
        self.addr = addr;
        self
    }
}

/// `subscribe` a addr and channel as the input.
/// # Example
/// ```no_run
/// let sub = RedisSub::new()
///     .addr("redis://localhost".into())
///     .channel("testChannel".into())
///     .subscribe(|msg| {
///         println!("get {}", msg);
///     });
/// ```
impl Subscriber for RedisSub {
    fn subscribe<F>(&mut self, mut f: F) -> Result<()> 
    where F: FnMut(String) + Send + 'static
    {
        let addr = self.addr.clone();
        let channel = self.channel.clone();

        let _ = tokio::spawn(async move {
            let client = redis::Client::open(addr).unwrap();
            let mut conn = client.get_connection().unwrap();

            let _: () = conn.subscribe(&[channel], |msg| {
                let recv : String = msg.get_payload().unwrap();
                f(recv);

                return redis::ControlFlow::Continue;
            }).unwrap();
        });

        Ok(())
    }
}