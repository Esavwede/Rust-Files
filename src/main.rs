

fn main() {
    variables();
    operations();
    if_else();
    matching();
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


fn operations()
{
    arithmetic();
}

fn arithmetic()
{
    println!(" 1 * 1 = {}", 1 * 1 );
    println!("1 + 1 = {}",1 + 1 );
    println!(" 1 / 1 = {}",1 / 1 );
    println!(" 1 - 1 = {}", 1 - 1 );
    println!("10 % 5 = {}", 10 % 5 ); 
}


fn if_else()
{
    let user_online: bool = false; 

    if !user_online 
    {
        println!("User is not online ")
    }
    else {
        print!("User is online ")
    }
}

fn matching()
{
    let number: i8 = 4; 

    match number {

     4 => print!("You have inputted the value {}",number),
     _ => {
        print!("Unknown Number inputted"); 
          println!("Please try again")
     }
    }
}