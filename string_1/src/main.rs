struct Person {
    age: u32,
    name: String,
    favorite_color: String,
}

fn person_info(name: &str, favorite_color: &str) {
    println!("Name: {}", name);
    println!("Favorite color: {}", favorite_color);
    println!("   ");
}

fn main() {
    // people created here so the vector can see them
    let people = vec![
        Person {
            name: String::from("Itachi"),
            age: 8,
            favorite_color: String::from("orange"),
        },
        Person {
            name: String::from("Sasuke"),
            age: 15,
            favorite_color: String::from("purple"),
        },
        Person {
            name: String::from("Shisui"),
            age: 10,
            favorite_color: String::from("green"),
        },
    ];

    for person in &people {
        if person.age <= 10 {
            person_info(&person.name, &person.favorite_color); // name matches now
        }
    }
}