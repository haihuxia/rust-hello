use std::ops::Deref;

fn a() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y)
}

fn b() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::deref::MyBox;

    #[test]
    fn a() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        // *(y.deref())
        assert_eq!(5, *y);
    }
}