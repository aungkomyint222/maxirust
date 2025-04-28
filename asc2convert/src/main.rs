use std::io;

fn fontoasc2(input: char) {
   
    if input == 'a' {
        println!();
        println!("  *****  ");
        println!(" *     * ");
        println!("*       *");
        println!("*********");
        println!("*       *");
        println!("*       *");
    } else if input == 'b' {
        println!();
        println!("******  ");
        println!("*     * ");
        println!("******  ");
        println!("*     * ");
        println!("*     * ");
        println!("******  ");
    } else if input == 'c' {
        println!();
        println!(" ***** ");
        println!("*      ");
        println!("*      ");
        println!("*      ");
        println!("*      ");
        println!(" ***** ");
    } else if input == 'd' {
        println!();
        println!("******  ");
        println!("*     * ");
        println!("*     * ");
        println!("*     * ");
        println!("*     * ");
        println!("******  ");
    } else if input == 'e' {
        println!();
        println!("*******");
        println!("*      ");
        println!("*****  ");
        println!("*      ");
        println!("*      ");
        println!("*******");
    } else if input == 'f' {
        println!();
        println!("*******");
        println!("*      ");
        println!("*****  ");
        println!("*      ");
        println!("*      ");
        println!("*      ");
    } else if input == 'g' {
        println!();
        println!(" ***** ");
        println!("*      ");
        println!("*  ****");
        println!("*     *");
        println!("*     *");
        println!(" ***** ");
    } else if input == 'h' {
        println!();
        println!("*     *");
        println!("*     *");
        println!("*******");
        println!("*     *");
        println!("*     *");
        println!("*     *");
    } else if input == 'i' {
        println!();
        println!("*******");
        println!("   *   ");
        println!("   *   ");
        println!("   *   ");
        println!("   *   ");
        println!("*******");
    } else if input == 'j' {
        println!();
        println!("*******");
        println!("    *  ");
        println!("    *  ");
        println!("    *  ");
        println!("*   *  ");
        println!(" ***   ");
    } else if input == 'k' {
        println!();
        println!("*    * ");
        println!("*   *  ");
        println!("****   ");
        println!("*   *  ");
        println!("*    * ");
        println!("*     *");
    } else if input == 'l' {
        println!();
        println!("*      ");
        println!("*      ");
        println!("*      ");
        println!("*      ");
        println!("*      ");
        println!("*******");
    } else if input == 'm' {
        println!();
        println!("*     *");
        println!("**   **");
        println!("* * * *");
        println!("*  *  *");
        println!("*     *");
        println!("*     *");
    } else if input == 'n' {
        println!();
        println!("*     *");
        println!("**    *");
        println!("* *   *");
        println!("*  *  *");
        println!("*   * *");
        println!("*    **");
    } else if input == 'o' {
        println!();
        println!(" ***** ");
        println!("*     *");
        println!("*     *");
        println!("*     *");
        println!("*     *");
        println!(" ***** ");
    } else if input == 'p' {
        println!();
        println!("****** ");
        println!("*     *");
        println!("****** ");
        println!("*      ");
        println!("*      ");
        println!("*      ");
    } else if input == 'q' {
        println!();
        println!(" ***** ");
        println!("*     *");
        println!("*     *");
        println!("*   * *");
        println!("*    * ");
        println!(" **** *");
    } else if input == 'r' {
        println!();
        println!("****** ");
        println!("*     *");
        println!("****** ");
        println!("*   *  ");
        println!("*    * ");
        println!("*     *");
    } else if input == 's' {
        println!();
        println!(" ***** ");
        println!("*      ");
        println!(" ***** ");
        println!("      *");
        println!("*     *");
        println!(" ***** ");
    } else if input == 't' {
        println!();
        println!("*******");
        println!("   *   ");
        println!("   *   ");
        println!("   *   ");
        println!("   *   ");
        println!("   *   ");
    } else if input == 'u' {
        println!();
        println!("*     *");
        println!("*     *");
        println!("*     *");
        println!("*     *");
        println!("*     *");
        println!(" ***** ");
    } else if input == 'v' {
        println!();
        println!("*     *");
        println!("*     *");
        println!("*     *");
        println!(" *   * ");
        println!("  * *  ");
        println!("   *   ");
    } else if input == 'w' {
        println!();
        println!("*     *");
        println!("*     *");
        println!("*  *  *");
        println!("* * * *");
        println!("**   **");
        println!("*     *");
    } else if input == 'x' {
        println!();
        println!("*     *");
        println!(" *   * ");
        println!("  ***  ");
        println!("  ***  ");
        println!(" *   * ");
        println!("*     *");
    } else if input == 'y' {
        println!();
        println!("*     *");
        println!(" *   * ");
        println!("  * *  ");
        println!("   *   ");
        println!("   *   ");
        println!("   *   ");
    } else if input == 'z' {
        println!();
        println!("*******");
        println!("     * ");
        println!("   *   ");
        println!(" *     ");
        println!("*      ");
        println!("*******");
    }
}

fn main() {
    loop {  // Add infinite loop
        // Print the initial pattern
        for _ in 0..18 {
            print!("*");
        }
        println!();
        println!("Convert chars (a to z to asc2)* or type 'exit' to quit");
       
        // Read input from user
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Check for exit condition
        if input.trim().to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        // Process each character in the input
        for c in input.trim().chars() {
            if c.is_ascii_alphabetic() {
                fontoasc2(c.to_ascii_lowercase());
            }
        }
        
        println!("\n"); // Add some spacing between iterations
    }
}