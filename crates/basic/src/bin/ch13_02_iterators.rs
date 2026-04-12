use helloworld::print_line_separator;

fn demo_1() {
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        assert_eq!(v1_iter.next(), None);
    }

    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum(); //消耗v1_iter的所有权
        //v1_iter.next(); //错误：v1_iter 已经被 sum() 消耗掉了，不能再调用 next() 方法了
        assert_eq!(total, 6);
    }

    fn iterator_change() {
        let mut v1 = vec![1, 2, 3];
        let v1_iter = v1.iter_mut(); //&mut v1_iter是一个可变迭代器，允许我们修改 v1 中的元素
        for item in v1_iter {
            *item += 50;
        }
    }

    fn iterator_collect() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let v1_iter_collected: Vec<_> = v1_iter.map(|x| x + 1).collect();
        assert_eq!(v1_iter_collected, vec![2, 3, 4]);
    }

    iterator_demonstration();
    iterator_sum();
    iterator_change();
    iterator_collect();
}

fn demo_2() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );

}

fn main() {
    demo_1();
    print_line_separator();
    demo_2();
}
