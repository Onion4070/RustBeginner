fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // Rustは標準で不変扱い，コンパイルエラー
    // x = 8;

    // 変更したい場合はmutをつける
    let mut y = 10;

    println!("The value of y is {}", y);
    y = 200;
    println!("The value of y is {}", y);

    // constはコンパイル時に値が決定している必要がある
    const CONSTANT: i32 = 5;
    println!("Constant value: {}", CONSTANT);


    // シャドーイング
    let z = 5;          // 5
    let z = z + 1;      // 6        複数回の宣言はOK

    {
        let z = z * 2;  // 12
        println!("The value of z in the inner scope is {}", z);
    }

    println!("The value of z is {}", z);

    let string = "aaa";     // &str
    println!("The value of string is {}", string);

    // 再宣言なので型が違ってもOK
    let string = string.len();
    println!("The value of string is {}", string);

    // 再代入では型が同じでないとコンパイルエラー
    let mut _c = "a";
    // これらはコンパイルエラー
    // let mut c = c.len();
    // c = c.len();
    
}
