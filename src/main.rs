/**
 * Area of shapes
 * H12M54AM
 * Oct 17, 2022
 */

struct Dimensions {
    width: i32,
    height: i32
}

impl Dimensions {
    // Bh / 2
    fn triangle_area(&self) -> i32{
        self.width * self.height / 2
    }

    // Height^2
    fn square_area(&self) -> i32 {
        self.height * self.height
    }

    // Bh
    fn rec_area(&self) -> i32 {
        self.height * self.width
    }
}

fn main() {
    let shape = Dimensions {
        width: 40,
        height: 30
    };

    println!("The area of a Triangle is {}", shape.triangle_area());
    println!("The area of a Square is {}", shape.square_area());
    println!("The area of a Rectangle is {}", shape.rec_area());
}