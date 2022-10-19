// Traditional struct
struct HSL {
    hue: u8,
    saturation: u8,
    lightness: u8,
}

// Tuple struct
struct HSLa(u8, u8, u8, u8);

struct Person {
    name: String,
    age: u16,
}

impl Person {
    // Constructor
    fn new(name: &str, age: u16) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    // fn
    fn greet(&self) {
        println!(
            "Hello 👋 my name is {} and i'm {} years old ",
            self.name, self.age
        );
    }

    // setter
    fn set_age(&mut self, age: u16) {
        self.age = age;
    }
}

pub fn run() {
    let mut color = HSL {
        hue: 156,
        saturation: 50,
        lightness: 50,
    };

    println!(
        "My color struct: {} {} {}",
        color.hue, color.saturation, color.lightness
    );

    //Mutate struct
    color.hue = 150;

    let mut color_transparent = HSLa(150, 50, 50, 25);
    color_transparent.3 = 0;
    println!(
        "My color struct tuple: {} {} {} {}",
        color_transparent.0, color_transparent.1, color_transparent.2, color_transparent.3
    );

    let mut person = Person::new("Romain Gueffier", 29);
    println!("new person is: {} {}", person.name, person.age);

    // call method of person
    person.greet();

    // mutate person with setter
    person.set_age(30);
    person.greet();
}
