fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // 左闭右开区间
    let world = &s[6..11];

    println!("{} {}", hello, world);
}
