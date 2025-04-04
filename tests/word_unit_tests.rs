
use hangman::word::read_lines;

#[test]
fn test_non_empty_read_lines_return()
{
    let file: &str = "tests/assets/words.txt";
    assert!(!read_lines(file).is_empty());
} 

#[test]
fn test_read_from_file()
{
    let file: &str = "tests/assets/words.txt";
    let s: String = read_lines(file).get(0).unwrap_or(&String::new()).to_string();
    assert_eq!("test", s);
}

#[test]
fn test_file_contains()
{
    let file: &str = "tests/assets/fruit_words.txt";
    assert!(read_lines(file).contains(&"apple".to_string()));
    assert!(read_lines(file).contains(&"pear".to_string()));
    assert!(read_lines(file).contains(&"orange".to_string()));
    assert!(read_lines(file).contains(&"mango".to_string()));
    assert!(read_lines(file).contains(&"strawberry".to_string()));
}