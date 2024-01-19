use assert_cmd::Command;

pub fn setup() {
    let mut cmd = Command::cargo_bin("task-manager").unwrap();
    cmd.assert().success(); // build binary
} 
