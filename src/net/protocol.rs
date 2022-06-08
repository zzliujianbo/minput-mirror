use log::warn;
use rdev::{Button, EventType, Key};

/// 通信协议
pub struct Protocol {
    /// 标记
    pub flag: Flag,
    /// 鼠标键盘类型
    pub key_mouse: KeyMouse,
    /// 触发事件
    pub event: Event,
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

        impl From<$type> for u8 {
            fn from(value: $type) -> Self {
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
    /// 0x00未知数据
    Unknown,
}

// 十六进制映射
from_u8!(Flag, KeyMouse = 0x01, CopyPaste = 0x02);

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
pub enum Event {
    /// 0x01按下按钮事件
    Press,
    /// 0x02释放按钮事件
    Release,
    /// 0x03移动事件
    /// 鼠标、滚轮x轴、y轴偏移
    Move(f64, f64),
}

impl From<u8> for Event {
    fn from(v: u8) -> Self {
        match v {
            0x01 => Event::Press,
            0x02 => Event::Release,
            //TODO 需要实现将数据转换为x、y轴偏移数据
            0x03 => Event::Move(0.0, 0.0),
            _ => panic!("unknown event value: {}", v),
        }
    }
}

impl From<Event> for u8 {
    fn from(v: Event) -> Self {
        match v {
            Event::Press => 0x01,
            Event::Release => 0x02,
            Event::Move(_, _) => 0x03,
        }
    }
}
