fn main() {
    let s = String::from("Hello World");
    // 文字列スライス
    let hello = &s[0..5];
    let hello = &s[..5];     // 等価(最初を省略 -> [0..x] とみなされる)

    let world = &s[6..11];
    let world = &s[6..];     // 等価(最後を省略 -> [0..s.len()] とみなされる)

    let hello_world = &s[..]; // "Hello World"と等価

    println!("{} {}", hello, world);
    println!("{}", hello_world);

    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear();      // コンパイルエラー (first_word() -> 不変な参照, s.clear() -> 可変な参照, 共存不可)
    println!("the first word is {}", word);


    // 配列にもスライスが存在する
    let a = [0, 1, 2, 3, 4, 5];
    let slice = &a[1..3];   // [1, 2]
    println!("{:?}", slice);
    // slice[1] = 5;       // スライスは不変な参照なので書き換えは不可

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
