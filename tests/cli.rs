
use assert_cmd::cargo::*; 
use assert_fs::fixture::FileWriteStr as _;
//Imprt cargo_bin_cmd! macro andmethods
use predicates::prelude::*; // Used fo rwriting assertions

//local file
use grrs::find_matches;

//how can I add lib.rs here ?   
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let test_data = std::io::Cursor::new(b"lorem ipsum\ndolor sit amet");

    find_matches(test_data, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("grrs");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
#[test]
fn empty_string() -> Result<(), Box<dyn std::error::Error>>{
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    // file.write_str("");
    let mut cmd = cargo_bin_cmd!("grrs");

    cmd.arg("").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("pattern cannot be empty"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>>{
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));
    Ok(())
}