struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}

fn main() {
    let taro = Person::new("太郎".to_string(), 18);
    println!("{}さんは{}才", taro.name, taro.age);
}
