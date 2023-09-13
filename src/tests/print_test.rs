use std::io::Cursor;
use std::io::prelude::*;

#[test]
fn test_print() {
    let input = "Hello, world!\n";
    let expected_output = "You entered: Hello, world!\n";

    let mut output = Vec::new();

    Cursor::new(input)
        .read_line(&mut String::new())
        .unwrap();

    {
        let mut handle = Cursor::new(&mut output);
        let _ = handle.write_all(input.as_bytes());
    }

    println!("Actual output: {:?}", String::from_utf8(output.clone()));
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}