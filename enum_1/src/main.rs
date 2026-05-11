fn main() {
    enum Color{
        Red,
        Yellow,
        Green, 
        Blue,
    }
    fn color_name(col: &Color) -> &str{
        match col {
            Color::Red => "Red",
            Color::Yellow => "Yellow",
            Color::Green => "Green",
            Color::Blue => "Blue",
        }
    }
    let col: Color = Color::Red;
    println!("color {}", color_name(&col));
}
