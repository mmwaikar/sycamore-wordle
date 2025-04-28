use super::keyboard_alphabet::KeyboardAlphabet;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Keyboard {
    pub keyboard: [KeyboardAlphabet; 26],
}

impl Keyboard {
    const FIRST_ROW: [char; 10] = ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
    const SECOND_ROW: [char; 9] = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
    const THIRD_ROW: [char; 7] = ['z', 'x', 'c', 'v', 'b', 'n', 'm'];

    pub fn new() -> Self {
        let keyboard = Self::get_keyboard();
        Self { keyboard }
    }

    fn get_keyboard() -> [KeyboardAlphabet; 26] {
        let mut keyboard = [KeyboardAlphabet::init(); 26];
        let mut i = 0;

        for c in Self::FIRST_ROW {
            keyboard[i] = KeyboardAlphabet::new(c, 1);
            i += 1;
        }
        for c in Self::SECOND_ROW {
            keyboard[i] = KeyboardAlphabet::new(c, 2);
            i += 1;
        }
        for c in Self::THIRD_ROW {
            keyboard[i] = KeyboardAlphabet::new(c, 3);
            i += 1;
        }
        keyboard
    }
}