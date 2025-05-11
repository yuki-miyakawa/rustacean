fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let mut s1 = "hello world";
    println!("s1の値: {}", s1);
    println!("s1の型: {}", type_of(s1));
    println!("s1のポインタ: {:p}", &s1);
    println!("s1のポインタ: {:p}", &&s1);

    s1 = "hello!!!!!!";
    println!("{}", s1);
    println!("s1のポインタ: {:p}", &s1);
    println!("s1のポインタ: {:p}", &&s1);
    
}
