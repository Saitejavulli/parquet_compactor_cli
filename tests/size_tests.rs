use pcompact::size_parser::parse_size;

#[test]
fn test_parse_kb() {
    let value = parse_size("128KB").unwrap();
    assert_eq!(value, 128 * 1024);
}

#[test]
fn test_parse_mb() {
    let value = parse_size("128MB").unwrap();
    assert_eq!(value, 128 * 1024 * 1024);
}

#[test]
fn test_parse_gb() {
    let value = parse_size("1GB").unwrap();
    assert_eq!(value, 1024 * 1024 * 1024);
}

#[test]
fn test_invalid_size() {
    assert!(parse_size("12XYZ").is_err());
}