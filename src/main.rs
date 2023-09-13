use std::io;

fn main() {
    println!("Please enter some text: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_print() {
        // Test code goes here
    }

    // Import the print_test module
    mod print_test;
}
