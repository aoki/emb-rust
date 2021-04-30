use oorandom::Rand32;

fn main() {
    let seed = 1;
    let mut rng = Rand32::new(seed);
    println!("my number = {}", rng.rand_u32());
    my_lib::print_my_number(rng.rand_u32());
    println!("Hello, world!");
}
