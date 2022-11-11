use shaku::Interface;

pub trait JwtEncoder: Interface {
    // const SELECT_KEY: &'static str;

    fn encode(&self, username: &str) -> Result<String, anyhow::Error>;
}
