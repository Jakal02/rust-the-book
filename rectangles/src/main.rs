
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    println!("Print the area of a rectangle.");
    let rect1: Rectangle = Rectangle {
        width: 20,
        height: 30
    };

    let area: u32 = rect1.area();

    println!("The Area is {}", area);
    println!("Rect first: {:?}", rect1);
    dbg!(rect1);
}
