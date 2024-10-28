fn five() -> i32 {
    //  Expressions do not include ending semicolons. If you add a semicolon to the end of an expression,
    // you turn it into a statement
    // Functions can return values to the code that calls them. We don’t name return values,
    // but we must declare their type after an arrow (->). In Rust, the return value of the function is
    // synonymous with the value of the final expression in the block of the body of a function.
    // You can return early from a function by using the return keyword and specifying a value,
    // but most functions return the last expression implicitly
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn functions() {
    let x = five();

    println!("The value of x is: {x}");

    let y = plus_one(5);

    println!("The value of y is: {y}");
}
