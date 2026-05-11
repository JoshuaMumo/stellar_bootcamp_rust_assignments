fn tuples_data() -> (i32, i32) {
    (2, 3)
}

fn main() {
    let (x, y) = tuples_data();  

    if y > 5 {
        println!("({}, {}) — {} is greater than 5", x, y, y);
    } else if y < 5 {
        println!("({}, {}) — {} is less than 5", x, y, y);
    } else {
        println!("({}, {}) — {} is equal to 5", x, y, y);
    }
}