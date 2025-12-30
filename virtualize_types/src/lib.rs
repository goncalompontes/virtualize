#![feature(associated_type_defaults)]
#![feature(min_specialization)]

mod first;
mod second;
mod third;

pub struct Noop;

impl Vtable for Noop {
    type Class = Noop;
}

impl Class for Noop {
    type Repr = ();
    type Vtable = Noop;
}

pub trait Class {
    type Repr;
    type Vtable: Vtable = Noop;
}

pub trait Vtable {
    type Class: Class;
}