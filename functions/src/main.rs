fn main() {
    println!("Hello, world!");

    another_function(5, 's');        // 5s

    // ; を書かないとそれが式(右辺値)の評価値となりyに束縛される
    // 関数は複数の文+文or式(終端に; がない)
    let y = {
        let x = 5;
        x + 1
    };
    println!("The value of y is {}", y);    // 6

    // z = 2y
    let z = multiple(y);
    println!("The value of 2y is {}", z);
}

// 定義は後でもOK
// 引数は(変数名): (型名)
fn another_function(x: i32, ch: char) {
    println!("The value of x is {}{}", x, ch);
}

// -> で戻り値の明示的型注釈
// ; はつけてはダメ(式ではなく文となる)
fn multiple(x: i32) -> i32 {
    x * 2
}
