fn main() {
    //_loop();
    //_while();
    //_for();
    _forr();
}

fn _loop() {
    loop {
        println!("Hello, world!");
    }
}

fn _while() {
    let mut number = 5;
    while number != 0 {
        println!("Yo wassup");
        number = number - 1;
    }
}

fn _for() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for blah in a.iter() {
        println!("{}", blah);
    }
}

fn _forr() {
    for numb in 1..5 {
        println!("{}", numb);
    }
}
