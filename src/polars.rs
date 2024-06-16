use polars::prelude::*;

pub fn example() {
    let df = df! {
        "a" => &[1,2,3,4,5],
        "b" => &["a", "b", "c", "d", "e"],
        "c" => &["A", "B", "B", "A", "C"]
    }
    .unwrap();
    println!("{:?}", df);
}
