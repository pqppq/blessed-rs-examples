use itertools::Itertools;

pub fn example() {
    let it = (1..3).interleave(vec![-1, -2]);
    itertools::assert_equal(it, vec![1,-1,2,-2]);

    let x = (1..=10).collect_vec();
    println!("{:?}", x);

    let y = (1..=2).collect_tuple::<(usize,usize)>();
    println!("{:?}", y);

    let it = vec![1,2,2,3,3,3].into_iter().unique();
    println!("{:?}", it.collect_vec());

    let it = vec![1,2,3,3,2,1].into_iter().dedup();
    println!("{:?}", it.collect_vec());

    let s = (0..=5).join(", ");
    println!("{s}");

    let a = itertools::fold(vec![1,2,3,4,5], 0, |a, b| a + b);
    println!("{a}");

    let b = (1..10).rev().collect_vec();
    println!("{:?}", b);
}
