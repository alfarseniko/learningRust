fn main() {
    //let guess = "42".parse().expect("Not a number!");
    //^^^ the above line is incorrect as the type is not provided to the variable
    // here u32 means unsigned 32-bit integer

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    /**
     SCALAR TYPES
        represent a single value
    1. Integer Type
            a number without a fractional component
            ref: two's complement representation
            unsigned integers can store variables from 0 to (2^8 -1)
            signed integers can store variables from -(2^7) to (2^7 -1)
            one bit is used for storing the positive or negative sign

    2. Floating Type

     * Basic Single floating point f32
     * And double floating point
     */
    let x = 4.0; // f64
    let y: f32 = 3.2; //f32

    println!("This is f64 = {} and this is f 32 = {}", x, y);

    /*
    3. Boolean Type
     */

    let bool = true;
    let ybool: bool = false;
    println!("{}{}", bool, ybool);

    /*
    4. Tuples
        A collection of different data types
     */

    let tup: (u8, i32, char) = (5, 5, 'c');
    let (a, b, c) = tup; // this step is called destructing

    println!("{}", a);
    println!("{}", tup.2);

    /*
    5. Arrays
    There are no dynamic arrays in Rust
    Once an array has been declared, its size will remain the same
     */

    let array1 = [1, 2, 3, 4, 5, 6];
    println!("{}", array1[3]);
}
