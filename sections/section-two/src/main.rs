fn main() {
       // The first method

       /*
        * The second method
        */

       /* The third method */
       println!("Hello World")
}

fn main() {
        let my_age: i32  = 14;
        let my_birthday: i32 = 2007;

        println!("Age: {} | birthday: {}", my_age, my_birthday);
}

fn main() {
    let _variable: i32 = 2007
}

fn main() {
    let _variable = 2007
}

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);
}

fn main() {
        // create a variable with f64 type and use static type
        let variable: f64 = 3.14000000;

        /* format and print the variable */
        println!("variable: {}", variable);
}
