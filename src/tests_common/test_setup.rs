use assert_cmd::Command;

pub fn setup() {
    let mut cmd = Command::cargo_bin("task-manager").unwrap();
    cmd.assert().success(); // build binary
} 

pub fn create_test_config() -> String {
    let s = r#"path: tmp.yml
last_updated: 12345
    "#;

    s.to_string()
} 


#[test]
fn test_setup() {
    crate::tests_common::test_setup::setup();

    // make assertions 
    assert!(Command::cargo_bin("task-manager").is_ok());
} 

#[test]
#[should_panic]
fn test_setup_fail() {
    let mut cmd = Command::cargo_bin("task-manager-invalid").unwrap();
} 

#[test]
fn test_after_setup() {
    // Setup will build binary 
    crate::tests_common::test_setup::setup();

    // Now we can assert it exists 
    assert!(Command::cargo_bin("task-manager").is_ok());
} 
