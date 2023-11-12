trait Orange {}

struct GrapeFruit {}

impl Orange for GrapeFruit {}

#[derive(Debug)]
struct FooBar<T> {
    origin: T,
}

impl FooBar<()> {
    fn from_nowhere() -> Self {
        Self { origin: () }
    }
}

impl<'a, F> FooBar<&'a mut F>
where
    F: Orange,
{
    fn from_orange(orange: &'a mut F) -> Self {
        FooBar { origin: orange }
    }
}

impl<F> FooBar<F>
where
    F: FnMut(u32) -> u32,
{
    fn from_callback(cb: F) -> Self {
        FooBar { origin: cb }
    }
}

impl FooBar<Box<dyn Orange>> {
    fn from_orange(orange: Box<dyn Orange>) -> Self {
        Self { origin: orange }
    }
}
