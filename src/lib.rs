use env_logger::{fmt::Color, Builder, Env};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use std::io::Write;

lazy_static! {
    ///配置对象
    static ref CONFIG: Config = init_config();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mode: String,
    pub server: Option<ConfigServer>,
    pub client: Option<ConfigClient>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigServer {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigClient {}

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

///初始化配置
fn init_config() -> Config {
    let f = std::fs::File::open("config.yaml").expect("读取配置文件失败");
    let config: Config = serde_yaml::from_reader(f).expect("配置文件解析失败");
    config
}

pub fn start() {
    init_logger();
    info!("read config: {:#?}", *CONFIG);
    info!("start {}", CONFIG.mode);
}
