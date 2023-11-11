#![allow(unused)]

// DI Container

use std::marker::PhantomData;

pub trait Register<T: ?Sized, U> {
    fn resolve(&self) -> U;
}

// pub struct PhantomData<T: ?Sized>;

struct Container<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> Container<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

trait Speak {
    fn speak(&self) {
        println!("speak speak speak");
    }
}

trait Read {
    fn read(&self) {
        println!("read read read")
    }
}

impl Register<dyn Speak, ConcreteA> for Container<dyn Speak> {
    fn resolve(&self) -> ConcreteA {
        ConcreteA
    }
}

impl Register<dyn Read, ConcreteB> for Container<dyn Read> {
    fn resolve(&self) -> ConcreteB {
        ConcreteB
    }
}

struct ConcreteA;
impl Speak for ConcreteA {}

struct ConcreteB;
impl Read for ConcreteB {}

fn speaker(s: impl Speak) {
    s.speak()
}

fn reader(r: impl Read) {
    r.read()
}

// --

trait ComplexTrait<T: Read> {
    fn do_something(&self);
}

struct Concrete<T> {
    dep: T,
}

impl<T: Read> ComplexTrait<T> for Concrete<T> {
    fn do_something(&self) {
        self.dep.read()
    }
}

// draw graph:

impl Register<dyn ComplexTrait<dyn Read>, Concrete<ConcreteB>>
    for Container<dyn ComplexTrait<dyn Read>>
{
    fn resolve(&self) -> Concrete<ConcreteB> {
        Concrete {
            dep: Container::<dyn Read>::new().resolve(),
        }
    }
}

impl Register<dyn ComplexTrait<dyn Speak>, Concrete<ConcreteA>>
    for Container<dyn ComplexTrait<dyn Speak>>
{
    fn resolve(&self) -> Concrete<ConcreteA> {
        Concrete {
            dep: Container::<dyn Speak>::new().resolve(),
        }
    }
}

fn complex_reader<T: Read>(r: impl ComplexTrait<T>) {
    r.do_something()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing() {
        let a = ConcreteA;

        // let container = Container::<dyn Speak>;
        let container = Container::<dyn Speak>::new();

        speaker(container.resolve()); // works fine
                                      // reader(container.resolve()); // not working as it wasn't for dyn read

        complex_reader(Container::<dyn ComplexTrait<dyn Read>>::new().resolve());

        // complex_reader(Container::<dyn ComplexTrait<dyn Speak>>::new().resolve());
    }
}
