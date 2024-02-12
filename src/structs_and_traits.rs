pub fn structs() {
    struct Color {
        _red: u8,
        _green: u8,
        _blue: u8,
    }

    let _bg: Color = Color {
        _red: 255,
        _green: 70,
        _blue: 0,
    };

    struct Person {
        name: String,
        age: u8,
        is_cool: bool,
    }

    impl Person {
        fn _is_cool(&self) -> bool {
            self.is_cool
        }
        fn print_name(&self) {
            println!("{}", self.name)
        }
    }

    let _me: Person = Person {
        name: String::from("Namık"),
        age: 30,
        is_cool: true,
    };

    let _name: String = _me.name.clone();
    let _age: u8 = _me.age;
    let _is_cool: bool = _me.is_cool;
    _me.print_name();

    //traits are like interfaces in other languages
    // you can define a trait and implement it for a struct
    trait Animal {
        fn can_fly(&self) -> bool;
        fn is_animal(&self) -> bool {
            true
        }
    }

    impl Animal for Person {
        fn can_fly(&self) -> bool {
            false
        }
    }
}
