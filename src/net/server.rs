use anyhow::{Ok, Result};
use log::{debug, info};
use std::net::UdpSocket;

use super::protocol::Protocol;

pub struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    pub fn new(ip: &str, port: u16) -> Result<Self> {
        let socket = UdpSocket::bind((ip, port))?;
        debug!("UdpSocket bind to {}:{}", ip, port);
        Ok(Self { socket })
    }

    pub fn send(&self, protocol: Protocol) -> Result<()> {
        //TODO
        //self.socket.send(protocol.into())?;
        //self.socket.send(&protocol.to_arr());
        Ok(())
    }
}

pub fn start(ip: &str, port: u16) -> Result<UdpServer> {
    UdpServer::new(ip, port)
    //thread::spawn(move || {
    //});

    //let mut buf = [0; 10];
    // max 1472 bytes, mtu(1500) - udp header(8) - ip header(20) = 1472
    //每次传输报文控制在最大1472字节，防止分片传输
    //每次接收512字节，最长不超过512
    //let mut buf = vec![];
    //debug!("buf length: {}", buf.len());
    //let (amt, src) = socket.recv_from(&mut buf)?;
    //debug!("amt: {}", amt);
    //debug!("buf: {:?}", buf);
    ////let buf = &mut buf[..amt];
    ////buf.reverse();
    //let buf = &[0x01, 0xef];
    //socket.send_to(buf, &src)?;
    //Ok(())
}
