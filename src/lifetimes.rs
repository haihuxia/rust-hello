// The first rule is that each parameter that is a reference gets its own lifetime parameter.
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32);

// The second rule is if there is exactly one input lifetime parameter,
// that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32

// The third rule is if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn a() {
    // 'static denotes the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
}