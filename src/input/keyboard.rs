use glutin::event::VirtualKeyCode;
use keyboard_types::{Code, Key};

pub fn vcode_to_code(code: VirtualKeyCode) -> Code {
    use Code::*;
    match code {
        VirtualKeyCode::Key1 => Digit1,
        VirtualKeyCode::Key2 => Digit2,
        VirtualKeyCode::Key3 => Digit3,
        VirtualKeyCode::Key4 => Digit4,
        VirtualKeyCode::Key5 => Digit5,
        VirtualKeyCode::Key6 => Digit6,
        VirtualKeyCode::Key7 => Digit7,
        VirtualKeyCode::Key8 => Digit8,
        VirtualKeyCode::Key9 => Digit9,
        VirtualKeyCode::Key0 => Digit0,
        VirtualKeyCode::A => KeyA,
        VirtualKeyCode::B => KeyB,
        VirtualKeyCode::C => KeyC,
        VirtualKeyCode::D => KeyD,
        VirtualKeyCode::E => KeyE,
        VirtualKeyCode::F => KeyF,
        VirtualKeyCode::G => KeyG,
        VirtualKeyCode::H => KeyH,
        VirtualKeyCode::I => KeyI,
        VirtualKeyCode::J => KeyJ,
        VirtualKeyCode::K => KeyK,
        VirtualKeyCode::L => KeyL,
        VirtualKeyCode::M => KeyM,
        VirtualKeyCode::N => KeyN,
        VirtualKeyCode::O => KeyO,
        VirtualKeyCode::P => KeyP,
        VirtualKeyCode::Q => KeyQ,
        VirtualKeyCode::R => KeyR,
        VirtualKeyCode::S => KeyS,
        VirtualKeyCode::T => KeyT,
        VirtualKeyCode::U => KeyU,
        VirtualKeyCode::V => KeyV,
        VirtualKeyCode::W => KeyW,
        VirtualKeyCode::X => KeyX,
        VirtualKeyCode::Y => KeyY,
        VirtualKeyCode::Z => KeyZ,
        VirtualKeyCode::Escape => Escape,
        VirtualKeyCode::F1 => F1,
        VirtualKeyCode::F2 => F2,
        VirtualKeyCode::F3 => F3,
        VirtualKeyCode::F4 => F4,
        VirtualKeyCode::F5 => F5,
        VirtualKeyCode::F6 => F6,
        VirtualKeyCode::F7 => F7,
        VirtualKeyCode::F8 => F8,
        VirtualKeyCode::F9 => F9,
        VirtualKeyCode::F10 => F10,
        VirtualKeyCode::F11 => F11,
        VirtualKeyCode::F12 => F12,
        VirtualKeyCode::Insert => Insert,
        VirtualKeyCode::Home => Home,
        VirtualKeyCode::Delete => Delete,
        VirtualKeyCode::End => End,
        VirtualKeyCode::PageDown => PageDown,
        VirtualKeyCode::PageUp => PageUp,
        VirtualKeyCode::Left => ArrowLeft,
        VirtualKeyCode::Up => ArrowUp,
        VirtualKeyCode::Right => ArrowRight,
        VirtualKeyCode::Down => ArrowDown,
        VirtualKeyCode::Back => Backspace,
        VirtualKeyCode::Return => Enter,
        VirtualKeyCode::Space => Space,
        VirtualKeyCode::Numpad0 => Numpad0,
        VirtualKeyCode::Numpad1 => Numpad1,
        VirtualKeyCode::Numpad2 => Numpad2,
        VirtualKeyCode::Numpad3 => Numpad3,
        VirtualKeyCode::Numpad4 => Numpad4,
        VirtualKeyCode::Numpad5 => Numpad5,
        VirtualKeyCode::Numpad6 => Numpad6,
        VirtualKeyCode::Numpad7 => Numpad7,
        VirtualKeyCode::Numpad8 => Numpad8,
        VirtualKeyCode::Numpad9 => Numpad9,
        VirtualKeyCode::NumpadAdd => NumpadAdd,
        VirtualKeyCode::NumpadDivide => NumpadDivide,
        VirtualKeyCode::NumpadDecimal => NumpadDecimal,
        VirtualKeyCode::NumpadComma => NumpadComma,
        VirtualKeyCode::NumpadEnter => NumpadEnter,
        VirtualKeyCode::NumpadEquals => NumpadEqual,
        VirtualKeyCode::NumpadMultiply => NumpadMultiply,
        VirtualKeyCode::NumpadSubtract => NumpadSubtract,
        VirtualKeyCode::Comma => Comma,
        VirtualKeyCode::Equals => Equal,
        VirtualKeyCode::Grave => Backquote,
        VirtualKeyCode::LAlt => AltLeft,
        VirtualKeyCode::LBracket => BracketLeft,
        VirtualKeyCode::LControl => ControlLeft,
        VirtualKeyCode::LShift => ShiftLeft,
        VirtualKeyCode::Minus => Minus,
        VirtualKeyCode::Period => Period,
        VirtualKeyCode::Semicolon => Semicolon,
        VirtualKeyCode::Slash => Slash,
        VirtualKeyCode::Tab => Tab,
        _ => NonConvert,
    }
}

