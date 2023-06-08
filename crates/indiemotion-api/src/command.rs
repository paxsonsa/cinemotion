use std::marker::PhantomData;

pub enum Command {
    Ping,
}

pub struct JSONProtocol {}

pub struct Encoder<P> {
    _p: PhantomData<P>,
}

impl Encoder<JSONProtocol> {
    pub fn encode(command: Command) -> Result<String, crate::Error> {
        Ok("".to_string())
    }

    pub fn decode(message: &str) -> Result<String, crate::Error> {
        Ok(message.to_string())
    }
}
