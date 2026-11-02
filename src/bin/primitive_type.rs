fn main() {
    let x: usize = 6;
    let y: f64 = 1.5;

    // 型の違う演算はできない
    // let z = x / y;

    // 型を揃える
    let z = (x as f64) / y;

    println!("z: {}", z);

    let big_integer = 1_000_000_000;
    println!("big_integer is {}", big_integer);

    // println!()は10進表記
    let octal = 0o77;
    println!("octal is {}", octal);

    let a = [1, 2, 3, 4, 5];
    println!("first of a is: {}", a[0]);

    // Panic
    // println!("out of bound of a is: {}", a[10]);
}
