
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    println!("Print the area of a rectangle.");
    let rect1: Rectangle = Rectangle {
        width: 20,
        height: 30
    };

    let area: u32 = area(&rect1);

    println!("The Area is {}", area);
    println!("Rect first: {:?}", rect1);
    dbg!(rect1);
}
