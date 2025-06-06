use sycamore::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HideContext(Signal<bool>);

impl HideContext {
    pub fn new() -> Self {
        Self(create_signal(true))
    }

    pub fn get(self) -> bool {
        self.0.get()
    }

    pub fn show(self) {
        self.0.set(false);
    }

    pub fn hide(self) {
        self.0.set(true);
    }
}
