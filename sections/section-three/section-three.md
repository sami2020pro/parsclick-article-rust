در مقاله ی بعدی `راست` میخواهیم مفاهیم جدیدتری رو با هم دیگه یادبگیریم.

![مقاله سوم آموزش راست](https://images.pexels.com/photos/3184454/pexels-photo-3184454.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260)

توی این مقاله میخواهیم با سه مفهوم جدید در زبان راست آشانا بشیم یعنی `Literals and Operators` و `Tuple` و `Array` در راست.

### اصطلاحات و عملگر ها یا Literals and Operators

به کد زیر توجه کنید

```rust
fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
```

به خروجی کد بالا هم در زیر توجه کنید

```
1 + 2 = 3
1 - 2 = -1
true AND false is false
true OR false is true
NOT true is false
0011 AND 0101 is 0001
0011 OR 0101 is 0111
0011 XOR 0101 is 0110
1 << 5 is 32
0x80 >> 2 is 0x20
One million is written as 1000000
```

حالا میخوایم قدم به قدم کد بالا رو بررسی کنیم.

### تاپل یا Tuple

تاپل ها مجموعه ای از **مقدار** های ***مختلف*** با **نوع** های ***مختلف*** هستند؛ برای درست کردن یک تاپل باید از علامت `()` یا به عبارت دیگه پرانتز استفاده کنیم.

### آرایه یا Array
