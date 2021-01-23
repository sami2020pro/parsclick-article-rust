fn hello() {
    println!("hello");
 }

 fn main() {
    hello();
}

fn hello() {
    println!("hello");
}

fn one() -> i32 {
    return 10;
}

fn one() -> i32 {
    10
}

fn one() {
    return 10;
}

fn one() {
    10
}

fn main(){
    let x: i32 = 5;
    
    print_zero(x);
    println!("The value of no is: {}", x);
}
 
fn print_zero(mut variable: i32) {
    variable = variable * 0;
    println!("param_no value is: {}", variable);
}