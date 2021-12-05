fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    let square = Rectangle::square(50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(unused)]
    fn area(&self) -> u32 {
        self.width * self.height
    }

    #[allow(unused)]
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width >= another.width && self.height >= another.height
    }

    #[allow(unused)]
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}