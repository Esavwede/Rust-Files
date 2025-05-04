use std::collections::HashMap;




fn main() {
    variables();
    operations();
    if_else();
    matching();
    match2();
    //loop1();
    while_loop();
    for_loop();
    strings();
    arrays();
    vectors();
    tuples();
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

fn match2()
{
    const END_VALUE: bool = false; 

    match END_VALUE 
    {
        true => {
            println!("End value is true")
        }

        false => 
        {
            print!("End value is false")
        }
    }
}


// fn loop1()
// {
//     let mut count = 1; 

//     loop 
//     {
//         println!("This loop will run until count > 8");
//         println!("Current Count: {}",count);

//         if count > 8 
//         {
//             println!("Current Count is greater that 8");
//         }

//         count += 1
//     }
// }


fn while_loop(){

    let mut age = 7; 

    while age < 18 {
        println!("Child is not yet an adult");
        age += 1
    }

    println!("Child is now an adult");
}


fn for_loop()
{
    for i in 1..6
    {
        println!("{}",i)
    }
}

fn strings()
{
    let mut name: &str = "name";
    println!("{}",name);

    name = "secondname";
    println!("{}",name);

}


fn arrays(){
    let mut letters = ['a','b','c','d','e'];

    println!("{} is the first character in the array ",letters[0]);
    for letter in letters 
    {
        println!("{}",letter)
    }

    letters[0] = 'z';

    println!("The first element in the array is now {}", letters[0]); 
    println!("The length of the letters array is {}", letters.len() );
}


fn vectors(){
    let mut numbers = vec![1,2,3,4,5];
    println!("Initial vector {:?}",numbers);

    numbers.push(6);
    println!("Final vector {:?}",numbers);

    println!("The values of my numbers vector are ");
    for number in &numbers 
    {
        println!("{}",number); 
    }

}


fn tuples()
{
    let my_tuple = ("1",1);

    println!("{}", my_tuple.0);
    println!("{}", my_tuple.1);

    let ( one_str, one_int) = get_user();
    println!("{} + {} = 2 ", one_str, one_int)
}


fn get_user() -> (String, i8)
{
    (String::from("1"), 1)
}
