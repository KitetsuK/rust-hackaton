fn main() {
    let person = Person::make_person("name");
    println!("Person name: {}", person.name)
}

impl Person<'_> {
    fn make_person(s: &str) -> Person<'_> {
        Person {name: s}
    }
}
struct Person<'a> {
    name: &'a str,
}
