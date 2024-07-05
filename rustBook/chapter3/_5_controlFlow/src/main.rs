fn main() {
    let number = 1;

    if number < 5 {
        println!("bad");
    } else {
        println!("Good");
    }

    let condition = true;
    let number = if condition { 5 } else { 4 };

    println!("{}", number);
}
