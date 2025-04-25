fn longest_palindrome(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    println!("{}", s);
    println!("{:?}", chars);
    println!("{}", len);

    let mut result = String::new();
    
    // Iterate over all possible substrings
    for start in 0..len {
        for end in (start + 1..len + 1).rev() {
            let sub = &s[start..end];
            if is_palindrome(sub) && sub.len() > result.len() {
                result = sub.to_string();
            }
        }
    }
    result
}

// Function to check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    let rev: String = s.chars().rev().collect();
    s == rev
}

fn main() {
    println!("Hello, world!");

    // Call the longest_palindrome function
    let palindrome = longest_palindrome("babad");
    println!("Longest Palindrome: {}", palindrome);
}
