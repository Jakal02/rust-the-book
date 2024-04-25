
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 19,
        height: 29
    };

    println!("Can rect1 fit rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 fit rect1? {}", rect1.can_hold(&rect1));
    println!("Can rect2 fit rect1? {}", rect2.can_hold(&rect1));

}
