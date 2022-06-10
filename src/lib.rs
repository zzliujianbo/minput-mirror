use env_logger::{fmt::Color, Builder, Env};
use lazy_static::lazy_static;
use log::{error, info};
use rdev::display_size;
use serde::{Deserialize, Serialize};
use std::io::Write;
mod dev;
mod net;

lazy_static! {
    ///配置对象
    static ref CONFIG: Config = get_config();
    static ref DISPLAY: Display = get_display();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mode: String,
    pub server: Option<ConfigServer>,
    pub client: Option<ConfigClient>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigServer {
    ///服务器监听地址
    pub ip: String,
    ///服务器监听端口
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigClient {
    ///服务器地址
    pub server_ip: String,
    ///服务器端口
    pub server_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Display {
    ///屏幕分辨率宽度
    pub width: u64,
    ///屏幕分辨率高度
    pub height: u64,
}

impl Display {
    pub fn new(width: u64, height: u64) -> Self {
        Display { width, height }
    }
}

/// 初始化日志
fn init_logger() {
    //默认INFO日志级别
    let env = Env::default().default_filter_or("trace");
    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            match record.level() {
                log::Level::Error => style.set_color(Color::Red).set_bold(true),
                log::Level::Warn => style.set_color(Color::Yellow).set_bold(true),
                log::Level::Info => style.set_color(Color::Green),
                log::Level::Debug => style.set_color(Color::Blue),
                log::Level::Trace => style.set_color(Color::Black),
            };

            writeln!(
                buf,
                "[{} {:^5} {}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                style.value(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or_default(),
                record.args()
            )
        })
        .init();
}

///获取配置
fn get_config() -> Config {
    let f = std::fs::File::open("config.yaml").expect("读取配置文件失败");
    let config: Config = serde_yaml::from_reader(f).expect("配置文件解析失败");
    config
}

///获取屏幕分辨率
fn get_display_size() -> (u64, u64) {
    display_size().expect("获取屏幕分辨率失败")
}

///获取屏幕详情
fn get_display() -> Display {
    let (w, h) = get_display_size();
    Display::new(w, h)
}

///启动服务
fn start_service() {
    let result = match CONFIG.mode.as_str() {
        "server" => dev::server::start(),
        "client" => dev::client::start(),
        _ => {
            panic!("配置文件mode错误(允许值：server/client)");
        }
    };

    if let Err(e) = result {
        error!("异常退出：{}", e);
    }
}

pub fn start() {
    init_logger();
    info!("config info: {:#?}", *CONFIG);
    info!("display info: {:#?}", *DISPLAY);
    start_service();
    info!("exit");
}
