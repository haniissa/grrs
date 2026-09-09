#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let test_data = std::io::Cursor::new(b"lorem ipsum\ndolor sit amet");

    lib::find_matches(test_data, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
