use super::{CreateSession, Echo, OpenSession};

pub enum Command {
    Echo(Echo),
    CreateSession(super::CreateSession),
    OpenSession(super::OpenSession),
}

fn decode(buf: B) -> Result<Command>
where
    B: bytes::Buf,
{
}
