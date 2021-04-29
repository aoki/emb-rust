struct Sensor {
    active: bool,
    latest: u32,
}

impl Sensor {
    fn read(&self) -> u32 {
        self.latest
    }

    fn init(&mut self) {
        self.active = true;
        self.latest = 42;
    }

    fn new() -> Sensor {
        Sensor {
            active: false,
            latest: 123,
        }
    }
}

enum Type {
    Int,
    Float,
    Boolean,
}

fn print_type(t: Type) {
    match t {
        Type::Int => println!("Type is integer"),
        Type::Float => println!("Type is floating point"),
        Type::Boolean => println!("Type is boolean"),
    }
}

fn double(number: u32) -> Option<u32> {
    if number > (u32::MAX / 2) {
        return None;
    }
    Some(number * 2)
}

fn use_sensor(s: &mut Sensor) {
    println!("latest = {}", s.latest);
    s.latest = s.latest + 100;
}

fn main() {
    let mut sens = Sensor::new();
    println!("latest = {}", sens.latest);
    use_sensor(&mut sens);
    println!("latest = {}", sens.latest);

    let doubled = double(10).unwrap();
    println!("doubled = {}", doubled);
    match double(u32::MAX / 2 + 1) {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }

    if let Some(x) = double(u32::MAX) {
        println!("double = {}", x);
    }

    print_type(Type::Int);
    print_type(Type::Float);
    print_type(Type::Boolean);

    let sensor = Sensor {
        active: false,
        latest: 0,
    };
    println!("active = {}, latest = {}", sensor.active, sensor.latest);

    let mut sensor2 = Sensor {
        active: false,
        latest: 0,
    };
    sensor2.latest = 42;
    println!("active = {}, latest = {}", sensor2.active, sensor2.latest);

    let mut sensor3 = Sensor {
        active: false,
        latest: 0,
    };

    sensor3.init();
    let latest = sensor3.read();
    println!("latest = {}", latest);

    let sensor4 = Sensor::new();
    println!("latest = {}", sensor4.latest);
}
