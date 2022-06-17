use log::warn;
use rdev::{Button, EventType, Key};

pub const PROTOCOL_LEN: usize = 19;

/// 通信协议
#[derive(Debug)]
pub struct Protocol {
    /// 标记
    pub flag: Flag,
    /// 鼠标键盘类型
    pub key_mouse: KeyMouse,
    /// 触发事件
    pub event: Event,
}

impl Protocol {
    pub fn to_arr(self) -> [u8; PROTOCOL_LEN] {
        let mut buf = [0u8; PROTOCOL_LEN];
        let (flag, arr) = buf.split_at_mut(1);
        let (key_mouse, event) = arr.split_at_mut(1);
        flag[0] = (&self.flag).into();
        key_mouse[0] = (&self.key_mouse).into();
        let e: [u8; 17] = (&self.event).into();
        event.copy_from_slice(&e);
        buf
    }
}

impl From<rdev::Event> for Protocol {
    fn from(e: rdev::Event) -> Self {
        e.event_type.into()
    }
}

impl From<EventType> for Protocol {
    fn from(et: EventType) -> Self {
        //Protocol { flag: Flag::KeyMouse, key_mouse: , event: () }
        let (key_mouse, event) = match et {
            EventType::KeyPress(k) => (k.into(), Event::Press),
            EventType::KeyRelease(k) => (k.into(), Event::Release),
            EventType::ButtonPress(b) => (b.into(), Event::Press),
            EventType::ButtonRelease(b) => (b.into(), Event::Release),
            EventType::MouseMove { x, y } => (KeyMouse::MouseMove, Event::Move(x, y)),
            EventType::Wheel { delta_x, delta_y } => (
                KeyMouse::MouseMiddle,
                Event::Move(delta_x as f64, delta_y as f64),
            ),
        };
        Self {
            flag: Flag::KeyMouse,
            key_mouse,
            event,
        }
    }
}

impl From<&[u8]> for Protocol {
    fn from(buf: &[u8]) -> Self {
        let flag = if buf.len() > 0 {
            Flag::from(buf[0])
        } else {
            Flag::Unknown
        };
        let key_mouse = if buf.len() > 1 {
            KeyMouse::from(buf[1])
        } else {
            KeyMouse::Unknown
        };
        let event = if buf.len() == PROTOCOL_LEN {
            //Event::from(&buf[2..])
            Event::Unknown
        } else {
            Event::Unknown
        };
        Protocol {
            flag,
            key_mouse,
            event,
        }
    }
}

macro_rules! from_u8 {
    ($type:ident, $($key:ident = $code:literal),*) => {

        impl From<u8> for $type {
            fn from(value: u8) -> Self {
                match value {
                    $(
                        $code => $type::$key,
                    )*
                    _ => {
                        warn!("unknown {} value: {:?}", stringify!($type), value);
                        $type::Unknown
                    },
                }
            }
        }

        impl From<&$type> for u8 {
            fn from(value: &$type) -> Self {
                match value {
                    $(
                        $type::$key => $code,
                    )*
                    _ =>{
                        warn!("unknown {} enum: {:?}", stringify!($type), value);
                        0
                    },
                }
            }
        }
    }
}

macro_rules! from_key {
    ($type:ident, $($key:ident),*) => {

        impl From<Key> for $type {
            fn from(k: Key) -> Self {
                match k {
                    $(
                        Key::$key => $type::$key,
                    )*
                    _ => {
                        warn!("Unknown key: {:?}", k);
                        KeyMouse::Unknown
                    },
                }
            }
        }
    }
}

/// 标记
#[derive(Debug)]
pub enum Flag {
    /// 0x01键盘鼠标触发
    KeyMouse,
    /// 0x02复制粘贴
    CopyPaste,
    /// 0x03客户端初始化连接
    ClientInitConnection,
    /// 0x00未知数据
    Unknown,
}

// 十六进制映射
from_u8!(
    Flag,
    KeyMouse = 0x01,
    CopyPaste = 0x02,
    ClientInitConnection = 0x03
);

/// 鼠标键盘
#[derive(Debug)]
pub enum KeyMouse {
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    /// also known as "windows", "super", and "command"
    MetaLeft,
    /// also known as "windows", "super", and "command"
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    MouseMove,
    MouseLeft,
    MouseRight,
    MouseMiddle,
    /// 0x00未知数据
    Unknown,
}

