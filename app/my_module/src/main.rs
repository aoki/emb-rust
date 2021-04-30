mod new_module;

#[derive(Debug)]
struct Sensor {
    active: bool,
    latest: u32,
}

fn main() {
    let s = Sensor {
        active: true,
        latest: 42,
    };

    println!("{:?}", s);
    new_module::public_func();
    println!("Hello, world!");
}
