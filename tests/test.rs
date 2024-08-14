use depclean::DepClean;

#[ignore]
#[test]
fn test_simple() {
    let root_path = std::env::current_dir().unwrap().join("tests/fixtures");
    let project_path = root_path.join("simple");

    let mut checker = DepClean::new();
    checker.run_with_path(&project_path);
}
