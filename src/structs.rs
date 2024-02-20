enum AddressType {
    Home,
    Work,
    Other,
}
struct Active {
    active: bool,
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: Active,
    address_type: AddressType,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn make_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn make_square2(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: Active { active: true },
        sign_in_count: 1,
        address_type: AddressType::Home,
    }
}

pub fn structs() {
    let mut user1 = User {
        email: String::from("asd@f.com"),
        username: "emre".to_string(),
        active: Active { active: true },
        sign_in_count: 1,
        address_type: AddressType::Home,
    };

    let name = user1.username;
    user1.username = "emreozturk".to_string();

    let user2 = build_user(String::from("qwd@gmail.com"), String::from("emreozturk"));

    let user3 = User {
        email: String::from("test@g.com"),
        username: String::from("test"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    let area = rect1.area();
    println!("{}", area);

    let rect2 = Rectangle::make_square(30);
    println!("{}", rect2.area());

    let can_hoold = rect1.can_hold(&rect2);
    println!("{}", can_hoold);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
