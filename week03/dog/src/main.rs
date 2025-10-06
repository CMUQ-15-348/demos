struct Dog {
    name: String,
    age: i32,
}

// This is bad, because it takes ownership of the dog
fn print_dog(dog: Dog) {
    println!("{} is {} years old", dog.name, dog.age);
}

fn better_print_dog(dog: Dog) -> Dog {
    println!("{} is {} years old", dog.name, dog.age);
    dog
}

// This borrows the dog struct
fn best_print_dog(dog: &Dog) {
    println!("{} is {} years old", dog.name, dog.age);
}

fn age_dog(dog: &mut Dog) {
    dog.age += 1;
}

fn main() {
    let mut my_dog = Dog {
        name: String::from("Fido"),
        age: 5,
    };
    println!("{} is {} years old", my_dog.name, my_dog.age);
    my_dog.age = 10;
    println!("{} is {} years old", my_dog.name, my_dog.age);

    /*
    // This doesn't work, because the assignment moves the ownership
    let same_dog = my_dog;
    println!("{} is {} years old", same_dog.name, same_dog.age);
    println!("{} is {} years old", my_dog.name, my_dog.age);
    */

    /*
    // This doesn't work, because the function call moves the ownership
    print_dog(my_dog);
    my_dog.age = 15;
    */
    
    my_dog = better_print_dog(my_dog);
    my_dog.age = 11;
    my_dog = better_print_dog(my_dog);

    best_print_dog(&my_dog);
    my_dog.age = 12;
    best_print_dog(&my_dog);

    age_dog(&mut my_dog);
    best_print_dog(&my_dog);

    let ptr1 = &my_dog;
    let ptr2 = &my_dog;

    best_print_dog(ptr1);
    best_print_dog(ptr2);

    /*
    // This rule: Only one mutable borrow at a time, will drive you crazy
    age_dog(&mut my_dog);
    best_print_dog(ptr1);
    */
}