// // 映射鼠标键盘为十六进制
// // 鼠标键盘映射表：https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
from_u8!(
    KeyMouse,
    Alt = 0x12,
    //AltGr = 0x11,
    Backspace = 0x08,
    CapsLock = 0x14,
    ControlLeft = 0xA2,
    ControlRight = 0xA3,
    Delete = 0x2E,
    DownArrow = 0x28,
    End = 0x23,
    Escape = 0x1B,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    Home = 0x24,
    LeftArrow = 0x25,
    MetaLeft = 0xA4,
    MetaRight = 0xA5,
    PageDown = 0x22,
    PageUp = 0x21,
    Return = 0x0D,
    RightArrow = 0x27,
    ShiftLeft = 0xA0,
    ShiftRight = 0xA1,
    Space = 0x20,
    Tab = 0x09,
    UpArrow = 0x26,
    PrintScreen = 0x2C,
    ScrollLock = 0x91,
    Pause = 0x13,
    NumLock = 0x90,
    BackQuote = 0xC0,
    Num0 = 0x30,
    Num1 = 0x31,
    Num2 = 0x32,
    Num3 = 0x33,
    Num4 = 0x34,
    Num5 = 0x35,
    Num6 = 0x36,
    Num7 = 0x37,
    Num8 = 0x38,
    Num9 = 0x39,
    KeyA = 0x41,
    KeyB = 0x42,
    KeyC = 0x43,
    KeyD = 0x44,
    KeyE = 0x45,
    KeyF = 0x46,
    KeyG = 0x47,
    KeyH = 0x48,
    KeyI = 0x49,
    KeyJ = 0x4A,
    KeyK = 0x4B,
    KeyL = 0x4C,
    KeyM = 0x4D,
    KeyN = 0x4E,
    KeyO = 0x4F,
    KeyP = 0x50,
    KeyQ = 0x51,
    KeyR = 0x52,
    KeyS = 0x53,
    KeyT = 0x54,
    KeyU = 0x55,
    KeyV = 0x56,
    KeyW = 0x57,
    KeyX = 0x58,
    KeyY = 0x59,
    KeyZ = 0x5A,
    Minus = 0xBD,
    //Equal = 0x53,
    //LeftBracket = 0x1A,
    //RightBracket = 0x1B,
    //SemiColon = 0x27,
    //Quote = 0x28,
    //BackSlash = 0x2B,
    //IntlBackslash = 0x56,
    Comma = 0xBC,
    Dot = 0xBE,
    //Slash = 0x35,
    Insert = 0x2D,
    //KpReturn = 0x4C,
    //KpMinus = 0x4A,
    //KpPlus = 0x4E,
    //KpMultiply = 0x6A,
    //KpDivide = 0x6F,
    Kp0 = 0x60,
    Kp1 = 0x61,
    Kp2 = 0x62,
    Kp3 = 0x63,
    Kp4 = 0x64,
    Kp5 = 0x65,
    Kp6 = 0x66,
    Kp7 = 0x67,
    Kp8 = 0x68,
    Kp9 = 0x69,
    //KpDelete = 0x53,
    //Function = 0x3A,
    MouseMove = 0x07,
    MouseLeft = 0x01,
    MouseRight = 0x02,
    MouseMiddle = 0x04
);

from_key!(
    KeyMouse,
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function
);

impl From<Button> for KeyMouse {
    fn from(b: Button) -> Self {
        match b {
            Button::Left => KeyMouse::MouseLeft,
            Button::Right => KeyMouse::MouseRight,
            Button::Middle => KeyMouse::MouseMiddle,
            Button::Unknown(v) => {
                warn!("unknown button: {}", v);
                KeyMouse::Unknown
            }
        }
    }
}

/// 鼠标键盘事件
#[derive(Debug)]
pub enum Event {
    /// 0x01按下按钮事件
    Press,
    /// 0x02释放按钮事件
    Release,
    /// 0x03移动事件
    /// 鼠标、滚轮x轴、y轴偏移
    Move(f64, f64),
    /// 0x00未知数据
    Unknown,
}

impl From<&[u8]> for Event {
    fn from(v: &[u8]) -> Self {
        if v.len() > 0 {
            match v[0] {
                0x01 => Event::Press,
                0x02 => Event::Release,
                0x03 => {
                    if v.len() == PROTOCOL_LEN - 2 {
                        //转换为x、y轴偏移数据
                        let x = f64::from_be_bytes(v[1..8].try_into().unwrap_or_default());
                        let y = f64::from_be_bytes(v[9..16].try_into().unwrap_or_default());
                        Event::Move(x, y)
                    } else {
                        Event::Unknown
                    }
                }
                _ => Event::Unknown,
            }
        } else {
            Event::Unknown
        }
    }
}

impl From<&Event> for [u8; 17] {
    fn from(v: &Event) -> Self {
        let mut buf = [0u8; 17];
        let (flag, xy) = buf.split_at_mut(1);
        let (x, y) = xy.split_at_mut(8);
        match v {
            Event::Press => flag[0] = 0x01,
            Event::Release => flag[0] = 0x02,
            //TODO 需要实现将数据转换为x、y轴偏移数据
            Event::Move(xf, yf) => {
                flag[0] = 0x03;
                x.copy_from_slice(&xf.to_be_bytes());
                y.copy_from_slice(&yf.to_be_bytes());
            }
            Event::Unknown => flag[0] = 0x00,
        };
        buf
    }
}

#[cfg(test)]
mod test {
    use super::{Event, Flag, KeyMouse, Protocol, PROTOCOL_LEN};

    #[test]
    fn test_protocol_to_u8() {
        let p = Protocol {
            flag: Flag::KeyMouse,
            key_mouse: KeyMouse::MouseMove,
            event: Event::Move(0.1, 0.1),
        };
        let buf: [u8; PROTOCOL_LEN] = p.to_arr();
        assert_eq!(
            buf,
            [
                0x01, 0x07, 0x03, 0x3F, 0xB9, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A, 0x3F, 0xB9, 0x99,
                0x99, 0x99, 0x99, 0x99, 0x9A,
            ]
        );
    }
}
