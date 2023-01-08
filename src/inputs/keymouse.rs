use crate::components::*;
use bevy::{input::{keyboard::KeyboardInput, ButtonState}, prelude::*};

pub fn keymouse_inputs(
    key_input: Res<Input<ScanCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut key_evr: EventReader<KeyboardInput>,
) {

    if key_input.pressed(ScanCode(0)) {
        info!("{:?}", KeyCode::Escape)
    }

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyMouse {
    Key(KeyCode),
    Mouse(MouseButton),
    ScanKey(ScanCode),
}



// impl KeyMouse {
//     /// Returns key description (e.g Left Shift)
//     pub fn display_string(&self) -> String {
//         use self::KeyMouse::*;
//         use winit::event::{MouseButton, VirtualKeyCode::*};
//         let key_string = match self {
//             Key(Key1) => "1",
//             Key(Key2) => "2",
//             Key(Key3) => "3",
//             Key(Key4) => "4",
//             Key(Key5) => "5",
//             Key(Key6) => "6",
//             Key(Key7) => "7",
//             Key(Key8) => "8",
//             Key(Key9) => "9",
//             Key(Key0) => "0",
//             Key(A) => "A",
//             Key(B) => "B",
//             Key(C) => "C",
//             Key(D) => "D",
//             Key(E) => "E",
//             Key(F) => "F",
//             Key(G) => "G",
//             Key(H) => "H",
//             Key(I) => "I",
//             Key(J) => "J",
//             Key(K) => "K",
//             Key(L) => "L",
//             Key(M) => "M",
//             Key(N) => "N",
//             Key(O) => "O",
//             Key(P) => "P",
//             Key(Q) => "Q",
//             Key(R) => "R",
//             Key(S) => "S",
//             Key(T) => "T",
//             Key(U) => "U",
//             Key(V) => "V",
//             Key(W) => "W",
//             Key(X) => "X",
//             Key(Y) => "Y",
//             Key(Z) => "Z",
//             Key(Escape) => "ESC",
//             Key(F1) => "F1",
//             Key(F2) => "F2",
//             Key(F3) => "F3",
//             Key(F4) => "F4",
//             Key(F5) => "F5",
//             Key(F6) => "F6",
//             Key(F7) => "F7",
//             Key(F8) => "F8",
//             Key(F9) => "F9",
//             Key(F10) => "F10",
//             Key(F11) => "F11",
//             Key(F12) => "F12",
//             Key(F13) => "F13",
//             Key(F14) => "F14",
//             Key(F15) => "F15",
//             Key(F16) => "F16",
//             Key(F17) => "F17",
//             Key(F18) => "F18",
//             Key(F19) => "F19",
//             Key(F20) => "F20",
//             Key(F21) => "F21",
//             Key(F22) => "F22",
//             Key(F23) => "F23",
//             Key(F24) => "F24",
//             Key(Snapshot) => "Print Screen",
//             Key(Scroll) => "Scroll Lock",
//             Key(Pause) => "Pause/Break",
//             Key(Insert) => "Insert",
//             Key(Home) => "Home",
//             Key(Delete) => "Delete",
//             Key(End) => "End",
//             Key(PageDown) => "PageDown",
//             Key(PageUp) => "PageUp",
//             Key(Left) => "Left Arrow",
//             Key(Up) => "Up Arrow",
//             Key(Right) => "Right Arrow",
//             Key(Down) => "Down Arrow",
//             Key(Back) => "Backspace",
//             Key(Return) => "Enter",
//             Key(Space) => "Space",
//             Key(Compose) => "Compose",
//             Key(Caret) => "^",
//             Key(Numlock) => "Numlock",
//             Key(Numpad0) => "Numpad 0",
//             Key(Numpad1) => "Numpad 1",
//             Key(Numpad2) => "Numpad 2",
//             Key(Numpad3) => "Numpad 3",
//             Key(Numpad4) => "Numpad 4",
//             Key(Numpad5) => "Numpad 5",
//             Key(Numpad6) => "Numpad 6",
//             Key(Numpad7) => "Numpad 7",
//             Key(Numpad8) => "Numpad 8",
//             Key(Numpad9) => "Numpad 9",
//             Key(AbntC1) => "Abnt C1",
//             Key(AbntC2) => "Abnt C2",
//             Key(NumpadAdd) => "Numpad +",
//             Key(Apostrophe) => "'",
//             Key(Apps) => "Context Menu",
//             Key(At) => "@",
//             Key(Ax) => "Ax",
//             Key(Backslash) => "\\",
//             Key(Calculator) => "Calculator",
//             Key(Capital) => "Caps Lock",
//             Key(Colon) => ":",
//             Key(Comma) => ",",
//             Key(Convert) => "Convert",
//             Key(NumpadDecimal) => "Numpad .",
//             Key(NumpadDivide) => "Numpad /",
//             Key(Equals) => "=",
//             Key(Grave) => "`",
//             Key(Kana) => "Kana",
//             Key(Kanji) => "Kanji",
//             Key(LBracket) => "[",
//             Key(RBracket) => "]",
//             Key(Mail) => "Mail",
//             Key(MediaSelect) => "MediaSelect",
//             Key(MediaStop) => "MediaStop",
//             Key(Minus) => "-",
//             Key(Plus) => "+",
//             Key(NumpadMultiply) => "Numpad *",
//             Key(Mute) => "Mute",
//             Key(MyComputer) => "My Computer",
//             Key(NavigateBackward) => "Navigate Backward",
//             Key(NavigateForward) => "Navigate Forward",
//             Key(NoConvert) => "Non Convert",
//             Key(NumpadComma) => "Num ,",
//             Key(NumpadEnter) => "Num Enter",
//             Key(NumpadEquals) => "Num =",
//             Key(OEM102) => "<",
//             Key(Period) => ".",
//             Key(Power) => "Power",
//             Key(PlayPause) => "Play / Pause",
//             Key(PrevTrack) => "Prev Track",
//             Key(NextTrack) => "Next Track",
//             Key(LAlt) => {
//                 if cfg!(macos) {
//                     "Left Option ⌥"
//                 } else {
//                     // Assume Windows, Linux, BSD, etc.
//                     "Left Alt"
//                 }
//             },
//             Key(RAlt) => {
//                 if cfg!(macos) {
//                     "Right Option ⌥"
//                 } else {
//                     // Assume Windows, Linux, BSD, etc.
//                     "Right Alt"
//                 }
//             },
//             Key(LControl) => "Left Ctrl"
//             Key(RControl) => {
//                 if cfg!(macos) {
//                     "Right Cmd ⌘"
//                 } else {
//                     // Assume Windows, Linux, BSD, etc.
//                     "Right Ctrl"
//                 }
//             },
//             Key(LShift) => "Left Shift",
//             Key(RShift) => "Right Shift",
//             // Key doesn't usually have a right counterpart on modern keyboards, to omit the
//             // qualifier. The exception to this is Mac OS which doesn't usually have
//             // this key at all, so we keep the qualifier to minimise ambiguity.
//             Key(LWin) => {
//                 if cfg!(windows) {
//                     "Win ⊞"
//                 } else if cfg!(macos) {
//                     "Left Cmd ⌘ (Super)" // Extra qualifier because both Ctrl and Win map to Cmd on Mac
//                 } else {
//                     // Assume Linux, BSD, etc.
//                     "Super"
//                 }
//             },
//             // Most keyboards don't have this key, so throw in all the qualifiers
//             Key(RWin) => {
//                 if cfg!(windows) {
//                     "Right Win ⊞"
//                 } else if cfg!(macos) {
//                     "Right Cmd ⌘ (Super)" // Extra qualifier because both Ctrl and Win map to Cmd on Mac
//                 } else {
//                     // Assume Linux, BSD, etc.
//                     "Right Super"
//                 }
//             },
//             Key(Semicolon) => ";",
//             Key(Slash) => "/",
//             Key(Sleep) => "Sleep",
//             Key(Stop) => "Media Stop",
//             Key(NumpadSubtract) => "Num -",
//             Key(Sysrq) => "Sysrq",
//             Key(Tab) => "Tab",
//             Key(Underline) => "_",
//             Key(Unlabeled) => "No Name",
//             Key(VolumeDown) => "Volume Down",
//             Key(VolumeUp) => "Volume Up",
//             Key(Wake) => "Wake",
//             Key(WebBack) => "Browser Back",
//             Key(WebFavorites) => "Browser Favorites",
//             Key(WebForward) => "Browser Forward",
//             Key(WebHome) => "Browser Home",
//             Key(WebRefresh) => "Browser Refresh",
//             Key(WebSearch) => "Browser Search",
//             Key(WebStop) => "Browser Stop",
//             Key(Yen) => "Yen",
//             Key(Copy) => "Copy",
//             Key(Paste) => "Paste",
//             Key(Cut) => "Cut",
//             Key(Asterisk) => "*",
//             Mouse(MouseButton::Left) => "Left Click",
//             Mouse(MouseButton::Right) => "Right Click",
//             Mouse(MouseButton::Middle) => "Middle Click",
//             Mouse(MouseButton::Other(button)) => {
//                 // Additional mouse buttons after middle click start at 1
//                 return format!("Mouse {}", button + 3);
//             },
//             ScanKey(scancode) => {
//                 return if let Some(layout) = key_layout {
//                     layout.get_key_as_string(*scancode)
//                 } else {
//                     format!("Unknown (0x{:X})", scancode)
//                 };
//             },
//         };

//         key_string.to_owned()
//     }
// }
