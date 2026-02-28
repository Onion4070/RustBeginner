# データ型
## スカラー
### 整数型

|Length|Signed|Unsigned|
|---|---|---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|arch|isize|usize|

**arch**はそのPCに最適なサイズ

### 数値リテラル
|数値リテラル|例|
|---|---|
|10進数|98_765|
|16進数|0xff|
|8進数|0o77|
|2進数|0b1111_0000|
|バイト(u8だけ)|b'A'|

桁をわかりやすく`_`で区切ってもOK

### 浮動小数点型
|Length|Signed|
|---|---|
|32-bit|f32|
|64-bit|f64|

Rustでは通常`f64`で扱われる

### 論理値型
`true`と`false`だけ

### 文字型
Rustの`char`は4バイト．Unicodeのスカラー値を表す．
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
                       // 明示的型注釈付きで
    let heart_eyed_cat = '😻';    //ハート目の猫
}
```
このように様々な文字を格納できる

## 複合型
### タプル型
一度設定すると長さが固定
```rust
fn main() {
    // タプル宣言
    let tup = (500, 6.4, 1);
    // まとめて格納
    let (x, y, z) = tup;

    // 明示的型注釈
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // 各要素へのアクセスは.indexで(0-indexed)
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```
### 配列型
```rust
fn main() {
    // 配列の宣言
    let a = [1, 2, 3, 4];

    // 明示的型注釈 (型名; 要素数)
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // ある初期値で初期化
    let c = [3; 5];     // [3, 3, 3, 3, 3];

    // 要素へのアクセス(0-indexed)
    let first = a[0];
    let second = a[1];

    // 範囲外アクセスはPanicを起こす
    let out_of_bound = a[10]; 
}
```
配列はスタック領域に確保される．
範囲外アクセスは`Panic`を起こして安全にプログラムが終了する．

