fn main() {
    // it must be an immutable variable to be shadowed
    let c = 5;

    println!("The value of c is: {}", c);

    // we cannot use mut with a variable to shadow it later
    let c = c + 5;

    println!("The value of c is: {}", c);
}
