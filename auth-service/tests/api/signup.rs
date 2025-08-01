#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::helpers::{get_random_email, TestApp};
use auth_service::{ErrorResponse, SignupResponse};
use axum::http::StatusCode;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": "true"
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let bad_email = "bad_email_at_example.com".to_owned();
    let bad_password = "psword".to_owned();

    let test_cases = [
        serde_json::json!({
            "email": bad_email,
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": bad_password,
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let bad_email = "bad_email_at_example.com".to_owned();
    let bad_password = "psword".to_owned();
    let good_password = "password123".to_owned();

    let test_cases = [
        serde_json::json!({
            "email": random_email,
            "password": good_password,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": good_password,
            "requires2FA": true
        }),
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        let response = app.post_signup(test_case).await;
        let expected_error = match i {
            0 => assert_eq!(
                response.status().as_u16(),
                201,
                "Failed for input: {:?}",
                test_case
            ), // user added on first loop
            1 => {
                assert_eq!(response.status().as_u16(), StatusCode::CONFLICT);
                assert_eq!(
                    response
                        .json::<ErrorResponse>()
                        .await
                        .expect("Could not deserialize response body to ErrorResponse")
                        .error,
                    "User already exists".to_owned()
                );
            }
            _ => panic!("Unexpected test case"),
        };
    }
}

#[tokio::test]
async fn should_return_200_if_good_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [serde_json::json!({
        "email": get_random_email(),
        "password": "password123",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            201,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let request_body_parameters = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let expected_response = SignupResponse {
        message: "User created successfully".to_owned(),
    };

    let response = app.post_signup(&request_body_parameters).await;
    assert_eq!(response.status().as_u16(), 201);
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}
