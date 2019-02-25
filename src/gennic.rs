struct PointA<T> {
    x: T,
    y: T,
}

impl<T> PointA<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl PointA<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointB<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> PointB<T, U> {
    pub fn mixup<V, W>(self, other: PointB<V, W>) -> PointB<T, W> {
        PointB {
            x: self.x,
            y: other.y,
        }
    }
}

fn a() {
    let integer = PointA { x: 5, y: 10 };
    let flot = PointA { x: 1.0, y: 4.0 };

    println!("p.x = {}", integer.x());
}

fn b() {
    let both_integer = PointB { x: 5, y: 10 };
    let both_float = PointB { x: 1.0, y: 4.0 };
    let integer_and_float = PointB { x: 5, y: 4.0 };
}

#[cfg(test)]
mod tests {
    use crate::gennic::PointB;

    #[test]
    fn a() {
        let p1 = PointB { x: 5, y: 10.4 };
        let p2 = PointB { x: "Hello", y: 'c'};

        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

        assert_eq!(5, p3.x);
        assert_eq!('c', p3.y);
    }
}