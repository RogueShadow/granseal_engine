use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    KeyEvent {
        state: KeyState,
        key: Key,
        modifiers: ModifierState,
    },
    MouseButton {
        state: KeyState,
        button: MouseButton,
        modifiers: ModifierState,
        position: [f64;  2],
    },
    MouseMoved {
        position: [f64; 2],
    },
    Draw,
    Update(Duration),
    Load,
    Resized(u32,u32),
}


#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
}

#[derive(Copy, Clone, Debug)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Copy, Clone, Debug)]
pub struct ModifierState {
    pub(crate) shift: bool,
    pub(crate) alt: bool,
    pub(crate) ctrl: bool,
}

#[derive(Hash,Eq,PartialEq,Debug,Copy,Clone)]
pub enum Key {
    Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Grave, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,Minus, Equals, BackSpace,
    Tab, Q, W, E, R, T, Y, U, I, O, P, LBracket, RBracket, BackSlash,
    CapsLock, A, S, D, F, G, H, J, K, L, SemiColon, Apostrophe, Enter,
    LShift, Z, X, C, V, B, N, M, Comma, Period, ForwardSlash, RShift,
    LCtrl, LWin, LAlt, Space, RAlt, RWin, RCtrl,

    PrintScreen, ScrollLock, Pause,

    Insert, Home, PageUp,
    Delete, End, PageDown,

    Up, Left, Down, Right,

    NumLock, NumDivide, NumMultiply, NumSubtract,
    Num7, Num8, Num9, NumAdd,
    Num4, Num5, Num6,
    Num1, Num2, Num3, NumEnter,
    Num0, NumDecimal,

    NotImplemented,
}



pub fn map_events(event: &winit::event::WindowEvent) -> Option<Event> {
    match event {
        winit::event::WindowEvent::KeyboardInput {
            input: winit::event::KeyboardInput { scancode: _scancode, state, virtual_keycode, modifiers },
            ..
        } => {
            if virtual_keycode.is_none() {return None;}
            Some(Event::KeyEvent {
                state: match state {
                    winit::event::ElementState::Pressed => {KeyState::Pressed}
                    winit::event::ElementState::Released => {KeyState::Released}
                },
                key: map_keys(&virtual_keycode.unwrap() ),
                modifiers: map_modifiers(modifiers)
            })
        },
        winit::event::WindowEvent::MouseInput { device_id: _device_id, state, button, modifiers } => {
            Some(Event::MouseButton {
                state: match state {
                    winit::event::ElementState::Pressed => {KeyState::Pressed}
                    winit::event::ElementState::Released => {KeyState::Released}
                },
                button: map_mouse_buttons(button),
                modifiers: map_modifiers(modifiers),
                position: [0.0,0.0]
            })
        }
        _ => None
    }
}

fn map_mouse_buttons(button: &winit::event::MouseButton) -> MouseButton {
    match button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Other(b) => MouseButton::Other(*b),
    }
}

fn map_modifiers(modifiers: &winit::event::ModifiersState) -> ModifierState {
    ModifierState {
        shift: modifiers.shift(),
        alt: modifiers.alt(),
        ctrl: modifiers.ctrl(),
    }
}

fn map_keys(key: &winit::event::VirtualKeyCode) -> Key {

    use winit::event::VirtualKeyCode as V;
    use Key as G;


    let key = match key {
        V::Escape => G::Escape,
        V::F1 => G::F1,
        V::F2 => G::F2,
        V::F3 => G::F3,
        V::F4 => G::F4,
        V::F5 => G::F5,
        V::F6 => G::F6,
        V::F7 => G::F7,
        V::F8 => G::F8,
        V::F9 => G::F9,
        V::F10 => G::F10,
        V::F11 => G::F11,
        V::F12 => G::F12,
        V::Grave => G::Grave,
        V::Key1 => G::Key1,
        V::Key2 => G::Key2,
        V::Key3 => G::Key3,
        V::Key4 => G::Key4,
        V::Key5 => G::Key5,
        V::Key6 => G::Key6,
        V::Key7 => G::Key7,
        V::Key8 => G::Key8,
        V::Key9 => G::Key9,
        V::Key0 => G::Key0,
        V::Minus => G::Minus,
        V::Equals => G::Equals,
        V::Back => G::BackSpace,
        V::Tab => G::Tab,
        V::Q => G::Q,
        V::W => G::W,
        V::E => G::E,
        V::R => G::R,
        V::T => G::T,
        V::Y => G::Y,
        V::U => G::U,
        V::I => G::I,
        V::O => G::O,
        V::P => G::P,
        V::LBracket => G::LBracket,
        V::RBracket => G::RBracket,
        V::Backslash => G::BackSlash,
        V::Capital => G::CapsLock,
        V::A => G::A,
        V::S => G::S,
        V::D => G::D,
        V::F=> G::F,
        V::G => G::G,
        V::H => G::H,
        V::J => G::J,
        V::K => G::K,
        V::L => G::L,
        V::Semicolon => G::SemiColon,
        V::Apostrophe => G::Apostrophe,
        V::Return => G::Enter,
        V::LShift => G::LShift,
        V::Z => G::Z,
        V::X => G::X,
        V::C => G::C,
        V::V => G::V,
        V::B => G::B,
        V::N => G::N,
        V::M => G::M,
        V::Comma => G::Comma,
        V::Period => G::Period,
        V::Slash => G::ForwardSlash,
        V::RShift => G::RShift,
        V::LControl => G::LCtrl,
        V::LWin => G::LWin,
        V::LAlt => G::LAlt,
        V::Space => G::Space,
        V::RAlt => G::RAlt,
        V::RWin => G::RWin,
        V::RControl => G::RCtrl,
        V::Snapshot => G::PrintScreen,
        V::Scroll => G::ScrollLock,
        V::Pause => G::Pause,
        V::Insert => G::Insert,
        V::Home => G::Home,
        V::PageUp => G::PageUp,
        V::Delete => G::Delete,
        V::End => G::End,
        V::PageDown => G::PageDown,
        V::Up => G::Up,
        V::Left => G::Left,
        V::Down => G::Down,
        V::Right => G::Right,
        V::Numlock => G::NumLock,
        V::NumpadDivide => G::NumDivide,
        V::NumpadMultiply => G::NumMultiply,
        V::NumpadSubtract => G::NumSubtract,
        V::Numpad7 => G::Num7,
        V::Numpad8 => G::Num8,
        V::Numpad9 => G::Num9,
        V::NumpadAdd => G::NumAdd,
        V::Numpad4 => G::Num4,
        V::Numpad5 => G::Num5,
        V::Numpad6 => G::Num6,
        V::Numpad1 => G::Num1,
        V::Numpad2 => G::Num2,
        V::Numpad3 => G::Num3,
        V::NumpadEnter => G::NumEnter,
        V::Numpad0 => G::Num0,
        V::NumpadDecimal => G::NumDecimal,
        _ => G::NotImplemented,
    };

    key
}
