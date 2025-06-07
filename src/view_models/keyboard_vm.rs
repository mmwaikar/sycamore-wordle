use super::keyboard_alphabet_vm::KeyboardAlphabetVM;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeyboardVM {
    pub keyboard: [KeyboardAlphabetVM; 26],
}

impl KeyboardVM {
    const FIRST_ROW: [char; 10] = ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
    const SECOND_ROW: [char; 9] = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
    const THIRD_ROW: [char; 7] = ['z', 'x', 'c', 'v', 'b', 'n', 'm'];

    pub fn new() -> Self {
        let keyboard = Self::init();
        Self { keyboard }
    }

    fn init() -> [KeyboardAlphabetVM; 26] {
        let mut keyboard_vm = [KeyboardAlphabetVM::init(); 26];
        let mut i = 0;

        for c in Self::FIRST_ROW {
            keyboard_vm[i] = KeyboardAlphabetVM::new(c, 1);
            i += 1;
        }
        for c in Self::SECOND_ROW {
            keyboard_vm[i] = KeyboardAlphabetVM::new(c, 2);
            i += 1;
        }
        for c in Self::THIRD_ROW {
            keyboard_vm[i] = KeyboardAlphabetVM::new(c, 3);
            i += 1;
        }
        keyboard_vm
    }
}