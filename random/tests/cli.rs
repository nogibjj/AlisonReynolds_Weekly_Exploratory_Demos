use assert_cmd::Command;

#[test]
//a test to invoke the cli with an subcommand 'flip'
fn flipping1() {
    let mut cmd = Command::cargo_bin("random").unwrap();
    cmd.arg("flip");
    cmd.assert().success();
}

//a test to invoke the cli with an subcommand 'flip'
#[test]
fn flipping10() {
    let mut cmd = Command::cargo_bin("random").unwrap();
    cmd.arg("flip-n").arg("10");
    cmd.assert().success();
}
