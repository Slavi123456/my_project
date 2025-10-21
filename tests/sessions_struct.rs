use anyhow::Result;
use my_project::structs::session::Session;

#[tokio::test]
async fn sessions_testing() -> Result<()> {
    assert!(Session::new("1".to_string(), 1).is_ok());
    assert!(Session::new("".to_string(), 1).is_err());
    Ok(())
}
