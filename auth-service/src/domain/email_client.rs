use super::Email;

// This trait represents the interface that all concrete email_clients should implement.
#[async_trait::async_trait]
pub trait EmailClient {
    async fn send_email(
        &self, // the concrete implementation of the trait
        email: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), String>;
}
