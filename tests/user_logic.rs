use anyhow::Result;
use my_project::structs::user::{StoredUser, User};

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
    assert!(!User::validate_name(""));
    assert!(!User::validate_name("j"));
    assert!(User::validate_name("jo"));
    assert!(User::validate_name("joan"));

    //Email should not be empty and contain '@', '.'
    assert!(!User::validate_email(""));
    assert!(!User::validate_email("j"));
    assert!(!User::validate_email("j@d"));
    assert!(User::validate_email("j@d."));

    //Password's lenght should be >= 8
    assert!(!User::validate_password(""));
    assert!(!User::validate_password("j"));
    assert!(User::validate_password("joan1234"));
    ////////////////////////////////////////////////////////
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
