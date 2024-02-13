pub fn ex7() {
    #[derive(Debug)]
    enum COLORS {
        YELLOW,
        BLUE,
        RED,
        GREEN,
        BLACK,
        GRAY,
        WHITE,
    }

    let _c: COLORS = COLORS::BLACK;

    fn which_color(color: COLORS) {
        match color {
            COLORS::BLUE => println!("blue"),
            COLORS::RED => println!("red"),
            COLORS::WHITE => println!("white"),
            COLORS::GREEN => println!("green"),
            COLORS::GRAY => println!("gray"),
            _ => println!("others"),
        };

    }

    which_color(COLORS::WHITE);
}
