use my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}