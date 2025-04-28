use std::io;

fn display_menu() {
    println!("\nMAIN MENU");
    println!("------------------");
    println!("a) Learn Rust Facts");
    println!("b) Learn Rust Types");
    println!("c) Learn Rust I/O");
    println!("q) Quit Program");
    println!("------------------");
    println!("Enter your choice (a/b/c/q): ");
}

fn display_rust_facts() {
    let facts = vec![
        "Rust was originally designed by Graydon Hoare at Mozilla Research.",
        "Rust has been voted the 'most loved programming language' in Stack Overflow's annual survey since 2016.",
        "Rust's mascot is a cute crab named 'Ferris'.",
        "Rust guarantees memory safety without using a garbage collector.",
        "The name 'Rust' comes from a fungus that is robust, distributed, and parallel.",
        "Rust's package manager and build system is called 'Cargo'.",
        "Rust has a built-in testing framework.",
        "Rust's ownership system prevents data races at compile time.",
        "Rust can be used for WebAssembly development.",
        "Rust is used by companies like Microsoft, Amazon, and Google for system programming.",
    ];

    println!("\nRUST FACTS");
    println!("------------------");
    for (i, fact) in facts.iter().enumerate() {
        println!("Fact #{}: {}\n", i + 1, fact);
    }
    
    println!("Press Enter to return to main menu...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

fn play_type_game() {
    println!("Welcome to the Rust Type Learning Game!");
    println!("Your goal is to guess the correct Rust type for each value.");
    println!("Type your answer and press Enter. Let's begin!\n");

    loop {
        let questions = vec![
            ("\"Hello\"", "&str", "A string literal is a &str in Rust."),
            ("42", "i32", "Whole numbers without decimals are i32 by default."),
            ("3.14", "f64", "Numbers with decimals are f64 by default."),
            ("'a'", "char", "Single characters in single quotes are char."),
            ("true", "bool", "true or false values are bool."),
            ("[1, 2, 3]", "[i32; 3]", "A fixed-size array of i32 values."),
            ("String::from(\"test\")", "String", "A growable string is a String type."),
            ("vec![1, 2, 3]", "Vec<i32>", "A dynamic array is a Vec<T> in Rust."),
            ("(1, \"test\")", "(i32, &str)", "A tuple can hold multiple types."),
            ("Some(42)", "Option<i32>", "An optional value uses the Option<T> enum."),
            ("Ok(\"success\")", "Result<&str, E>", "A result type for error handling, where E is an error type."),
            ("{ x: 1, y: 2 }", "struct", "A custom struct with named fields."),
            ("&42", "&i32", "A reference to an i32 value."),
            ("&[1, 2, 3]", "&[i32]", "A slice of i32 values, borrowed from an array or Vec."),
            ("255u8", "u8", "An unsigned 8-bit integer, ranging from 0 to 255."),
            ("|x| x + 1", "fn(i32) -> i32", "A closure or function taking an i32 and returning an i32."),
            ("Box::new(42)", "Box<i32>", "A boxed i32 value, allocated on the heap."),
            ("std::fs::File", "File", "A type representing an open file, from the std::fs module."),
            ("b\"hello\"", "&[u8; 5]", "A byte string literal, represented as a fixed-size array of u8."),
            ("1_000_000", "i32", "Numeric literals can use underscores for readability, still an i32."),
        ];

        let mut score = 0;
        let total_questions = questions.len();

        for (index, (value, correct_type, hint)) in questions.iter().enumerate() {
            println!("Question {}: What is the type of `{}`?", index + 1, value);
            let mut input = String::new();
            
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let input = input.trim();
            
            if input == *correct_type {
                println!("Correct! {}", hint);
                score += 1;
            } else {
                println!(
                    "Incorrect. The type of `{}` is `{}`. {}",
                    value, correct_type, hint
                );
            }
            println!();
        }

        println!("Game Over!");
        println!("Your score: {}/{}", score, total_questions);
        if score == total_questions {
            println!("Perfect! You're a Rust type master!");
        } else if score >= total_questions / 2 {
            println!("Good job! Keep practicing those Rust types!");
        } else {
            println!("You'll get better with practice! Try again!");
        }

        println!("\nPress Enter to return to main menu...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        break;
    }
}

fn play_io_quiz() {
    println!("Welcome to the Rust I/O Learning Quiz!");
    println!("Test your knowledge about Rust's input/output operations.\n");

    let questions = vec![
        (
            "Which trait must be imported to use stdin()?",
            "std::io",
            "The std::io module provides I/O functionality"
        ),
        (
            "What method is used to read a line from stdin into a String?",
            "read_line",
            "read_line(&mut String) reads a line from standard input"
        ),
        (
            "How do you open a file for reading in Rust?",
            "File::open",
            "File::open() creates a new File instance for reading"
        ),
        (
            "Which method creates or opens a file for writing?",
            "File::create",
            "File::create() opens a file in write-only mode"
        ),
        (
            "What trait is required for reading from a file?",
            "Read",
            "The Read trait provides basic methods for reading bytes"
        ),
        (
            "What trait is required for writing to a file?",
            "Write",
            "The Write trait provides basic methods for writing bytes"
        ),
        (
            "Which method reads the entire contents of a file into a string?",
            "read_to_string",
            "read_to_string(&mut String) reads all contents into a string"
        ),
        (
            "What function can quickly write a string to a file?",
            "write_all",
            "write_all() writes all bytes to an output"
        ),
        (
            "Which type represents a buffered reader?",
            "BufReader",
            "BufReader<R> adds buffering to any reader"
        ),
        (
            "What happens if File::open fails to open a file?",
            "Result<T, E>",
            "It returns a Result type that must be handled"
        ),
    ];

    let mut score = 0;
    let total_questions = questions.len();

    for (index, (question, correct_answer, hint)) in questions.iter().enumerate() {
        println!("Question {}: {}", index + 1, question);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        
        if input.to_lowercase() == correct_answer.to_lowercase() {
            println!("Correct! {}", hint);
            score += 1;
        } else {
            println!(
                "Incorrect. The answer is {}. {}",
                correct_answer, hint
            );
        }
        println!();
    }

    println!("Quiz Complete!");
    println!("Your score: {}/{}", score, total_questions);
    if score == total_questions {
        println!("Perfect! You're a Rust I/O expert!");
    } else if score >= total_questions / 2 {
        println!("Good job! Keep learning about Rust's I/O operations!");
    } else {
        println!("Keep practicing! Rust I/O operations are important!");
    }

    println!("\nPress Enter to return to main menu...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

fn main() {
    println!("Welcome to RustLearner!");
    
    loop {
        display_menu();
        
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().to_lowercase().as_str() {
            "a" => display_rust_facts(),
            "b" => play_type_game(),
            "c" => play_io_quiz(),
            "q" => {
                println!("Thank you for using RustLearner! Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}