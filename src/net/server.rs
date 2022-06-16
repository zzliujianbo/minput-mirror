use anyhow::Result;
use log::{debug, info, warn};
use rdev::{Event, EventType};
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{mpsc::Receiver, Arc},
    thread,
};

use crate::net::protocol::Flag;

use super::protocol::{Protocol, PROTOCOL_LEN};

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

    pub fn send(&self, protocol: Protocol) -> Result<()> {
        //TODO
        //self.socket.send(protocol.into())?;
        //self.socket.send(&protocol.to_arr());
        //socket.send_to(buf, &src)?;
        Ok(())
    }
}

pub fn start(ip: &str, port: u16, rx: Receiver<Event>) -> Result<()> {
    let udp = Arc::new(UdpServer::new(ip, port)?);
    let udp_clone = udp.clone();

    thread::spawn(move || loop {
        for event in rx.iter() {
            match event.event_type {
                EventType::KeyPress(k) => {
                    debug!("key down: {:?}", k);
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
            match protocol.flag {
                Flag::ClientInitConnection => {
                    info!("client connect: {}", addr);
                    //udp_clone.clients.push(addr);
                }
                _ => {
                    warn!("unknown protocol: {:?}", protocol);
                }
            }
            //if protocol.flag == Flag::ClientInitConnection {
            //    udp_clone.clients.push(addr);
            //}
            debug!(
                "recv from {:?}, {:?}, {:?}",
                addr.ip(),
                addr.port(),
                protocol
            );
        } else {
            warn!("接收数据错误: {:?}", recv);
        }
    });
    Ok(())
}
