use helloworld::print_line_separator;

fn demo_1() {
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
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn demo_2() {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    let d1 = p1.distance(&p2);
    println!("The distance between p1 and p2 is {}.", d1);
    let d2 = (&p1).distance(&p2);
    println!("The distance between p1 and p2 is {}.", d2);
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}
