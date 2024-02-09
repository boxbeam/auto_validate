use auto_validate_macro::auto_validate;
use validator::ValidationErrors;

fn main() {
    validation_function("Test".into()).unwrap();
}

#[auto_validate]
fn validation_function(#[validate(url)] str: String) -> Result<(), ValidationErrors> {
    println!("{str}");
    Ok(())
}

#[auto_validate]
fn validation_function_with_lifetime(
    #[validate(email)] email: &str,
) -> Result<(), ValidationErrors> {
    println!("{email}");
    Ok(())
}

#[test]
fn test_validation() {
    assert!(validation_function("https://test.com".into()).is_ok());
    assert!(validation_function("/test::".into()).is_err());
    assert!(validation_function_with_lifetime("test@example.com").is_ok());
    assert!(validation_function_with_lifetime("@example.com").is_err());
}
