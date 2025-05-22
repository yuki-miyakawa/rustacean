#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area_impl(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels by area()", area(width, height));

    let rect = (width, height);
    println!("The area of the rectangle is {} square pixels by area_tuple()", area_tuple(rect));

    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect_struct: {:?}", rect_struct);
    println!("The area of the rectangle is {} square pixels by area_struct()", area_struct(&rect_struct));

    println!("The area of the rectangle is {} square pixels by area_impl()", rect_struct.area_impl());
    println!("{:?}", rect_struct)
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
