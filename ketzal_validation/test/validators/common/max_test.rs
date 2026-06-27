use crate::model::users_model::User;

#[test]
fn test_max_string_valid() {
    let user = User {
        name: "Juan".into(),
        midelnames: None,
        email: "juan@example.com".into(),
    };
    assert!(user.validate().is_ok());
}

#[test]
fn test_max_name_exceeded() {
    let mut user = User {
        name: "Juan".into(),
        midelnames: None,
        email: "juan@example.com".into(),
    };
    user.name = "12345678901".into();
    let result = user.validate();
    assert!(result.is_err());
}

#[test]
fn test_max_email_exceeded() {
    let user = User {
        name: "Ana".into(),
        midelnames: None,
        email: "a".repeat(51),
    };
    let result = user.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field == "email" && e.rule == "max")
    );
}

#[test]
fn test_max_optional_some_exceeded() {
    let user = User {
        name: "Ana".into(),
        midelnames: Some("MMM".repeat(7)),
        email: "ana@example.com".into(),
    };
    let result = user.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field == "midelnames" && e.rule == "max")
    );
}
