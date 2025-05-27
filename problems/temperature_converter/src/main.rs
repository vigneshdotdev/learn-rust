use std::io;

fn main() {
    println!("Hey guys, Welcome to temperature converter!");
    let mut temperature: String = String::new();
    let mut is_fahren_to_celsius: String = String::new();
    println!("plese enter your temperature:");   
    io::stdin().read_line(&mut temperature).expect("The given input is not valid");
    println!("Do you want to convert this into Fahrenheit to Celsius ?");  
    io::stdin().read_line(&mut is_fahren_to_celsius).expect("The given input is not valid");
    let temperature: f32 = temperature.trim().parse().expect("Failed to parse input to int");

    println!("Given Temperature is {temperature}");

    let is_fahren_to_celsius: bool = is_fahren_to_celsius.trim() == "yes";

    let converted_value = if is_fahren_to_celsius {convert_f_to_c(temperature)} else {convert_c_to_f(temperature)};

    println!("Calculated {} from {}, The result is {}", if is_fahren_to_celsius {"Celsius"} else {"Fahrenheit"}, if !is_fahren_to_celsius {"Celsius"} else {"Fahrenheit"}, converted_value );


}


fn convert_f_to_c(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) * (5.0/9.0);
}

fn convert_c_to_f(celsius: f32) -> f32 {
    return (9.0/5.0 * celsius) + 32.0;
}