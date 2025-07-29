#[cfg(test)]
mod droplet_tests {
    use reqwest;
    use serde_json::json;

    const DROPLET_BASE_URL: &str = "https://axum.gardenway.org";

    #[tokio::test]
    async fn test_droplet_auth_signup_route() {
        let client = reqwest::Client::new();

        // Generate a unique email for this test
        let test_email = format!("test+{}@example.com", uuid::Uuid::new_v4());

        let signup_data = json!({
            "email": test_email,
            "password": "test_password_123",
            "requires2FA": false
        });

        let response = client
            .post(&format!("{}/auth/signup", DROPLET_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&signup_data)
            .send()
            .await
            .expect("Failed to send request to droplet");

        // Check that we get a 201 Created response
        assert_eq!(
            response.status(),
            201,
            "Expected 201 Created from droplet signup"
        );

        // Check that we get the expected JSON response
        let response_text = response.text().await.expect("Failed to get response text");
        let expected_message = r#"{"message":"User created successfully"}"#;
        assert_eq!(response_text, expected_message, "Unexpected response body");
    }

    #[tokio::test]
    async fn test_droplet_auth_signup_duplicate_user() {
        let client = reqwest::Client::new();

        // Use a consistent email for duplicate testing
        let test_email = "duplicate_test@example.com";

        let signup_data = json!({
            "email": test_email,
            "password": "test_password_123",
            "requires2FA": false
        });

        // First signup - should succeed
        let first_response = client
            .post(&format!("{}/auth/signup", DROPLET_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&signup_data)
            .send()
            .await
            .expect("Failed to send first request to droplet");

        // Should get 201 or 500 (if user already exists from previous test runs)
        assert!(
            first_response.status() == 201 || first_response.status() == 500,
            "First signup should be 201 (success) or 500 (already exists)"
        );

        // Second signup with same email - should fail
        let second_response = client
            .post(&format!("{}/auth/signup", DROPLET_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&signup_data)
            .send()
            .await
            .expect("Failed to send second request to droplet");

        // Should get 500 for duplicate user
        assert_eq!(
            second_response.status(),
            500,
            "Duplicate signup should return 500"
        );
    }

    #[tokio::test]
    async fn test_droplet_auth_signup_invalid_data() {
        let client = reqwest::Client::new();

        let invalid_data = json!({
            "email": "not-an-email",
            "password": "",
            // missing requires2FA field
        });

        let response = client
            .post(&format!("{}/auth/signup", DROPLET_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&invalid_data)
            .send()
            .await
            .expect("Failed to send request to droplet");

        // Should get 422 for invalid data
        assert_eq!(response.status(), 422, "Invalid data should return 422");
    }

    #[tokio::test]
    async fn test_droplet_ssl_certificate() {
        let client = reqwest::Client::new();

        // This test verifies that the SSL certificate is valid
        let response = client
            .get(&format!("{}/auth/hello", DROPLET_BASE_URL))
            .send()
            .await
            .expect("Failed to connect to droplet - SSL certificate may be invalid");

        // If we can make the request without SSL errors, the certificate is valid
        // The actual response status doesn't matter as much as the SSL handshake succeeding
        assert!(
            response.status().is_success() || response.status().is_client_error(),
            "SSL handshake should succeed even if route returns error"
        );
    }
}
