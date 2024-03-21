use notify_rust::Notification;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = "Esta é uma notificação de teste";

    loop {
        Notification::new()
            .icon("https://cdn.icon-icons.com/images/icon-icons.svg")
            .urgency(notify_rust::Urgency::Critical)
            .summary("Nova notificação")
            .body(&message)
            .show()?;gitr

        sleep(Duration::from_secs(60)).await;
    }
}