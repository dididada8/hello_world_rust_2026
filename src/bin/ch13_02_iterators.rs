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
        let total: i32 = v1_iter.sum();//消耗v1_iter的所有权
        //v1_iter.next(); //错误：v1_iter 已经被 sum() 消耗掉了，不能再调用 next() 方法了
        assert_eq!(total, 6);
    }
    iterator_demonstration();
    iterator_sum();
}

fn main() {
    demo_1();
}
