fn main() {
    variables();
}



fn variables()
{
    // String 
    let name = "Hanna";
    println!("My name is {}",name); 

    // Integers 
    let age: i32 = 24; 
    let special_number: i8 = 126;

    println!("My age is {}",age); 
    println!("My special number is {}",special_number);

    // Floats 
    let weight: f32 = 101.7; 

    println!("I weigh {} pounds",weight);

    // Char
    let first_letter_of_name: char = 'H';

    println!("The first letter of my name is {}",first_letter_of_name);

    // Boolean
    let active_status: bool = true;

    println!("My active status is: {}", active_status);
    
}