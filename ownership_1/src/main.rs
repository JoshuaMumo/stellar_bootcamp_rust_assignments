struct GroceryItem {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(grocery_item: &GroceryItem) {
    println!("quantity {}", grocery_item.quantity);
}

fn display_id_number(grocery_item: &GroceryItem) {
    println!("id_number {}", grocery_item.id_number);
}

fn main() {
    let grocery_item_value = GroceryItem {
        quantity: 10,
        id_number: 20,
    };
    display_quantity(&grocery_item_value);
    display_id_number(&grocery_item_value);
}