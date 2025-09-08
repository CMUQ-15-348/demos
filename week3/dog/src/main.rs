struct Dog {
    name: String,
    age: i32
}

fn print_dog(dog: Dog) {
    println!("{} is {} years old", dog.name, dog.age);
}

fn better_print_dog(dog: Dog) -> Dog {
    println!("{} is {} years old", dog.name, dog.age);
    return dog;
}

fn best_print_dog(dog: &Dog) {
    println!("{} is {} years old", dog.name, dog.age);
}

fn main() {
    let mut my_dog = Dog {
        name: String::from("Fido"),
        age: 5,
    };

    // let other_dog = my_dog;
    // println!("{} is {} years old", other_dog.name, other_dog.age);
    // println!("{} is {} years old", my_dog.name, my_dog.age);

    // print_dog(my_dog);
    // print_dog(my_dog);

    // my_dog = better_print_dog(my_dog);
    // my_dog = better_print_dog(my_dog);

    let ptr1 = &my_dog;
    let ptr2 = &my_dog;
    best_print_dog(ptr1);
    best_print_dog(ptr2);

    my_dog.age = 50;
    best_print_dog(ptr1);
    best_print_dog(ptr2);

}
