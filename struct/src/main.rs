// 全要素を出力するようにコンパイラに教える
#[derive(Debug)]

// 構造体定義
struct User {
    // 変数名: 型名で記述
    username: String, 
    email: String, 
    sign_in_count: u64, 
    active: bool
}

// implでその型に限定した関数を定義
impl Rectangle {
    // Selfは自身の型を表す
    // selfを使わない -> staticメソッドに対応
    fn square(width: u32) -> Self {
        Self {
            width, 
            height: width, 
        }
    }

    // selfは自分自身(Rectangleのインスタンス)
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // 書き換える場合は可変な参照
    fn set_width(&mut self, width: u32) {
        self.width = width;
    } 
}

// タプルで名無しのメンバがいてもOK
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// derive宣言は構造体ごとに
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"), 
        username: String::from("someusername123"), 
        active: true, 
        sign_in_count: 1, 
    }; 

    println!("{}", user1.email);

    // メールアドレスを変更
    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);


    let user1 = build_user("jackson@example.com".to_string(), "jackson".to_string());

    // {:?}はデバッグ表示
    println!("{:?}", &user1);

    let user2 = User {
        email: String::from("johnny@example.com"), 
        username: String::from("jonny"), 
        ..user1     // user1の値を引き継ぎ
    };

    println!("{:?}", &user2);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let origin = Point(0, 0, 0);


    let mut rect = Rectangle{
        width: 30, 
        height: 50, 
    }; 

    println!(
        "The area of the rectangle is {} sqare pixels.", 
        // area(&rect)
        rect.area()
    );

    rect.width = 60;

    println!(
        "The area of the rectangle is {} sqare pixels.", 
        // area(&rect)
        rect.area()
    );

    println!("Before");
    println!("{:?}", &rect);

    // widthをset_widthで変更
    rect.set_width(40);
    println!("After");
    println!("{:?}", &rect);


    // staticなメソッドへのアクセスは :: を使う
    let square = Rectangle::square(60);
    println!("{:?}", &square);

    println!(
        "The area of rectangle is {} square pixels. ", 
        square.area(), 
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,              // 左辺と右辺が一致するときは略記可能(email: email)と等価
        username, 
        active: true, 
        sign_in_count: 1
    }
}

// 借用して渡す(所有権が関数内にムーブするため)
// 関数内で値変更はしないので可変参照(mut)にする必要はない
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}