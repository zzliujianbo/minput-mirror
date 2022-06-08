use crate::{net::server, CONFIG};
use anyhow::Result;

pub fn start() -> Result<()> {
    let server_config = CONFIG.server.as_ref().expect("配置文件错误");
    server::start(server_config.ip.as_str(), server_config.port)
}
