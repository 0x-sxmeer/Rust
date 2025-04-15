struct Person {
    name: String,
    age: u32,
}
//Struct

enum TrafficLight {
    Yellow,
    Red,
    Green,
}


fn main() {

let user1 = Person {
    name: String::from("John"),
    age: 30,
};

println!("Name: {}, Age: {}", user1.name, user1.age);


let light = TrafficLight::Yellow;
let light2 = TrafficLight::Red;
let light3 = TrafficLight::Green;
match light {
    TrafficLight::Yellow => println!("The light is yellow"),
    TrafficLight::Red => println!("The light is red"),
    TrafficLight::Green => println!("The light is green"),

}



}