pub fn scan_to_code(scan_code: u32) -> Code {
    use Code::*;
    match scan_code {
        0x1 => Escape,
        0x2 => Digit1,
        0x3 => Digit2,
        0x4 => Digit3,
        0x5 => Digit4,
        0x6 => Digit5,
        0x7 => Digit6,
        0x8 => Digit7,
        0x9 => Digit8,
        0xA => Digit9,
        0xB => Digit0,
        0xC => Minus,
        0xD => Equal,
        0xE => Backspace,
        0xF => Tab,
        0x10 => KeyQ,
        0x11 => KeyW,
        0x12 => KeyE,
        0x13 => KeyR,
        0x14 => KeyT,
        0x15 => KeyY,
        0x16 => KeyU,
        0x17 => KeyI,
        0x18 => KeyO,
        0x19 => KeyP,
        0x1A => BracketLeft,
        0x1B => BracketRight,
        0x1C => Enter,
        0x1D => ControlLeft,
        0x1E => KeyA,
        0x1F => KeyS,
        0x20 => KeyD,
        0x21 => KeyF,
        0x22 => KeyG,
        0x23 => KeyH,
        0x24 => KeyJ,
        0x25 => KeyK,
        0x26 => KeyL,
        0x27 => Semicolon,
        0x28 => Quote,
        0x29 => Backquote,
        0x2A => ShiftLeft,
        0x2B => Backslash,
        0x2C => KeyZ,
        0x2D => KeyX,
        0x2E => KeyC,
        0x2F => KeyV,
        0x30 => KeyB,
        0x31 => KeyN,
        0x32 => KeyM,
        0x33 => Comma,
        0x34 => Period,
        0x35 => Slash,
        0x36 => ShiftRight,
        0x37 => NumpadMultiply,
        0x38 => AltLeft,
        0x39 => Space,
        0x3A => CapsLock,
        0x3B => F1,
        0x3C => F2,
        0x3D => F3,
        0x3E => F4,
        0x3F => F5,
        0x40 => F6,
        0x41 => F7,
        0x42 => F8,
        0x43 => F9,
        0x44 => F10,
        0x45 => Pause,
        0x46 => ScrollLock,
        0x47 => Numpad7,
        0x48 => Numpad8,
        0x49 => Numpad9,
        0x4A => NumpadSubtract,
        0x4B => Numpad4,
        0x4C => Numpad5,
        0x4D => Numpad6,
        0x4E => NumpadAdd,
        0x4F => Numpad1,
        0x50 => Numpad2,
        0x51 => Numpad3,
        0x52 => Numpad0,
        0x53 => NumpadDecimal,
        0x54 => PrintScreen,
        0x56 => IntlBackslash,
        0x57 => F11,
        0x58 => F12,
        0x59 => NumpadEqual,
        0x70 => KanaMode,
        0x71 => Lang2,
        0x72 => Lang1,
        0x73 => IntlRo,
        0x79 => Convert,
        0x7B => NonConvert,
        0x7D => IntlYen,
        0x7E => NumpadComma,
        0x110 => MediaTrackPrevious,
        0x119 => MediaTrackNext,
        0x11C => NumpadEnter,
        0x11D => ControlRight,
        0x120 => AudioVolumeMute,
        0x121 => LaunchApp2,
        0x122 => MediaPlayPause,
        0x124 => MediaStop,
        0x12E => AudioVolumeDown,
        0x130 => AudioVolumeUp,
        0x132 => BrowserHome,
        0x135 => NumpadDivide,
        0x137 => PrintScreen,
        0x138 => AltRight,
        0x145 => NumLock,
        0x147 => Home,
        0x148 => ArrowUp,
        0x149 => PageUp,
        0x14B => ArrowLeft,
        0x14D => ArrowRight,
        0x14F => End,
        0x150 => ArrowDown,
        0x151 => PageDown,
        0x152 => Insert,
        0x153 => Delete,
        0x15B => MetaLeft,
        0x15C => MetaRight,
        0x15D => ContextMenu,
        0x15E => Power,
        0x165 => BrowserSearch,
        0x166 => BrowserFavorites,
        0x167 => BrowserRefresh,
        0x168 => BrowserStop,
        0x169 => BrowserForward,
        0x16A => BrowserBack,
        0x16B => LaunchApp1,
        0x16C => LaunchMail,
        0x16D => MediaSelect,
        0x1F1 => Lang2,
        0x1F2 => Lang1,
        _ => Unidentified,
    }
}

