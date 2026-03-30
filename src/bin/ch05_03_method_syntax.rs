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
            // `&self` 和 `other: &Point` 都是不可变借用（`&T`）：
            // 1) 调用时会借用当前对象和参数对象，不发生所有权转移，方法返回后 p1/p2 仍可继续使用。
            // 2) 因为是不可变借用，方法体内只能读取坐标，不能修改 `self` 或 `other` 的字段。
            // 3) 等价的完全限定调用形式是 `Point::distance(&p1, &p2)`。
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    let d1 = p1.distance(&p2); // 隐式借用：等价于 Point::distance(&p1, &p2)
    println!("The distance between p1 and p2 is {}.", d1);
    let d2 = (&p1).distance(&p2); // 显式写出 self 的引用，借用语义与上一行相同
    println!("The distance between p1 and p2 is {}.", d2);
}

fn demo_3() {
    #[derive(Debug)]
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

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 area: {}", rect1.area());
    println!("rect2 area: {}", rect2.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
    print_line_separator();
    demo_3();
}
