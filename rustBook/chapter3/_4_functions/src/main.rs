fn main() {
    let x = 5;
    let y = {
        let a = 4;
        a + 3
    };
    println!("{}", y);

    println!("{}", five());
}

// fn another_fn(x: i32) {
//     println!("Another function gives value {}", x);
// }

/*
    Statements do not return a value and hence cannot be used as an argument to a function
*/

fn five() -> i64 {
    542534534
}
