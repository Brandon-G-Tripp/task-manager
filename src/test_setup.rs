use assert_cmd::Command;

pub fn setup() {
    let mut cmd = Command::cargo_bin("task-manager").unwrap();
    cmd.assert().success(); // build binary
} 

pub fn create_test_config() -> String {
    let s = r#"
        path: tests/data/config.yml
        last_updated: 12345
    "#;

    s.to_string()
} 


