
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