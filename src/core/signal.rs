use crate::core::Shared;
use crate::core::{Slot, SlotFunc, SlotMethod};

pub struct Signal<Arg: 'static> {
    slots: Vec<Box<dyn Slot<Arg>>>,
}

impl<Arg: 'static> Default for Signal<Arg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Arg: 'static> Signal<Arg> {
    pub fn new() -> Self {
        Self { slots: vec![] }
    }

    pub fn emit(&self, arg: &Arg) {
        for slot in self.slots.iter() {
            slot.call(arg);
        }
    }

    pub fn connect_func<F: Fn(&Arg) + 'static>(&mut self, callback: F) {
        let slot = SlotFunc::new(callback);
        self.slots.push(Box::new(slot));
    }

    pub fn connect_method<Receiver: 'static, F: Fn(&mut Receiver, &Arg) + 'static>(
        &mut self,
        receiver: Shared<Receiver>,
        callback: F,
    ) {
        let slot = SlotMethod::new(receiver, callback);
        self.slots.push(Box::new(slot));
    }
}
