fn main() {
    println!("Hello, world!");
    let number_collection : Vec<_> = [12, 1402, 10 ,932, 234, 221, 9, 8, 7]
    .iter()
    .filter_map(|n| (n<&30).then_some(n))
    .collect();

    println!("{number_collection:?}");
}
