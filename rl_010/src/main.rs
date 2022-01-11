fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("the area of the rectangle is {} square pixels.",
             area(width1, height1));
    let dem = (30, 50);
    println!("the area of the rectangle is {} square pixels.",
             area2(dem));
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the area of the rectangle is {} square pixels.",
             area3(&rect));
    println!("Hello, world!");
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area2(demensions: (u32, u32)) -> u32 {
    demensions.0 * demensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
