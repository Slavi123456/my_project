use anyhow::Result;
use my_project::structs::app_state::AppState;

#[tokio::test]
async fn app_state_testing() -> Result<()> {
    let state = AppState::new_without_db().unwrap();

    Ok(())
}
