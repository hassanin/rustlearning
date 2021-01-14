use libraries::{get_name, make_human_speak, Employee, FromString, Human, Person};

fn main() {
    let person = Person {
        name: "Mohamed",
        age: 0,
        height: 0,
    };
    println!("Preson is {:?}", person);
    let my_string = serde_json::to_string(&person).unwrap();
    let my_binary = serde_json::to_vec(&&person).unwrap();
    println!("My person is {}", my_string);
    println!("My person is {:?}", my_binary);
    println!("Hello, world!");
    make_human_speak(Box::new(person));
    for i in 0..10 {
        print!("{}", i);
    }
    person.speak();
    let lines = r##" Hello there
    My name is Bond
    James Bond"##;
    let ds = lines
        .split_whitespace()
        .into_iter()
        .map(str::to_string)
        // .map(<str as ToString>::to_string)
        .map(|s| s.to_ascii_lowercase())
        .collect::<Vec<String>>();
    ds.into_iter().for_each(|s| println!("{}", s));
    let rts = Employee::from_string("Hello there").unwrap();

    println!("Employee is {:?}", rts);
    let mut name = get_name(&person);
    name = &"Excellent0";
    let mut person2 = person;
    person2.name = "Ahmedsd";
    println!("name is {}", name);
    println!("Person 2 name is {:?}", person2);
    println!("Person 1 name is {:?}", person);
}
