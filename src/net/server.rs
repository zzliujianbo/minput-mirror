use anyhow::{Ok, Result};
use log::{debug, info};
use std::net::UdpSocket;

pub fn start(ip: &str, port: u16) -> Result<()> {
    //TODO 需要检测鼠标键盘是否存在，如果不存在则进行警告
    let socket = UdpSocket::bind((ip, port))?;
    info!("start server [{}:{}] success", ip, port);

    //let mut buf = [0; 10];
    // max 1472 bytes, mtu(1500) - udp header(8) - ip header(20) = 1472
    //每次传输报文控制在最大1472字节，防止分片传输
    //每次接收512字节，最长不超过512
    let mut buf = vec![];
    debug!("buf length: {}", buf.len());
    let (amt, src) = socket.recv_from(&mut buf)?;
    debug!("amt: {}", amt);
    debug!("buf: {:?}", buf);
    //let buf = &mut buf[..amt];
    //buf.reverse();
    let buf = &[0x01, 0xef];
    socket.send_to(buf, &src)?;
    Ok(())
}
