fn basic_print() {
    let x: i32 = 128;
    let y = 256;
    let mut z: f32 = 512.12;
    let s = "str";
    println!("Hello, world!");
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    println!("{}", s);

    z = 1024.24;
    println!("{}", z);
}

fn main() {
    basic_print()
}
