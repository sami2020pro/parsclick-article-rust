fn main() {
    let x = (1i8, 1000i32, 0.2f64, true, 'A', 4u64);
    println!("one element of x: {}", x.4);
}

fn main() {
    let x: [i32; 5] = [1, 2, 3, 4, 5];
    let y = ['A', 'B', 'C', 'D'];

    println!("first element of x: {}", x[4]);
    println!("last element of y: {}", y[1]);
}