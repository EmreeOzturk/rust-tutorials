pub fn ex11() {
    let item1 = Item {
        quantity: 5,
        id: 32,
    };

    print_quantity(&item1);
    print_id(&item1);
}

struct Item {
    quantity: i32,
    id: i32,
}

fn print_quantity(i: &Item) {
    println!("item's quantity {}", i.quantity)
}
fn print_id(i: &Item) {
    println!("item's id {}", i.id)
}
