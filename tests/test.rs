fn main() {
    let source:Vec<i32> = vec![1,2,3,4,5,6,7,8,9];

    let output = source.iter().filter(|x| **x%2==0) .map(double_fn)
    //;
    .collect::<Vec<_>>();


    println!("{:?}", output);
}


fn double_fn(i: &i32) -> i32 {
    println!("double_fn called with {}", i);
    2 * i
}