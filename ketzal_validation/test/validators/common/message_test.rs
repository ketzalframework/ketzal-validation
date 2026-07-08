use crate::model::message_model::MessageModel;

#[test]
fn test_custom_message_min() {
    let model = MessageModel {
        name: "ab".into(),
        email: None,
    };
    let result = model.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    let err = errors.errors.iter().find(|e| e.rule == "min").unwrap();
    assert_eq!(err.message, "Custom name min message");
}

#[test]
fn test_custom_message_min_with_email() {
    let model = MessageModel {
        name: "abc".into(),
        email: Some("ab".into()),
    };
    let result = model.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    let err = errors
        .errors
        .iter()
        .find(|e| e.field == "email" && e.rule == "min")
        .unwrap();
    assert_eq!(err.message, "The email needs at least 5 chars");
}

#[test]
fn test_custom_message_max() {
    let model = MessageModel {
        name: "abc".into(),
        email: Some("toolongemail@example.com".into()),
    };
    let result = model.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    let err = errors.errors.iter().find(|e| e.rule == "max").unwrap();
    assert_eq!(err.message, "The email must not exceed 20 chars");
}

#[test]
fn test_default_message_when_no_custom() {
    let model = MessageModel {
        name: "ab".into(),
        email: None,
    };
    let result = model.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .errors
            .iter()
            .any(|e| e.field == "name" && e.rule == "min")
    );
}
