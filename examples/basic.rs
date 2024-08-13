use depcheck::DepChecker;

// Instruction:
// run `cargo run -example basic`
fn main() {
    let root_path = std::env::current_dir().unwrap().join("tests/fixtures");
    let project_path = root_path.join("simple");

    let mut checker = DepChecker::new();
    checker.run_with_path(&project_path);
}
