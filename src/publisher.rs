//! publisher mod for simple pub-sub

use redis::Commands;

use crate::Result;

/// The standard trait for a `Publisher`
pub trait Publisher {
    /// publish a message to all the subscribers.
    fn publish(&mut self, message: String) -> Result<()>;

    /// get the listening address of the publisher.
    fn get_addr(&self) -> Result<String>;
}

/// An publisher using redis.
pub struct RedisPub {
    conn: redis::Connection,
    channel: String,
    addr: String,
}

impl RedisPub {
    /// Instantiate a new RedisPub.
    /// 
    /// # Example
    /// ```no_run
    /// let pub = RedisPub::new("redis://localhost")?;
    /// ```
    pub fn new(addr: &str) -> Result<Self> {
        let conn = redis::Client::open(addr)?
            .get_connection()?;
        
        Ok(Self {
            conn,
            channel: String::new(),
            addr: addr.into(),
        })
    }

    /// Set the redis channel for the RedisPub
    /// # Example
    /// 
    /// ```no_run
    /// let pub = RedisPub::new("redis://localhost")?
    ///     .channel("test".into());
    /// ```
    pub fn channel(mut self, channel: String) -> Self {
        self.channel = channel;
        self
    }
}

impl Publisher for RedisPub {
    fn publish(&mut self, message: String) -> Result<()> {
        self.conn.publish(self.channel.clone(), message)?;
        Ok(())
    }

    /// get the listening address of the publisher.
    fn get_addr(&self) -> Result<String> {
        Ok(self.addr.to_owned())
    }
}