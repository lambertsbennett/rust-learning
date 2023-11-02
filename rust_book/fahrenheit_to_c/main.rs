use std::io;

fn main() {
    let mut temp = String::new();

    println!("Enter a temperature in Fahrenheit (floating point):");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = temp.trim().parse().expect("Enter a floating point number!");

    let temp_c = f_to_c(temp);

    println!("{temp} in C is {temp_c}.");
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}
