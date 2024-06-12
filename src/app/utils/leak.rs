pub trait Leak {
    fn leak(self) -> &'static Self;
}

impl<T> Leak for T {
    fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }
}
