use std::net::UdpSocket;
use std::thread;

use crate::{net::server, CONFIG};
use anyhow::Result;
use log::debug;
use log::info;
use log::warn;
use rdev::listen;
use rdev::Event;

pub fn start() -> Result<()> {
    //TODO 需要检测鼠标键盘是否存在，如果不存在则进行警告
    let server_config = CONFIG.server.as_ref().expect("配置文件错误");
    //let (tx, rx) = std::sync::mpsc::channel::<Event>();

    let udp = server::start(server_config.ip.as_str(), server_config.port)?;

    let handle_event = move |event: Event| {
        //match event.event_type {
        //    rdev::EventType::MouseMove { x, y } => (),
        //    _ => println!("My callback {:?}", event),
        //}
        //tx.send(event)
        //    .unwrap_or_else(|e| warn!("send event error: {:?}", e));
        udp.send(event.into())
            .unwrap_or_else(|e| warn!("send event error: {:?}", e));
    };

    if let Err(error) = listen(handle_event) {
        panic!("监听鼠标键盘失败: {:?}", error);
    }

    Ok(())
}
