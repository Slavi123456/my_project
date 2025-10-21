use anyhow::Result;
use my_project::structs::user::{
    StoredUser, User, UserProfile, validate_email, validate_name, validate_password,
};
use serde_json::json;

#[tokio::test]
async fn user_testing() -> Result<()> {
    ////////////////////////////////////////////////////////
    let user = User::new(
        "Joan".into(),
        "Doan".into(),
        "j@d.c".into(),
        "12345678".into(),
    )
    .expect("User creation failed");

    assert!(user.validate().is_ok());
    ////////////////////////////////////////////////////////
    assert!(User::new("".into(), "Doan".into(), "j@d.c".into(), "12345678".into()).is_err());
    assert!(
        User::new(
            "Joan".into(),
            "Doan".into(),
            "invalid".into(),
            "12345678".into()
        )
        .is_err()
    );
    assert!(User::new("Joan".into(), "Doan".into(), "j@d.c".into(), "1234".into()).is_err());
    assert!(User::new("".into(), "Doan".into(), "invalid".into(), "1234".into()).is_err());
    ////////////////////////////////////////////////////////
    //Password's lenght should be >= 2
    assert!(!validate_name(""));
    assert!(!validate_name("j"));
    assert!(validate_name("jo"));
    assert!(validate_name("joan"));

    //Email should not be empty and contain '@', '.'
    assert!(!validate_email(""));
    assert!(!validate_email("j"));
    assert!(!validate_email("j@d"));
    assert!(validate_email("j@d."));

    //Password's lenght should be >= 8
    assert!(!validate_password(""));
    assert!(!validate_password("j"));
    assert!(validate_password("joan1234"));
    ////////////////////////////////////////////////////////
    let data = json!(
    {
        "first_name": "John",
        "last_name": "Doe",
        "email": "john@doe.com",
        "password": "johnDoe123"
    });
    let parsed: User = serde_json::from_value(data).unwrap();
    let user = User::new("John", "Doe", "john@doe.com", "johnDoe123").unwrap();
    assert_eq!(parsed, user);

    Ok(())
}

#[tokio::test]
async fn stored_user() -> Result<()> {
    let user = User::new(
        "Joan".into(),
        "Doan".into(),
        "j@d.c".into(),
        "12345678".into(),
    )
    .unwrap();
    let stored_user = StoredUser::new(1, user).unwrap();

    assert_eq!(stored_user.user_id(), 1);

    let profile = stored_user.get_user_profile();
    assert_eq!(profile.first_name(), "Joan");
    assert_eq!(profile.last_name(), "Doan");
    assert_eq!(profile.email(), "j@d.c");
    // /////////////////////////////////////////////////////////////

    Ok(())
}

#[tokio::test]
async fn user_profile() -> Result<()> {
    let profile = UserProfile::new("John", "Doe", "john@doe.com").unwrap();

    let parsed = serde_json::to_value(profile).unwrap();

    let data = json!(
    {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john@doe.com",
    });

    assert_eq!(parsed, data);

    Ok(())
}
