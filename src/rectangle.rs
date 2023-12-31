//  Calculating the area of a rectangle
use std::io;

fn main() {
 
 println!("Please input width.");
 
 let mut width = String::new();

 io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line");

        let width:u32  =  width.trim().parse().expect("msg");

 println!("Please input height.");

 let mut height = String::new();

 io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

        let height:u32  =  height.trim().parse().expect("msg");

    println!(
        "The area of the rectangle is {} square pixels",
        area (width, height)
    );


fn area(width: u32, height: u32) -> u32 {
    width * height
}
}









struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1= Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}





#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}






#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
   
   println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
   );
}




#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
   
   if rect1.width() {
   println!("The rectangle has a nonzero width; it is {}", rect1.width);
    
   }
   
}









struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle  {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle  {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle  {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
