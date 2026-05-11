struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: String::from("Itachi"),
            locker: Some(101),
        },
        Student {
            name: String::from("Sasuke"),
            locker: None,
        },
        Student {
            name: String::from("Shisui"),
            locker: Some(247),
        },
    ];

    for student in &students {
        match student.locker {
            Some(number) => println!("{} is assigned locker {}", student.name, number),
            None => println!("{} has no locker assigned", student.name),
        }
    }
}