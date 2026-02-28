fn main() {
    let number = 5;

    // ifに()は使わない
    if number < 3 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    // FizzBuzz
    let x = 32;

    // ifの後はブロックが必要
    // if x % 15 == 0 println!("Fizz");     // コンパイルエラー

    // ifの右辺値は必ずbool値
    // if x % 2 { println!("Odd") };        // コンパイルエラー

    if x % 15 == 0 { println!("FizzBuzz"); }
    else if x % 3 == 0 { println!("Fizz"); }
    else if x % 5 == 0 { println!("Buzz"); }
    else { println!("{}", x); }

    // ifは式なので評価値が存在する
    let condition = true;
    let number = if condition { 5 } else { 0 };     // 3項演算子と同じ
    println!("The value of number is {}", number);

    // 但し型は同じでなければならない(コンパイル時に定まらないため)
    // let number = if condition { 5 } else { "zero" };     // コンパイルエラー

    // loop (無限ループ)
    // loop {
    //     println!("again");
    // }

    let mut count = 0;

    // loopにラベル付けもできる
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // counting_upのloopを抜ける -> println!("End count = ...")に進む
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End cout = {}", count);

    // while (条件付きループ)
    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFT OFF!!!");

    // for (コレクションを覗き見るなど)
    let a = [10, 20, 30, 40, 50];

    // Pythonみたいなfor-each
    for element in a {
        println!("the value is {}", element);
    }

    // Pythonみたいなfor range()
    for i in 0..5 {
        println!("{}", i);
    }
}
