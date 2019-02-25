struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn a() {
    let mut user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let user3 = User {
        username: String::from("anotherusername5678"),
        email: String::from("another1@example.com"),
        ..user1
    };
}

fn b() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// An Example Program Using Structs

fn area_a(width: u32, height: u32) -> u32 {
    width * height
}

fn area_b(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area_c(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[cfg(test)]
mod tests {
    use crate::structs::{self, Rectangle};

    #[test]
    fn test_area_a() {
        let width = 30;
        let height = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            structs::area_a(width, height)
        );

        assert_eq!(1500, structs::area_a(width, height));
    }

    #[test]
    fn test_area_b() {
        let rect = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            structs::area_b(rect)
        );

        assert_eq!(1500, structs::area_b(rect));
    }

    #[test]
    fn test_area_c() {
        let rect = Rectangle { width: 30, height: 50 };

        // rect1 is Rectangle { width: 30, height: 50 }
        println!("rect is {:?}", rect);

        // rect1 is Rectangle {
        //    width: 30,
        //    height: 50
        // }
        println!("rect is {:#?}", rect);

        println!(
            "The area of the rectangle is {} square pixels.",
            structs::area_c(&rect)
        );

        assert_eq!(1500, structs::area_c(&rect));
    }

    #[test]
    fn test_rectangle() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        assert_eq!(1500, rect1.area());
        assert_eq!(true, rect1.can_hold(&rect2));
        assert_eq!(false, rect1.can_hold(&rect3));
    }
}