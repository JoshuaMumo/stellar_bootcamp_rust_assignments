fn main() {
    enum Drink{
        Fanta,
        Cocacola,
        Sprite,
    }
    struct Drinks<'a>{
        Fanta: &'a str,
        Cocacola: &'a str,
        Sprite: &'a str,
    }
    let drinks_flavors_info: Drinks = Drinks{
        Fanta: "fruit based flavored",
        Cocacola: "cola flavored",
        Sprite: "lemon flavored"
    };
    fn drink_flavor<'a>(drink: &'a Drink, flavors: &'a Drinks) -> &'a str{
        match drink {
            Drink::Fanta => flavors.Fanta,
            Drink::Cocacola => flavors.Cocacola,
            Drink::Sprite => flavors.Sprite,
        }
    }
     let drink: Drink = Drink::Fanta;
     println!("Drink flavor: {}",drink_flavor(&drink, &drinks_flavors_info))
}
