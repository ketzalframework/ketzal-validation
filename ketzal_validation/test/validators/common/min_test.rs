use crate::model::users_model::User;

#[test]
fn test_min_string_valid() {
    let user = User {
        name: "Ana".into(),
        midelnames: None,
        email: "ana@example.com".into(),
    };
    assert!(user.validate().is_ok());
}

#[test]
fn test_min_name_insufficient() {
    let user = User {
        name: "An".into(),
        midelnames: None,
        email: "user@example.com".into(),
    };
    let result = user.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field == "name" && e.rule == "min")
    );
}

#[test]
fn test_min_email_insufficient() {
    let user = User {
        name: "Ana".into(),
        midelnames: None,
        email: "abc".into(),
    };
    let result = user.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field == "email" && e.rule == "min")
    );
}

#[test]
fn test_min_optional_some_insufficient() {
    let user = User {
        name: "Ana".into(),
        midelnames: Some("M".into()),
        email: "ana@example.com".into(),
    };
    let result = user.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field == "midelnames" && e.rule == "min")
    );
}