pub fn vk_to_key(vk: VirtualKeyCode) -> Option<Key> {
    Some(match vk {
        VirtualKeyCode::Back => Key::Backspace,
        VirtualKeyCode::Tab => Key::Tab,
        VirtualKeyCode::Return => Key::Enter,
        VirtualKeyCode::LShift | VirtualKeyCode::RShift => Key::Shift,
        VirtualKeyCode::LControl | VirtualKeyCode::RControl => Key::Control,
        VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => Key::Alt,
        VirtualKeyCode::Pause => Key::Pause,
        VirtualKeyCode::Capital => Key::CapsLock,
        // TODO: disambiguate kana and hangul? same vk
        VirtualKeyCode::Kana => Key::KanaMode,
        VirtualKeyCode::Kanji => Key::KanjiMode,
        VirtualKeyCode::Escape => Key::Escape,
        VirtualKeyCode::PageUp => Key::PageUp,
        VirtualKeyCode::PageDown => Key::PageDown,
        VirtualKeyCode::End => Key::End,
        VirtualKeyCode::Home => Key::Home,
        VirtualKeyCode::Left => Key::ArrowLeft,
        VirtualKeyCode::Up => Key::ArrowUp,
        VirtualKeyCode::Right => Key::ArrowRight,
        VirtualKeyCode::Down => Key::ArrowDown,
        VirtualKeyCode::MediaSelect => Key::Select,
        VirtualKeyCode::Snapshot => Key::Print,
        VirtualKeyCode::Insert => Key::Insert,
        VirtualKeyCode::Delete => Key::Delete,
        VirtualKeyCode::LWin | VirtualKeyCode::RWin => Key::Meta,
        VirtualKeyCode::Apps => Key::ContextMenu,
        VirtualKeyCode::Sleep => Key::Standby,
        VirtualKeyCode::F1 => Key::F1,
        VirtualKeyCode::F2 => Key::F2,
        VirtualKeyCode::F3 => Key::F3,
        VirtualKeyCode::F4 => Key::F4,
        VirtualKeyCode::F5 => Key::F5,
        VirtualKeyCode::F6 => Key::F6,
        VirtualKeyCode::F7 => Key::F7,
        VirtualKeyCode::F8 => Key::F8,
        VirtualKeyCode::F9 => Key::F9,
        VirtualKeyCode::F10 => Key::F10,
        VirtualKeyCode::F11 => Key::F11,
        VirtualKeyCode::F12 => Key::F12,
        VirtualKeyCode::Numlock => Key::NumLock,
        VirtualKeyCode::Scroll => Key::ScrollLock,
        // VirtualKeyCode::BROWSER_BACK => Key::BrowserBack,
        // VirtualKeyCode::BROWSER_FORWARD => Key::BrowserForward,
        // VirtualKeyCode::BROWSER_REFRESH => Key::BrowserRefresh,
        // VirtualKeyCode::BROWSER_STOP => Key::BrowserStop,
        // VirtualKeyCode::BROWSER_SEARCH => Key::BrowserSearch,
        // VirtualKeyCode::BROWSER_FAVORITES => Key::BrowserFavorites,
        // VirtualKeyCode::BROWSER_HOME => Key::BrowserHome,
        // VirtualKeyCode::VOLUME_MUTE => Key::AudioVolumeMute,
        // VirtualKeyCode::VOLUME_DOWN => Key::AudioVolumeDown,
        // VirtualKeyCode::VOLUME_UP => Key::AudioVolumeUp,
        // VirtualKeyCode::MEDIA_NEXT_TRACK => Key::MediaTrackNext,
        // VirtualKeyCode::MEDIA_PREV_TRACK => Key::MediaTrackPrevious,
        // VirtualKeyCode::MEDIA_STOP => Key::MediaStop,
        // VirtualKeyCode::MEDIA_PLAY_PAUSE => Key::MediaPlayPause,
        // VirtualKeyCode::LAUNCH_MAIL => Key::LaunchMail,
        // VirtualKeyCode::LAUNCH_MEDIA_SELECT => Key::LaunchMediaPlayer,
        // VirtualKeyCode::LAUNCH_APP1 => Key::LaunchApplication1,
        // VirtualKeyCode::LAUNCH_APP2 => Key::LaunchApplication2,
        // VirtualKeyCode::OEM_ATTN => Key::Alphanumeric,
        // VirtualKeyCode::CONVERT => Key::Convert,
        // VirtualKeyCode::MODECHANGE => Key::ModeChange,
        // VirtualKeyCode::PROCESSKEY => Key::Process,
        // VirtualKeyCode::ATTN => Key::Attn,
        // VirtualKeyCode::CRSEL => Key::CrSel,
        // VirtualKeyCode::EXSEL => Key::ExSel,
        // VirtualKeyCode::EREOF => Key::EraseEof,
        // VirtualKeyCode::PLAY => Key::Play,
        // VirtualKeyCode::ZOOM => Key::ZoomToggle,
        // VirtualKeyCode::OEM_CLEAR => Key::Clear,
        _ => return None,
    })
}