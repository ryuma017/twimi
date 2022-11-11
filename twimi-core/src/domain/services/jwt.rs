use shaku::Interface;

pub trait JwtEncoder: Interface {
    fn encode(&self, username: &str) -> Result<String, anyhow::Error>;
}
