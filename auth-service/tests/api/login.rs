use crate::helpers::TestApp;
use auth_service::{ErrorResponse, SignupResponse};
use uuid::Uuid;

#[tokio::test]
async fn login_should_return_200() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let good_password = "12345678".to_string();

    let login_body = serde_json::json!({
                "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_should_return_422_for_malformed_json_object() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let good_password = "12345678".to_string();

    let test_login_bodies = [
        serde_json::json!(
            {
                "password": good_password.clone(),
                "requires2FA": true
            }
        ),
        serde_json::json!(
            {
                "password": good_password.clone(),
                "requires2FA": true
            }
        ),
        serde_json::json!(
            {
                "email": random_email,
                "password": good_password.clone(),
            }
        ),
        serde_json::json!({}),
    ];

    for login_body in test_login_bodies.iter() {
        let response = app.post_login(login_body).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            response,
        );
    }
}

// tests an invalid email or an invalid password
#[tokio::test]
async fn login_should_return_400_for_invalid_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let bad_email = "bad_email_at_example.com".to_owned();
    let bad_email2 = "".to_owned();
    let good_password = "12345678".to_string();
    let bad_password = "psword".to_owned();
    let bad_password2 = "".to_owned();

    let test_login_bodies = [
        serde_json::json!(
            {
                "email": bad_email,
                "password": good_password.clone(),
                "requires2FA": true
            }
        ),
        serde_json::json!(
            {
                "email": bad_email2,
                "password": good_password.clone(),
                "requires2FA": true
            }
        ),
        serde_json::json!(
            {
                "email": random_email,
                "password": bad_password.clone(),
                "requires2FA": true
            }
        ),
        serde_json::json!(
            {
                "email": random_email,
                "password": bad_password2.clone(),
                "requires2FA": true
            }
        ),
    ];

    for login_body in test_login_bodies.iter() {
        let response = app.post_login(login_body).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            response,
        );
    }
}

// missing user, or password  not matching the database
#[tokio::test]
async fn login_should_return_401_for_valid_credentials_not_found_in_database() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let non_matching_email = "non_matching_email@example.com";
    let good_password = "12345678".to_string();
    let bad_password = "87654321".to_owned();

    // Signup a good user
    let good_signup_request_body_parameters = serde_json::json!({
        "email": random_email,
        "password": good_password,
        "requires2FA": true
    });

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    let response = app.post_signup(&good_signup_request_body_parameters).await;
    assert_eq!(response.status().as_u16(), 201);
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
    // end of signup a good user

    let test_login_bodies = [
        serde_json::json!(
            {
                "email": non_matching_email,
                "password": good_password.clone(),
                "requires2FA": true
            }
        ),
        serde_json::json!(
            {
                "email": random_email,
                "password": bad_password.clone(),
                "requires2FA": true
            }
        ),
    ];

    for login_body in test_login_bodies.iter() {
        let response = app.post_login(login_body).await;

        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            response,
        );
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", &Uuid::new_v4())
}
