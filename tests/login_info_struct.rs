use anyhow::Result;
use my_project::structs::login::LoginInfo;
use serde_json::json;

#[tokio::test]
async fn login_info() -> Result<()> {
    let json = json!({
        "email": "j@d.c",
        "password": "12345678"
    });
    let info: LoginInfo = serde_json::from_value(json).unwrap();
    let login = LoginInfo::new("j@d.c", "12345678").unwrap();
    assert_eq!(info, login);
    Ok(())
}
