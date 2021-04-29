fn main() {
    println!("Hello, world!");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);

    let x = 42;
    println!("x = {}", x);
    let mut y = 24;
    println!("y = {}", y);
    y = 42;
    println!("y = {}", y);

    const FOURTY_TWO: i32 = 42;
    println!("const FOURTY_TWO = {}", FOURTY_TWO);

    static mut X: i32 = 42;
    unsafe {
        println!("static mut = {}", X);
    }
}
