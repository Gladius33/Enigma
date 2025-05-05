#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_app_initialization() {
        let test_path = "test_data/enigma_init";
        if Path::new(test_path).exists() {
            fs::remove_dir_all(test_path).unwrap();
        }

        let result = EnigmaApp::init(test_path, "@testuser").await;
        assert!(result.is_ok());

        let app = result.unwrap();
        let username = app.local_user.read().unwrap().username.clone();
        assert_eq!(username, "@testuser");

        // Cleanup
        fs::remove_dir_all(test_path).unwrap();
    }
}
