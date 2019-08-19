#[cfg(test)]
mod tests {

    #[test]
    fn a() {
        let x = 1;

        match x {
            1 => println!("one"), // OK
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn b() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y), // OK, Some(y) is new y
            _ => println!("Default case, x = {:?}", x),
        }
    }

    #[test]
    fn c() {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"), // OK
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn d() {
        let x = 5;

        match x {
            1...5 => println!("one through five"), // OK
            _ => println!("something else"),
        }
    }

    #[test]
    fn e() {
        let x = 'c';

        match x {
            'a'...'j' => println!("early ASCII letter"), // OK
            'k'...'z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    #[test]
    fn f() {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }

    #[test]
    fn g() {
        struct Point {
            x: i32,
            y: i32,
        }

        fn main() {
            let p = Point { x: 0, y: 7 };

            let Point { x, y } = p;
            assert_eq!(0, x);
            assert_eq!(7, y);
        }
    }

    #[test]
    fn h() {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y), // OK
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }

    #[test]
    fn i() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            },
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x,
                    y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                )
            } // OK
        }
    }

    #[test]
    fn j() {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                )
            },
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    h,
                    s,
                    v
                )
            } // OK
            _ => ()
        }
    }

    #[test]
    fn k() {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);
    }

    #[test]
    fn l() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            } // OK
            _ => {
                setting_value = new_setting_value;
            }
        }
    }

    #[test]
    fn m() {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            },
        }
    }

    #[test]
    fn n() {
        let _x = 5; // not warning
        let y = 10;
    }

    #[test]
    fn o() {
        let s = Some(String::from("Hello!"));

        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }

    #[test]
    fn p() {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }

    #[test]
    fn q() {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            },
        }
    }

    #[test]
    fn r() {
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x), // OK
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    #[test]
    fn s() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x), // OK
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    #[test]
    fn t() {
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"), // OK
        }
    }

    #[test]
    fn u() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id_variable @ 3...7 } => {
                println!("Found an id in range: {}", id_variable)
            }, // OK
            Message::Hello { id: 10...12 } => {
                println!("Found an id in another range")
            },
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            },
        }
    }
}