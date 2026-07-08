use crate::model::auth_model::RegisterRequest;

#[test]
fn test_confirmed_valid() {
    let req = RegisterRequest {
        password: "secret123".into(),
        password_confirmation: "secret123".into(),
    };
    assert!(req.validate().is_ok());
}

#[test]
fn test_confirmed_mismatch() {
    let req = RegisterRequest {
        password: "secret123".into(),
        password_confirmation: "different".into(),
    };
    let result = req.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.errors.iter().any(|e| e.field == "password" && e.rule == "confirmed"));
}

#[test]
fn test_confirmed_min_error() {
    let req = RegisterRequest {
        password: "short".into(),
        password_confirmation: "short".into(),
    };
    let result = req.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.errors.iter().any(|e| e.field == "password" && e.rule == "min"));
    assert!(!errors.errors.iter().any(|e| e.rule == "confirmed"));
}
