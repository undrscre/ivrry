// contexts/buttons.rs (or wherever this lives)
use headless_chrome::{Browser, LaunchOptionsBuilder};
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use minijinja::{context, value::Value};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::{fs, path::Path};
use log::{warn, debug, info};

#[derive(Serialize, Deserialize, Clone)]
pub struct Button {
    url: String,
    name: String,
    art: String,
    #[serde(default)]
    screenshot: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Buttons {
    friends: Vec<Button>,
}

pub async fn context() -> Value {
    let file = fs::read_to_string("content/data/buttons.json")
        .expect("Unable to read buttons.json");
    let mut buttons: Buttons =
        serde_json::from_str(&file).expect("JSON was not formatted correctly");

    for btn in buttons.friends.iter_mut() {
        let out = format!("assets/screenshots/{}.png", slug(&btn.name));
        if Path::new(&out).exists() {
            btn.screenshot = out.replace("assets/", "/assets/");
        }
    }

    let missing: Vec<Button> = buttons
        .friends
        .iter()
        .filter(|b| b.screenshot.is_empty())
        .cloned()
        .collect();

    if !missing.is_empty() {
        let total = missing.len();
        tokio::task::spawn_blocking(move || {
            let browser = match Browser::new(LaunchOptionsBuilder::default().args(vec![
            OsStr::new("--enable-features=DnsOverHttps"),
            OsStr::new("--dns-over-https-mode=secure"),
            OsStr::new("--dns-over-https-templates=https://cloudflare-dns.com/dns-query"),
        ]).build().unwrap()) {
                Ok(b) => b,
                Err(e) => {
                    warn!("couldn't launch chrome, skipping screenshots: {e}");
                    return;
                }
            };

            let ok_count = std::sync::atomic::AtomicUsize::new(0);
            let fail_count = std::sync::atomic::AtomicUsize::new(0);

            missing.into_par_iter().for_each(|btn| {
                let out = format!("assets/screenshots/{}.png", slug(&btn.name));
                match shoot(&browser, &btn.url, &out) {
                    Ok(_) => {
                        info!("shot {} -> {}", btn.url, out);
                        ok_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                    Err(e) => {
                        warn!("screenshot failed for {}: {e}", btn.url);
                        fail_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            });

            let ok = ok_count.load(std::sync::atomic::Ordering::Relaxed);
            let fail = fail_count.load(std::sync::atomic::Ordering::Relaxed);
            info!("screenshots done: {}/{} succeeded ({} failed)", ok, total, fail);
        });
    }

    context! { buttons => buttons }
}

fn shoot(browser: &Browser, url: &str, out: &str) -> anyhow::Result<()> {
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    // kill scrollbars before capture — otherwise you get chrome's native
    // scrollbar rendered right into the png, ugly as sin on a preview thumb
    tab.evaluate(
        "var s=document.createElement('style');
         s.innerHTML='::-webkit-scrollbar{display:none}body{scrollbar-width:none}';
         document.head.appendChild(s);",
        false,
    )?;

    let png = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;
    fs::create_dir_all("assets/screenshots")?;
    fs::write(out, png)?;
    Ok(())
}

fn slug(s: &str) -> String {
    s.to_lowercase().replace(' ', "-")
}