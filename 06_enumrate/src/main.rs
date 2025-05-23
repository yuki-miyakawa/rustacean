fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    
    let some_u8_value = Some(0u8);
    let some_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_value {
        println!("three");
    }
    if let Some(x) = some_value {
        println!("{}", x);
    }

    
    println!("{}", let Some(x) = 5);
// three



}
