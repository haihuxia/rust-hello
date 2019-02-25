fn a() {
    let data = "initial content";

    let s = data.to_string();
    let s = "initial content".to_string();
    let s = String::from("initial content");

    let mut a = String::from("foo");
    a.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

fn b() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
}

fn c() {
    let s = String::from("hello world");

    for c in s.chars() {
        println!("{}", c);
    }

    for b in s.bytes() {
        println!("{}", b);
    }
}