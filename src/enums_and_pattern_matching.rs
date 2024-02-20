enum IpAddrKind {
    V4(String),
    V6(String)
}

pub fn enums_and_pattern() {
    let mut localhost: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1"));
    let six: IpAddrKind = IpAddrKind::V6(String::from("127.0.0.4"));


    let some_value : Option<i32> = Some(5);

    if let Some(3) = some_value {
        println!("three");
    } else {
        println!("not three");
    }
}

struct IpAddr<'a> {
    kind: IpAddrKind,
    address: &'a str,
}