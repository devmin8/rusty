use std::io;

const NAME: &str = "shan";

fn city() -> String {
    String::from("toronto")
}

// () -> this is unit and it is returned when nothing is returned here.
// basically tuple with no values
fn main() {
    let messages = "hello";

    println!("{messages}");

    let mut messages = String::new();

    io::stdin()
        .read_line(&mut messages)
        .expect("Failed to read line");

    println!("{messages}");

    println!("{NAME}");

    let guess: u64 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let decimal = 10_000;
    println!("{decimal}");

    let x = 0.1 + 0.2;
    println!("precision = {}", x);

    let tuple = (10, "my tuple", 'c');
    let first_arg = tuple.0;

    println!("{first_arg}");

    let mut a = [1, 2, 3, 4, 5];
    let b = [1, 2, 3, 4, 5];

    a[4] = 999;

    let array = [a, b].concat();
    // `{:?}` (or {:#?}
    println!("{:?}", array);
    println!("pretty print {:#?}", array);

    println!("city = {:?}, array = {:?}", city(), array);

    let num = 10;

    if num < 5 {
        println!("number < 5")
    }

    let condition = false;
    let new_number = if condition { 10 } else { 20 };

    println!("{:?}", new_number);
}
