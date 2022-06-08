use crate::CONFIG;
use anyhow::Result;
use log::info;

pub fn start() -> Result<()> {
    let client_config = CONFIG.client.as_ref().expect("配置文件错误");
    info!(
        "connect server [{}:{}]",
        client_config.server_ip, client_config.server_port
    );

    Ok(())
}
