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
    static ref ACTIVE_CLIENT: Option<SocketAddr> = None;
    static ref CLIENTS: Arc<RwLock<Vec<SocketAddr>>> = Arc::new(RwLock::new(vec![]));
}

pub struct UdpServer {
    //socket: Arc<UdpSocket>,
    socket: Arc<UdpSocket>,
    active_client: Option<SocketAddr>,
    clients: Vec<SocketAddr>,
}

impl UdpServer {
    pub fn new(ip: &str, port: u16) -> Result<Self> {
        let socket = UdpSocket::bind((ip, port))?;
        debug!("UdpSocket bind to {}:{}", ip, port);
        Ok(Self {
            socket: Arc::new(socket),
            clients: vec![],
            active_client: None,
        })
    }

    pub fn send(&self, protocol: Protocol, client: &SocketAddr) -> Result<()> {
        //TODO
        //self.socket.send(protocol.into())?;
        //self.socket.send(&protocol.to_arr());
        self.socket.send_to(&protocol.to_arr(), client)?;
        Ok(())
    }
}

pub fn start(ip: &str, port: u16, rx: Receiver<Event>) -> Result<()> {
    let udp = Arc::new(UdpServer::new(ip, port)?);
    let udp_clone = udp.clone();
    thread::spawn(move || {
        for event in rx.iter() {
            debug!("event: {:?}", event);
            match event.event_type {
                EventType::KeyPress(k) => {
                    debug!("key down: {:?}", k);
                    //udp_clone.send(protocol)?;
                    let client = active_client(0_f64, 0_f64);
                    if let Some(client) = client {
                        //let protocol = Protocol::new(Flag::MouseMove, x, y);
                        udp.send(event.into(), &client);
                    }
                }
                EventType::MouseMove { x, y } => {
                    //debug!("mouse move: ({}, {})", x, y);
                    //udp_clone.send(protocol)?;
                }
                _ => {}
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
        if let Ok((len, addr)) = recv {
            let protocol = Protocol::from(&buf[..]);
            debug!(
                "recv from {:?}, {:?}, {:?}",
                addr.ip(),
                addr.port(),
                protocol
            );
            match protocol.flag {
                Flag::ClientInitConnection => {
                    info!("client connect: {}", addr);
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

pub fn active_client(x: f64, y: f64) -> Option<SocketAddr> {
    if let Ok(clients) = CLIENTS.read() {
        if clients.len() > 0 {
            return Some(clients[0]);
        }
    }
    None
}
