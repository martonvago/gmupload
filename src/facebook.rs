use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::input::InsertTextParams;
use futures::StreamExt;
use chromiumoxide::Element;
use std::time::Duration;
use std::time::Instant;


pub async fn compose_post(
    text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_data_dir = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".grandma-facebook-chrome");
    
    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head()
            .user_data_dir(user_data_dir)
            .build()?,
    )
    .await?;

    let browser_task = tokio::spawn(async move {
        while handler.next().await.is_some() {}
    });

    let page = browser.new_page("https://www.facebook.com").await?;
    page.wait_for_navigation().await?;

    let Ok(create_post) = wait_for_element(
        &page,
        r#"div[aria-label="Create a post"]"#,
    ).await else {
        eprintln!("Could not find 'Create a post'. Grandma may need to log in.");
        // Wait until browser closed manually
        browser_task.await?;
        return Ok(());
    };
    create_post.click().await?;

    let editor = wait_for_element(
        &page,
        r#"div[role="textbox"]"#,
    ).await?;
    editor.click().await?;
    page.execute(InsertTextParams::new(text.to_string())).await?;

    // Wait until browser closed manually
    browser_task.await?;
    Ok(())
}

async fn wait_for_element(
    page: &chromiumoxide::Page,
    selector: &str,
) -> Result<Element, Box<dyn std::error::Error + Send + Sync>> {
    const TIMEOUT: Duration = Duration::from_secs(15);
    const POLL_INTERVAL: Duration = Duration::from_millis(200);
    let start = Instant::now();

    while start.elapsed() < TIMEOUT {
        if let Ok(element) = page.find_element(selector).await {
            return Ok(element);
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }

    Err(format!("Could not find element: {selector}").into())
}