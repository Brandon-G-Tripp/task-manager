use assert_cmd::Command;

pub fn setup() {
    // let cmd = Command::cargo_bin("clean")
    //     .unwrap()
    //     .current_dir(env!("CARGO_MANIFEST_DIR"));
    let output = std::process::Command::new("cargo")
        .arg("clean")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();

    let output = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
} 

pub fn create_test_config() -> String {
    let s = r#"path: tmp.yml
last_updated: 12345
    "#;

    s.to_string()
} 


#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    #[should_panic]
    fn test_setup_fail() {
        let cmd = Command::cargo_bin("task-manager-invalid").unwrap();
    } 

    // #[test]
    // fn test_after_setup() {
    //     // Setup will build binary 
    //     crate::tests_common::test_setup::setup();

    //     // Now we can assert it exists 
    //     assert!(Command::cargo_bin("task-manager").is_ok());
    // } 
}
