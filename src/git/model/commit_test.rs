use crate::git::model::Commit;

#[test]
fn parse_git_line_should_return_commit_object() {
    let line = "0ee0b0041380df22675472392ab54ff835b07b48;Kamil Czerwiński;kamil@czerwinski.dev;Add 2 test apps;2022-06-25T20:57:13+02:00";
    let commit = Commit::from_line(line).unwrap();

    assert_eq!(commit.hash, "0ee0b0041380df22675472392ab54ff835b07b48");
    assert_eq!(commit.message, "Add 2 test apps");
    assert_eq!(commit.author.name, "Kamil Czerwiński");
    assert_eq!(commit.author.email, "kamil@czerwinski.dev");
    assert_eq!(commit.date, chrono::DateTime::parse_from_str("2022-06-25T20:57:13+02:00", "%Y-%m-%dT%H:%M:%S%z").unwrap());
}

#[test]
fn parse_git_line_should_return_error_when_line_is_invalid() {
    let line = "0ee0b0041380df22675472392ab54ff835b07b48;Kamil Czerwiński;";
    let commit = Commit::from_line(line);

    assert!(commit.is_err());
}
