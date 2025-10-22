use anyhow::Result;
use my_project::structs::{app_state::AppState, login::LoginInfo, user::User};

#[tokio::test]
async fn app_state_testing() -> Result<()> {
    let mut state = AppState::new_without_db().unwrap();

    /////////////////////////////////////////////////////////
    let user = User::new("John", "Doe", "j@d.c", "12345678").unwrap();
    let result = state.add_user(user.clone()).await;

    assert!(result.is_ok());
    assert_eq!(state.print_user_count().await, 1);

    //////////////////////////////////////////////////////////
    //Invalid find_user test
    assert!(
        state
            .find_user(LoginInfo::test_new_unchecked("g@d.c", "12345678"))
            .await
            .is_err()
    );
    assert!(
        state
            .find_user(LoginInfo::test_new_unchecked("j@d.c", "1234"))
            .await
            .is_err()
    );
    assert!(
        state
            .find_user(LoginInfo::test_new_unchecked("g@d.c", "1234"))
            .await
            .is_err()
    );
    //////////////////////////////////////////////////////////
    let valid_login = LoginInfo::new("j@d.c", "12345678").unwrap();
    let user_id = 0;
    assert_eq!(state.find_user(valid_login).await.unwrap(), user_id);

    //////////////////////////////////////////////////////////
    //Session chekcing

    //For now session_id = user_id
    let user_id: usize = 0;
    let session_id = user_id;
    let session_id_str = session_id.to_string();
    let invalid_session_id = 1.to_string();

    let session_result = state.add_session(user_id).await;
    assert!(session_result.is_ok());
    assert_eq!(state.print_session_count().await, 1);

    assert!(!state.is_session_valid(&invalid_session_id).await);
    assert!(state.is_session_valid(&session_id_str).await);

    //////////////////////////////////////////////////////////
    assert!(
        state
            .get_user_id_from_session(&invalid_session_id.clone())
            .await
            .is_err()
    );

    assert_eq!(
        state
            .get_user_id_from_session(&session_id_str)
            .await
            .unwrap(),
        session_id
    );
    //////////////////////////////////////////////////////////
    assert!(
        state
            .get_user_profile_from_session_id(&invalid_session_id)
            .await
            .is_err()
    );

    let result_user_profile = state
        .get_user_profile_from_session_id(&session_id_str)
        .await;
    assert!(result_user_profile.is_ok());
    assert_eq!(result_user_profile.unwrap(), user.get_user_profile());

    //////////////////////////////////////////////////////////
    state.delete_session(&invalid_session_id).await;
    assert_eq!(state.print_session_count().await, 1);

    state.delete_session(&session_id_str).await;
    assert_eq!(state.print_session_count().await, 0);

    Ok(())
}
