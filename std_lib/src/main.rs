use std::vec;

fn main() {
    let mut vector: Vec<u32> = Vec::new();
    vector.push(23);
    vector.push(23);
    vector.push(23);
    vector.push(23);
    vector.push(23);
    vector.push(23);
    vector.push(23);
    vector.push(23);
    println!("Content of vec: {:?}",vector);

    let mut vec_with_values = vec![23,23,43];
    vec_with_values.push(43);
    println!("Content of vec_with_values -> {:?}",vec_with_values);


    let first_element_in_vec: &u32 = &vector[0];

    println!("First element in the vector: {}",first_element_in_vec);

    print!("Itring over vec : ");

    for value in &mut vector {
        *value += 2;
        print!("{},",value);
    }
    println!("");

    let vector: Vec<u32> = Vec::new();

    let first_element_in_vec = vector.get(0);
    match first_element_in_vec {
        Some(first_element_in_vec) => println!("Value of first element is: {}",first_element_in_vec),
        None => println!("no first position available"),
    }


}
