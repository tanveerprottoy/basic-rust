pub fn basic() {
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

pub fn tuple() {
    let t: &(i32, f32) = &(18, 22.06);
    println!("{:?}", t);
}
