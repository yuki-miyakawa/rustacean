struct User {
    name: String,
    age: u32,
}

impl User {
    fn print_name(&mut self) {
        self.name = "John".to_string();
        println!("{}", self.name);
        println!("{}", self.age);
    }
}

fn main() {
    let  user = User {
        name: "taro".to_string(),
        age: 20,
    };

    user.print_name();
}
