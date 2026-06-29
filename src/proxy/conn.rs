use worker::*;

pub struct ProxyStream;

impl ProxyStream {
    pub fn new() -> Self {
        Self
    }

    pub async fn process(&self) -> Result<()> {
        Ok(())
    }
}
