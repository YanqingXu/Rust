fn main() {
    for i in 1..=5 {    // 1..=5 is a range that includes 5
        println!("{}", i);
    }

    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}
