use assert_cmd::Command;

fn get_bin() -> Command {
    Command::new("./test_target/debug/ece461_team19_cli")
}

#[test]
fn working() {}

#[test]
fn no_arguments() {
    let mut cmd = get_bin();
    cmd.assert().failure();
}

#[test]
fn bad_arguments() {
    let mut cmd1 = get_bin();
    cmd1.arg("what").assert().failure();

    let mut cmd2 = get_bin();
    cmd2.arg("url").assert().failure();

    let mut cmd3 = get_bin();
    cmd3.arg("report").assert().failure();
}

#[test]
fn bad_file_name() {
    let mut cmd = get_bin();
    cmd.args(["url", "notafile.osu"]).assert().failure();
}

#[test]
fn empty_file() {
    let mut cmd = get_bin();
    cmd.args(["url", "tests/empty.txt"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn new_lines() {
    let mut cmd = get_bin();
    cmd.args(["url", "tests/newline.txt"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn bad_url() {
    let mut cmd = get_bin();
    cmd.args(["url", "tests/badurl.txt"]).assert().failure();
}

#[test]
fn good_urls() {
    let mut cmd = get_bin();
    cmd.args(["url", "tests/url.txt"]).assert().success();
}
