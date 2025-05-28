fn main() {
    let string_from_stack = "this is the string litral which is stored in stack memeory, So accessing this is very efficient";
    println!("{string_from_stack}");

    let mut string_from_heap_which_can_be_mutable = String::from("This is from heap and it is bit slower but we can modify this as much as we want");
    string_from_heap_which_can_be_mutable.push_str(", Hey see this <- this i have added using push_str method!");
    println!("{string_from_heap_which_can_be_mutable}");

    // let string_object_to_given_to_other_method: String = String::from("This is object owned by main method");
    // ownership_change_method(string_object_to_given_to_other_method); 
    // // println!("I expect error while accesing {}", string_object_to_given_to_other_method); yeah working as expeted it throws error
    
    // let non_copy_obj: String = String::from("Object which resides in heap");
    // let copy_obj = non_copy_obj;
    // println!("using copied object, {}",non_copy_obj);
    println!("first word: {}",find_first(&String::from("hello is my first string")));
    
}

fn ownership_change_method(string_obj: String) {
    println!("{}",string_obj);
}



fn find_first(input: &String) -> &str {
    let byte_of_string = input.as_bytes();
    for (i,&character) in byte_of_string.iter().enumerate() {
        if character == b' ' {
            return &input[0..i];
        }
    }
    &input[0..input.len()]
}