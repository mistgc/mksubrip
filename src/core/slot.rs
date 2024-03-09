// pub use slot::{Slot, SlotFunc, SlotMethod};
use crate::core::Shared;

pub trait Slot<Arg> {
    fn call(&self, arg: &Arg);
}

type Callback<Arg> = Box<dyn Fn(&Arg)>;
type Method<Receiver, Arg> = Box<dyn Fn(&mut Receiver, &Arg)>;

pub struct SlotFunc<Arg> {
    callback: Callback<Arg>,
}

impl<Arg> SlotFunc<Arg> {
    pub fn new<F: Fn(&Arg) + 'static>(callback: F) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl<Arg> Slot<Arg> for SlotFunc<Arg> {
    fn call(&self, arg: &Arg) {
        (self.callback)(arg);
    }
}

pub struct SlotMethod<Receiver, Arg> {
    receiver: Shared<Receiver>,
    callback: Method<Receiver, Arg>,
}

impl<Receiver, Arg> SlotMethod<Receiver, Arg> {
    pub fn new<F: Fn(&mut Receiver, &Arg) + 'static>(
        receiver: Shared<Receiver>,
        callback: F,
    ) -> Self {
        Self {
            receiver,
            callback: Box::new(callback),
        }
    }
}

impl<Receiver, Arg> Slot<Arg> for SlotMethod<Receiver, Arg> {
    fn call(&self, arg: &Arg) {
        (self.callback)(&mut *self.receiver.borrow_mut(), arg);
    }
}
