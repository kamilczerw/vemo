use crate::git::provider::CliProvider;

#[test]
fn init_should_return_provider() {
    let provider = CliProvider::init();

    assert!(provider.is_ok());
}

// #[test]
// fn
