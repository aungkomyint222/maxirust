use std::io::{self};
fn main(){
    println!("this is the 5 digit number");
    let a=5;
    let b=4;
    let c=3;
    let d=2;
    let e=1;
    if a < 0 {
        print!("{} is negative", a);
    } else if a > 0 {
        print!("{} is positive", a);
    } else {
        print!("{} is zero", a);
    }
    println!("{},{},{},{},{}",a,b,c,d,e);
    println!("What is muutable and immutable??");
    println!("Object that can be change after creation is called mutable and immutable is something that cannot be change is called immutable");
    let age=23;
    println!("You will be {} years old in the next 100 yeear",age+100);

    let lines = vec![
        "My name is aung ko myint",
        "I am 23 years old",
        "I want to become the greatest 1 million x programmer",
        "This is my resume",
        "I have work in SEO and digital marketing field",
        "I studied computer science and multimedia design ",
        "stop",
    ];
    
    for line in lines {
        println!("{}", line);  // This will print each line automatically
        
        // Wait for Enter key before continuing
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }

    println!("I am sleeping now for 35 lines of code lol");
    println!("today is day 2 i want to do sth lets do sth counter when a user click 
    enter it will count one two three four five ");
    let mut counter = 0;
    println!("Press Enter to count, Ctrl+C to exit");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Check if the input is just Enter (empty line)
        if input.trim().is_empty() {
            counter += 1;
            println!("Count: {}", counter);
        }
    }
}