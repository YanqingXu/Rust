fn main() {
    let v = Some(3u8);
    if let Some(3) = v {
        println!("Three");
    } else {
        println!("Not three");
    }
}
