fn main() {
    println!("Hello, world!");

    let maybe_number = Some(5);
    // matchの一部を取り出すときにif let
    if let Some(number) = maybe_number {
        println!("found {}", number);
    } 
    else {
        println!("Nothing found");
    }
    
    // matchでは全要素を列挙する必要がある
    match maybe_number {
        Some(number) => println!("found {}", number),
        None => println!("Nothing found"),
    }
}
