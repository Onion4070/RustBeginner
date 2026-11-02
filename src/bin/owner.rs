fn main() {
    {
        // sはここから有効, &str変更不可, スタック領域に確保
        let s: &str = "hello";
        println!("{}", s);
    }
    // スコープを抜けるとsは"hello"を破棄する
    // println!("{}", s);       // コンパイルエラー

    // 一方Stringは後から変更可能, ヒープ領域に確保
    {
        let mut s = String::from("Hello");
        s.push_str(", World");
        println!("{}", s);      // Hello, World
    }
    // ここでsはメモリ解放


    // これは問題ない(スタックに確保されるものは問題ない)
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);

    //　s1はs2にムーブ(s1は無効になる)
    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("s1 is {}", s1);        // コンパイルエラー

    // コピーを作成したければcloneメソッドを使う
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // OK
    println!("s1 is {}", s1);       // hello
    println!("s2 is {}", s2);       // hello


    let s = String::from("Hello");      // sがスコープに入る
    takes_ownwership(s);                // ここで関数にムーブ
    // println!("s is {}", s);                //コンパイルエラー

    let x = 128;
    makes_copy(x);              // xがコピーされる
    println!("x is {}", x);     // コピーなのでOK


    // 戻り値に指定して所有権を返す方法もあるが面倒...

    // 所有権を一時的に借りる借用を使う
    let s1 = String::from("borrow");
    let len = calc_len(&s1);
    println!("s = {}, s.len() = {}", s1, len);       // s = borrow, s.len() = 6

    // String& s は　String s(ヒープ領域)を指すポインタである

    // 可変な借用には &mut をつける
    let mut s1 = String::from("Hello");
    println!("s1 is {}", s1);
    change(&mut s1);
    println!("s1 is {}", s1);

    // 制約: 可変な借用は1つまでしか持てない (データ競合を防ぐため)
    let mut s = String::from("Hello");
    let r1 = &mut s;
    // let r2 = &mut s;    // コンパイルエラー

    // println!("{}, {}", r1, r2);

    // スコープを抜ければ問題ない
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1がdrop
    let r2 = &mut s;    // OK

    // 制約: 不変な参照と可変な参照を組み合わせられない
    let mut s = String::from("hello");

    let r1 = &s;        // OK
    let r2 = &s;        // OK
    // let r3 = &mut s;    // NG(コンパイルエラー)

    // println!("{}, {}, {}", r1, r2, r3);

    // let &s = dangle();

    let s = no_dangle();
    println!("{}", s)

}


fn takes_ownwership(string: String) { // stringがスコープに入る
    println!("{}", string);
} // ここでstringがスコープを抜けdropする．(stringが解放される)

fn makes_copy(x: i32) {
    println!("x is {}", x);
}

// &をつけて借用する(所有権を貸すだけなので関数内部でムーブはできない)
fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World");
}

// fn dangle() -> &String {
//     // sはスタック領域に確保されるが...
//     let s = String::from("Hello");
//     // ヒープ領域内の実データは有効だが，そのポインタを確保するsが解放されてしまう
//     &s      // 関数外でもsへアクセスされる可能性がある -> コンパイルエラー
// }

// 関数外から扱いたいなら所有権を移す
fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}