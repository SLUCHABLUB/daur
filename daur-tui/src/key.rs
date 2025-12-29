use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::event::MediaKeyCode;
use crossterm::event::ModifierKeyCode;
use serde::Deserialize;
use serde::de::Error;
use std::borrow::Cow;
use std::str::FromStr as _;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct Key {
    modifiers: KeyModifiers,
    code: KeyCode,
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        Key {
            modifiers: event.modifiers,
            code: event.code,
        }
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fn parse_code(string: &str) -> Option<KeyCode> {
            if let Ok(character) = char::from_str(string) {
                return Some(KeyCode::Char(character));
            }

            if let Some(function) = string.strip_prefix("f")
                && let Ok(number) = u8::from_str(function)
            {
                return Some(KeyCode::F(number));
            }

            Some(match string {
                "backspace" => KeyCode::Backspace,
                "enter" => KeyCode::Enter,
                "left" => KeyCode::Left,
                "right" => KeyCode::Right,
                "up" => KeyCode::Up,
                "down" => KeyCode::Down,
                "home" => KeyCode::Home,
                "end" => KeyCode::End,
                "page_up" => KeyCode::PageUp,
                "page_down" => KeyCode::PageDown,
                "tab" => KeyCode::Tab,
                "back_tab" => KeyCode::BackTab,
                "delete" => KeyCode::Delete,
                "insert" => KeyCode::Insert,
                "null" => KeyCode::Null,
                "esc" => KeyCode::Esc,
                "caps_lock" => KeyCode::CapsLock,
                "scroll_lock" => KeyCode::ScrollLock,
                "num_lock" => KeyCode::NumLock,
                "print_screen" => KeyCode::PrintScreen,
                // TODO: handle the distinction between PAUSE and MEDIA_PAUSE
                // "pause" => KeyCode::Pause,
                "menu" => KeyCode::Menu,
                "keypad_begin" => KeyCode::KeypadBegin,

                // Added for convince.
                "space" => KeyCode::Char(' '),

                "play" => KeyCode::Media(MediaKeyCode::Play),
                "pause" => KeyCode::Media(MediaKeyCode::Pause),
                "play_pause" => KeyCode::Media(MediaKeyCode::PlayPause),
                "reverse" => KeyCode::Media(MediaKeyCode::Reverse),
                "stop" => KeyCode::Media(MediaKeyCode::Stop),
                "fast_forward" => KeyCode::Media(MediaKeyCode::FastForward),
                "rewind" => KeyCode::Media(MediaKeyCode::Rewind),
                "track_next" => KeyCode::Media(MediaKeyCode::TrackNext),
                "track_previous" => KeyCode::Media(MediaKeyCode::TrackPrevious),
                "record" => KeyCode::Media(MediaKeyCode::Record),
                "lower_volume" => KeyCode::Media(MediaKeyCode::LowerVolume),
                "raise_volume" => KeyCode::Media(MediaKeyCode::RaiseVolume),
                "mute_volume" => KeyCode::Media(MediaKeyCode::MuteVolume),

                "left_shift" => KeyCode::Modifier(ModifierKeyCode::LeftShift),
                "left_control" => KeyCode::Modifier(ModifierKeyCode::LeftControl),
                "left_alt" => KeyCode::Modifier(ModifierKeyCode::LeftAlt),
                "left_super" => KeyCode::Modifier(ModifierKeyCode::LeftSuper),
                "left_hyper" => KeyCode::Modifier(ModifierKeyCode::LeftHyper),
                "left_meta" => KeyCode::Modifier(ModifierKeyCode::LeftMeta),
                "right_shift" => KeyCode::Modifier(ModifierKeyCode::RightShift),
                "right_control" => KeyCode::Modifier(ModifierKeyCode::RightControl),
                "right_alt" => KeyCode::Modifier(ModifierKeyCode::RightAlt),
                "right_super" => KeyCode::Modifier(ModifierKeyCode::RightSuper),
                "right_hyper" => KeyCode::Modifier(ModifierKeyCode::RightHyper),
                "right_meta" => KeyCode::Modifier(ModifierKeyCode::RightMeta),
                "iso_level_3_shift" => KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift),
                "iso_level_5_shift" => KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift),

                _ => return None,
            })
        }

        fn parse_modifier(string: &str) -> Option<KeyModifiers> {
            Some(match string {
                "shift" => KeyModifiers::SHIFT,
                "control" => KeyModifiers::CONTROL,
                "alt" => KeyModifiers::ALT,
                "super" => KeyModifiers::SUPER,
                "hyper" => KeyModifiers::HYPER,
                "meta" => KeyModifiers::META,

                _ => return None,
            })
        }

        fn invalid_key<E: Error>(key: &str) -> E {
            Error::custom(format!("invalid key: `{key}`"))
        }

        let string = Cow::<str>::deserialize(deserializer)?;
        let mut string: &str = &string;

        let mut modifiers = KeyModifiers::empty();

        Ok(loop {
            if let Some(code) = parse_code(string) {
                break Key { modifiers, code };
            }

            let Some((prefix, suffix)) = string.split_once('_') else {
                return Err(invalid_key(string));
            };

            let Some(modifier) = parse_modifier(prefix) else {
                return Err(invalid_key(string));
            };

            modifiers |= modifier;
            string = suffix;
        })
    }
}
