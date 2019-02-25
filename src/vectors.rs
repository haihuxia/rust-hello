fn a() {
    let v: Vec<i32> = Vec::new();

    let a = vec![1, 2, 3];

    let mut b = Vec::new();
    b.push(5);
    b.push(6);
    b.push(7);
    b.push(8);

    let six = &b[1];
    match b.get(1) {
        Some(six) => println!("The third element is {}", six),
        None => println!("There is no third element."),
    }
}

fn b() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn c() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn d() {
    enum SpreadsheetCell {
        Int(i32),
        Flot(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Flot(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}