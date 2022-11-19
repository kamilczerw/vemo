use crate::git::provider::cli::Provider;

#[test]
fn init_should_return_provider() {
    let provider = Provider::init();

    assert!(provider.is_ok());
}
