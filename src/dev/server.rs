use crate::{net::server, CONFIG};
use anyhow::Result;
use log::info;
use log::warn;
use rdev::listen;
use rdev::Event;

pub fn start() -> Result<()> {
    //TODO 需要检测鼠标键盘是否存在，如果不存在则进行警告
    let server_config = CONFIG.server.as_ref().expect("配置文件错误");
    let (tx, rx) = std::sync::mpsc::channel::<Event>();

    server::start(server_config.ip.as_str(), server_config.port, rx)?;

    let handle_event = move |event: Event| match event.event_type {
        rdev::EventType::KeyPress(_) => {
            tx.send(event)
                .unwrap_or_else(|e| warn!("send event error: {:?}", e));
        }
        _ => {}
    };
    info!("start server success");
    if let Err(error) = listen(handle_event) {
        panic!("监听鼠标键盘失败: {:?}", error);
    }

    Ok(())
}
