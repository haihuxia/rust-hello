mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::breathe_in();
        }
    }

    fn breathe_in() {}

    mod a {
        use crate::modules::sound::instrument;

        fn fna() {
            instrument::clarinet();
            instrument::clarinet();
            instrument::clarinet();
        }
    }

    mod b {
        mod modc {
            pub mod modd {
                pub fn clarinet() {}
            }
        }

        use self::modc::modd;

        fn fna() {
            modd::clarinet();
            modd::clarinet();
            modd::clarinet();
        }
    }

    mod c {
        use crate::modules::sound::instrument::clarinet;

        fn fna() {
            clarinet();
            clarinet();
            clarinet();
        }
    }
}

fn a() {
    // Absolute path
    crate::modules::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a() {
        let mut v = super::plant::Vegetable::new("squash");
        v.name = String::from("butternut squash");
        println!("{} are delicious", v.name);
    }
}