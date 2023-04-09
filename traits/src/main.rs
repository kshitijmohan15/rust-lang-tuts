#[derive(Debug)]
struct Dog {
    name: String,
}
trait Speak {
    fn speak(&self);
}
impl Speak for Dog {
    fn speak(&self) {
        println!("woof woof {}", self.name)
    }
}
fn main() {
    let dog = Dog {
        name: String::from("Kshitij"),
    };
    take_dog(dog);
}

fn take_dog(dog: Dog) {
    println!("took a dog man its name is {}", dog.name)
}
