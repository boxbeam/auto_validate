use auto_validate_macro::auto_validate;
use validator::{ValidationError, ValidationErrors};

fn main() {
    validation_function("Test".into()).unwrap();
}

#[auto_validate]
fn validation_function(#[validate(url)] str: &str) -> Result<(), ValidationErrors> {
    println!("{str}");
    Ok(())
}

#[test]
fn test_validation() {
    assert!(validation_function("https://test.com").is_ok());
    assert!(validation_function("/test::").is_err());
}
