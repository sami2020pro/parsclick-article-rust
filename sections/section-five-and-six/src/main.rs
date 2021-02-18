struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point: Point = Point { x: 10, y: 5 };
    println!("x: {:?} and y: {:?}", point.x, point.y);
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn original() -> Point {
        Point { x: 1, y: 1 }
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let point_one: Point = Point::original();
    println!("x: {:?} and y: {:?}", point_one.x, point_one.y);

    let point_two: Point = Point::new(10, 5);
    println!("x: {:?} and y: {:?}", point_two.x, point_two.y);
}

#[derive(Debug)]
struct Number {
	x: i32,
}

impl Number {
	fn print_min_y(&mut self, x: i32) -> i32 {
		self.x -= x;
		return self.x;
	}
}

fn main() {
    let number: &mut Number = &mut Number { x: 11 };
    println!("number.print_min_y: {:?}", number.print_min_y(1));
    number.print_min_y(1);

    println!("self.x: {:?}", number.x);
    println!("number: {:?}", number);
}

fn main() {
    fn function (i: i32) -> i32 { i + i }

    let closure_annotated = |i: i32| -> i32 { i + i };
    let closure_inferred  = |i     |          i + i  ;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}