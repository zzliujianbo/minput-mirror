use anyhow::Result;
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use rdev::{Event, EventType};
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{mpsc::Receiver, Arc, RwLock},
    thread, vec,
};

use crate::{net::protocol::Flag, DISPLAY};

use super::protocol::{Protocol, PROTOCOL_LEN};

lazy_static! {
    pub(crate) static ref ACTIVE_CLIENT: RwLock<Option<SocketAddr>> = RwLock::new(None);
    static ref CLIENTS: RwLock<Vec<SocketAddr>> = RwLock::new(vec![]);
}

pub struct UdpServer {
    socket: Arc<UdpSocket>,
}

impl UdpServer {
    pub fn new(ip: &str, port: u16) -> Result<Self> {
        let socket = UdpSocket::bind((ip, port))?;
        debug!("UdpSocket bind to {}:{}", ip, port);
        Ok(Self {
            socket: Arc::new(socket),
        })
    }

    pub fn send(&self, protocol: Protocol) -> Result<()> {
        //if let Ok(client) = ACTIVE_CLIENT.read() {
        //    if client.is_some() {
        //        self.socket.send_to(&protocol.to_arr(), client.unwrap())?;
        //    }
        //}
        Ok(())
    }
}

pub fn start(ip: &str, port: u16, rx: Receiver<Event>) -> Result<()> {
    let udp = Arc::new(UdpServer::new(ip, port)?);
    let udp_clone = udp.clone();
    thread::spawn(move || {
        for event in rx.iter() {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    //active_client(x, y);
                }
                _ => {}
            }
            let result = udp.send(event.into());
            if result.is_err() {
                error!("send event error: {:?}", result);
            }
        }
    });
    thread::spawn(move || loop {
        // max 1472 bytes, mtu(1500) - udp header(8) - ip header(20) = 1472
        //每次传输报文控制在最大1472字节，防止分片传输
        //每次接收512字节，最长不超过512
        let mut buf = [0u8; PROTOCOL_LEN];
        //let recv = udp_clone.socket.recv_from(&mut buf);
        let recv = udp_clone.socket.recv_from(&mut buf);
        if let Ok((_, addr)) = recv {
            let protocol = Protocol::from(&buf[..]);
            debug!(
                "recv from {:?}, {:?}, {:?}",
                addr.ip(),
                addr.port(),
                protocol
            );
            match protocol.flag {
                Flag::ClientInitConnection => {
                    debug!("client connect: {}", addr);
                    if let Ok(mut clients) = CLIENTS.write() {
                        if clients.contains(&addr) {
                            warn!("client {} already exist", addr);
                        } else {
                            debug!("add {} client success", addr);
                            clients.push(addr);
                        }
                    } else {
                        error!("clients write error");
                    }
                }
                _ => {
                    warn!("unknown protocol: {:?}", protocol);
                }
            }
        } else {
            warn!("接收数据错误: {:?}", recv);
        }
    });
    Ok(())
}
