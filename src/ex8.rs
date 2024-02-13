pub fn ex8() {
    enum FlavorDrinks {
        OrangeJuice,
        MOJITO,
    }

    enum Ounces {
        Small,
        Big,
    }

    struct DrkAndOnc {
        drink: FlavorDrinks,
        ounce: Ounces,
    }

    let ss: DrkAndOnc = DrkAndOnc {
        drink: FlavorDrinks::MOJITO,
        ounce: Ounces::Big,
    };

    fn print(flavor: DrkAndOnc) {
        match flavor.drink {
            FlavorDrinks::MOJITO => println!("mojito"),
            FlavorDrinks::OrangeJuice => println!("orange juice"),
        }

        match flavor.ounce {
            Ounces::Big => println!("big"),
            Ounces::Small => println!("small"),
        }
    }

    print(ss);
}
