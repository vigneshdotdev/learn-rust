fn main() {
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    const THIS_IS_INT_CONST: i32 = 5 + 10 + 5;

    println!("The value of THIS_IS_INT_CONST is: {THIS_IS_INT_CONST}");

    let mut this_is_my_num: i128 = 123123123123123123123123123123123123123;

    println!("This is my num: {}", this_is_my_num);

    this_is_my_num = 433214234234234234234;

    println!("This is my num after mutation: {}", this_is_my_num);

    another_function(32, '💛');

    find_value_y();

    println!(
        "Going to multiply the 10 with 20 using multiply_by_20 method {}",
        multiply_by_20(10)
    );

    // This is my first comment in rust

    // Control Statements
    use_control_statement_for_first_time();
    divisibility_test();
    let number_from_if_expr = if 3 > 6 {
        "Three is greater than six"
    } else {
        "Three is less than six"
    };
    println!("The number from if expression is: {}", number_from_if_expr);

    // Loops

    // Repeatation with loop keyword
    loop_until_1000();

    let value_from_loop: String = get_value_from_loop();
    println!("value from loop: {value_from_loop}");

    nested_loop_with_name();
    // Repeatation with while loop
    explore_while_loop();

    let this_is_tupple: (i32, bool, f64) = (230, true, 20.0);
    println!(
        "First position {}, Second postion {}, Thrid position {} of my tuple",
        this_is_tupple.0, this_is_tupple.1, this_is_tupple.2
    );

    let this_is_arr1 = [true; 6];

    println!(
        "First position {}, Second postion {}, Thrid position {} of my array",
        this_is_arr1[0], this_is_arr1[1], this_is_arr1[2]
    );

    let my_second_arr: [i32; 5] = [5, 4, 3, 2, 1];

    for number in my_second_arr {
        println!("Current number in array is: {number}");
    }

    count_down_to_complete_chapter_three();
}

fn another_function(x: i32, unicode_label: char) {
    println!(
        "in Another funtion :) and value of x is: {x} also the emoji given is: {unicode_label}"
    );
    let y = 23;
    println!("{}", y);
}

fn find_value_y() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("Value of Y is: {}", y);
}

fn multiply_by_20(input_number: i32) -> i32 {
    return input_number * 20;
}

fn use_control_statement_for_first_time() {
    //Verify the given number is less than 5 or not. It is less print LESS else print Greater
    const GIVEN_NUMBER: i8 = 6;
    if GIVEN_NUMBER < 5 {
        println!("LESS");
    } else {
        println!("GREATER");
    }
}

fn divisibility_test() {
    const NUMBER: u8 = 7;

    if NUMBER % 4 == 0 {
        println!("Number is divisible by 4");
    } else if NUMBER % 3 == 0 {
        println!("Number is divisibile by 3");
    } else if NUMBER % 2 == 0 {
        println!("Number is divisibile by 2");
    } else {
        println!("Unable to find divisbility");
    }
}

fn loop_until_1000() {
    //using loop
    let mut counter: u16 = 1;
    loop {
        println!("Inside loop! and current count is: {counter}");
        counter += 1;
        if counter == 1001 {
            break;
        }
    }
}

fn get_value_from_loop() -> String {
    let mut counter = 0;
    loop {
        println!("Finding correct word {counter}th time");
        if counter == 10 {
            return "Found 10".to_string();
        }
        counter += 1;
    }
}

fn nested_loop_with_name() {
    let mut counter = 0;
    'loop_label: loop {
        let mut remaining: u8 = 10;
        println!("Current counter: {counter}");
        loop {
            println!("Remaing: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'loop_label;
            }
            remaining -= 1;
        }
        counter += 1;
    }
}

fn explore_while_loop() {
    let mut counter = 10;
    while counter > 0 {
        println!("Counter is {counter}, and it is positive");
        counter -= 1;
    }

    println!("yeah...Counter reached ZERO");
}

fn count_down_to_complete_chapter_three() {
    for count in (1..11).rev() {
        println!("{count} remaining!!!");
    }
    println!("Completed Chapter third successfully");
}
