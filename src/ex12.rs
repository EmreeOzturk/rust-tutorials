pub fn ex12() {
    let dims: Dimension = Dimension::new(1.0, 2.0, 3.0);
    let color: Color = Color::GREEN;

    let box1 = Box::new(5, color, dims);
    let box2: Box = Box::new(1, Color::RED, Dimension::new(2.2, 3.3, 4.4));

    box1.print();
    box2.print();
}

struct Dimension {
    x: f32,
    y: f32,
    z: f32,
}

enum Color {
    GREEN,
    RED,
}

impl Dimension {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn print(&self) {
        println!("x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl Color {
    fn print(&self) {
        match self {
            Color::GREEN => println!("color is : {:?}", "GREEn"),
            Color::RED => println!("color is : {:?}", "RED"),
        }
    }
}

struct Box {
    weight: i32,
    color: Color,
    dimension: Dimension,
}

impl Box {
    fn new(weight: i32, color: Color, dimension: Dimension) -> Self {
        Self {
            weight,
            color,
            dimension,
        }
    }

    fn print(&self) {
        println!("weight : {}", self.weight);
        self.dimension.print();
        self.color.print();
    }
}
