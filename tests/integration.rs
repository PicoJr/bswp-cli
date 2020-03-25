#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::io::{Read, Write};
    use tempfile::Builder;

    #[test]
    fn test_stdin_stdout_missing_pattern() {
        let mut cmd = Command::cargo_bin("bswp-cli").unwrap();
        cmd.write_stdin("AAAA").assert().failure();
    }

    #[test]
    fn test_stdin_stdout_1_pattern() {
        let mut cmd = Command::cargo_bin("bswp-cli").unwrap();
        cmd.arg("-e")
            .arg("0x42,0xFF,2,0")
            .write_stdin("AAAA")
            .assert()
            .success()
            .stdout("BABA");
    }

    #[test]
    fn test_stdin_stdout_2_patterns() {
        let mut cmd = Command::cargo_bin("bswp-cli").unwrap();
        cmd.arg("-e")
            .arg("0x42,0xFF,2,0")
            .arg("-e")
            .arg("0x43,0xFF,2,1")
            .write_stdin("AAAA")
            .assert()
            .success()
            .stdout("BCBC");
    }

    #[test]
    fn test_input_file_stdout() {
        let temp_input = Builder::new()
            .prefix("input")
            .suffix(".bin")
            .tempfile()
            .unwrap();
        write!(temp_input.as_file(), "AAAA").unwrap();

        let mut cmd = Command::cargo_bin("bswp-cli").unwrap();
        cmd.arg("-e")
            .arg("0x42,0xFF,2,0")
            .arg("-i")
            .arg(temp_input.path())
            .assert()
            .success()
            .stdout("BABA");
    }

    #[test]
    fn test_stdin_output_file() {
        let mut temp_output = Builder::new()
            .prefix("output")
            .suffix(".bin")
            .tempfile()
            .unwrap();

        let mut cmd = Command::cargo_bin("bswp-cli").unwrap();
        cmd.arg("-e")
            .arg("0x42,0xFF,2,0")
            .arg("-o")
            .arg(temp_output.path())
            .write_stdin("AAAA")
            .assert()
            .success()
            .stdout(""); // nothing should be written to stdout
        let mut buf = String::new();
        temp_output.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "BABA");
    }

    #[test]
    fn test_input_file_output_file() {
        let temp_input = Builder::new()
            .prefix("input")
            .suffix(".bin")
            .tempfile()
            .unwrap();
        write!(temp_input.as_file(), "AAAA").unwrap();

        let mut temp_output = Builder::new()
            .prefix("output")
            .suffix(".bin")
            .tempfile()
            .unwrap();

        let mut cmd = Command::cargo_bin("bswp-cli").unwrap();
        cmd.arg("-e")
            .arg("0x42,0xFF,2,0")
            .arg("-i")
            .arg(temp_input.path())
            .arg("-o")
            .arg(temp_output.path())
            .assert()
            .success()
            .stdout(""); // nothing should be written to stdout
        let mut buf = String::new();
        temp_output.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "BABA");
    }
}
