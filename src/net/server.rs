use anyhow::Result;
use log::{debug, info, warn};
use std::{net::UdpSocket, sync::Arc, thread};

use super::protocol::Protocol;

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
        //TODO
        //self.socket.send(protocol.into())?;
        //self.socket.send(&protocol.to_arr());
        //socket.send_to(buf, &src)?;
        Ok(())
    }
}

pub fn start(ip: &str, port: u16) -> Result<UdpServer> {
    let udp = UdpServer::new(ip, port)?;
    let socket = udp.socket.clone();
    thread::spawn(move || loop {
        // max 1472 bytes, mtu(1500) - udp header(8) - ip header(20) = 1472
        //每次传输报文控制在最大1472字节，防止分片传输
        //每次接收512字节，最长不超过512
        let mut buf = [0u8; 1];
        let recv = socket.recv_from(&mut buf);
        if let Ok((len, addr)) = recv {
            debug!("recv from {:?}, {:?}", addr.ip(), addr.port());
        } else {
            warn!("接收数据错误: {:?}", recv);
        }
        //let (len, addr) = udp.socket.recv_from(&mut buf)?;
        //let protocol = Protocol::from_arr(&buf[..len]);
    });
    Ok(udp)
}
