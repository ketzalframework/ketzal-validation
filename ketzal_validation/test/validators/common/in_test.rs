use crate::model::in_model::InModel;

#[test]
fn test_in_valid() {
    let model = InModel {
        role: "admin".into(),
        status: None,
    };
    assert!(model.validate().is_ok());
}

#[test]
fn test_in_valid_guest() {
    let model = InModel {
        role: "guest".into(),
        status: None,
    };
    assert!(model.validate().is_ok());
}

#[test]
fn test_in_invalid() {
    let model = InModel {
        role: "superadmin".into(),
        status: None,
    };
    let result = model.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.errors.iter().any(|e| e.rule == "in"));
}

#[test]
fn test_not_in_valid() {
    let model = InModel {
        role: "admin".into(),
        status: Some("active".into()),
    };
    assert!(model.validate().is_ok());
}

#[test]
fn test_not_in_invalid() {
    let model = InModel {
        role: "admin".into(),
        status: Some("banned".into()),
    };
    let result = model.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.errors.iter().any(|e| e.rule == "not_in"));
}

#[test]
fn test_not_in_none() {
    let model = InModel {
        role: "admin".into(),
        status: None,
    };
    assert!(model.validate().is_ok());
}
