use std::net::SocketAddr;
use std::sync::RwLock;

use crate::{net::server, CONFIG};
use anyhow::Result;
use lazy_static::lazy_static;
use log::info;
use log::warn;
use rdev::listen;
use rdev::Event;

pub fn start() -> Result<()> {
    //TODO 需要检测鼠标键盘是否存在，如果不存在则进行警告
    let server_config = CONFIG.server.as_ref().expect("配置文件错误");
    let (tx, rx) = std::sync::mpsc::channel::<Event>();

    server::start(server_config.ip.as_str(), server_config.port, rx)?;

    let handle_event = move |event: Event| {
        tx.send(event)
            .unwrap_or_else(|e| warn!("send event error: {:?}", e))
    };

    info!("start server success");
    if let Err(error) = listen(handle_event) {
        panic!("监听鼠标键盘失败: {:?}", error);
    }

    Ok(())
}

pub fn active_client(x: f64, y: f64) {
    //判断是否由另一个屏幕在激活状态
    if let Ok(client) = ACTIVE_CLIENT.read() {
        //如果另一个屏幕存在激活状态，则不在进行计算鼠标是否在当前屏幕。
        if client.is_some() {
            return;
        }
    }

    if let Ok(clients) = CLIENTS.read() {
        //判断是否有客户端接入
        if clients.len() > 0 {
            //如果有客户端接入则计算鼠标是否在当前屏幕
            if x as u64 >= DISPLAY.width - 10 {
                if let Ok(mut client) = ACTIVE_CLIENT.write() {
                    *client = Some(clients[0]);
                }
            }
        }
    }
}
