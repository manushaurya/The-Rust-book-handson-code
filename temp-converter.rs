use std::io;

fn main() {
    let result: f64;
    let mut choice = String::new();
    let mut temp_str = String::new();
    let temp: f64;
    println!("Make your choice :");
    println!("1. Fahreinheit to Celcius");
    println!("2. Celcius to Fahrenheit");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice = choice.trim().parse().expect("Please type a number!");
    println!("You've selected {}", choice);
    
    if choice=="1"{
        println!("Enter the value in fahrenheit:");
        io::stdin()
        .read_line(&mut temp_str)
        .expect("Failed to read temperature");
        temp = temp_str.trim().parse().unwrap();
        result = fah2cel(temp);
        println!("In celcius: {result}");
    }
    else if choice=="2"{
        println!("Enter the value in celcius:");
        io::stdin()
        .read_line(&mut temp_str)
        .expect("Failed to read temperature");
        temp = temp_str.trim().parse().unwrap();
        result = cel2fah(temp);
        println!("In fahrenheit: {result}");
    }
    else{
        println!("Incorrect selection!");
    }
}

fn cel2fah(cel: f64) -> f64 {
    (cel*(9.0/5.0))+32.0
}

fn fah2cel(fah: f64) -> f64 {
    (fah-32.0)*(5.0/9.0)
}
