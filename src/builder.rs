use crate::routes::{about, blog, buttons, diagnostic, guestbook, index, projects, not_found};
use dotenv::dotenv;
use reqwest::multipart::{Form, Part};
use std::{
    env,
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

pub const OUT_DIR: &str = "dist";

macro_rules! generate_page {
    ($module:ident, $filename:expr) => {
        let content = $module::page_html().await;
        fs::write(format!("{}/{}", OUT_DIR, $filename), content).unwrap();
    };
}

pub async fn build() -> Result<(), std::io::Error> {
    fs::create_dir_all(OUT_DIR)?;
    fs::create_dir_all(format!("{}/assets", OUT_DIR))?;
    fs::create_dir_all(format!("{}/blog", OUT_DIR))?;

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy("assets", OUT_DIR, &options)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    generate_page!(not_found, "not_found.html");

    generate_page!(index, "index.html");
    generate_page!(about, "about.html");
    generate_page!(buttons, "buttons.html");
    generate_page!(projects, "projects.html");
    generate_page!(blog, "blog/index.html");
    generate_page!(guestbook, "guestbook.html");

    // TODO: skip this step in publish mode
    generate_page!(diagnostic, "build.html");

    let posts = fs::read_dir("content/posts")?;
    for post in posts {
        let post = post?;
        let slug = post
            .file_name()
            .to_str()
            .unwrap()
            .trim_end_matches(".md")
            .to_owned();

        let content = blog::post_html(&slug).await;
        fs::write(format!("{}/blog/{}.html", OUT_DIR, &slug), content)?;
    }

    Ok(())
}

pub async fn publish(dist_dir: &str) -> Result<(), reqwest::Error> {
    let start_time = std::time::SystemTime::now();
    dotenv().ok();
    let api_endpoint = env::var("API_ENDPOINT").expect("API_ENDPOINT must be set");
    let api_key = env::var("API_KEY").expect("API_ENDPOINT must be set");
    let cookie = env::var("TOKEN").expect("CSRF_TOKEN must be set");

    let client = reqwest::Client::new();

    let _ = fs::remove_file("site.zip").unwrap();
    let _ = zip_dir(dist_dir, "site.zip");
    let zip = fs::read("site.zip").unwrap();

    let id_req = client
        .get(format!("{}/files/big/create", api_endpoint))
        .header("Authorization", &api_key)
        .send()
        .await?;

    let mut id = String::new();
    if !id_req.status().is_success() {
        eprintln!("Failed to retrieve bigfile")
    } else {
        println!("Id success");
        id = id_req
            .json::<HashMap<String, String>>()
            .await?
            .get("id")
            .unwrap()
            .to_owned();
    }
    let form = Form::new()
            .part("id", Part::text(id.clone()))
            .part("file", Part::bytes(zip).file_name("site.zip"));

    let file = client
        .post(format!("{}/files/big/append", api_endpoint))
        .header("Authorization", &api_key)
        .multipart(form)
        .send()
        .await?;

    if !file.status().is_success() {
        eprintln!("Failed to append")
    } else {
        println!("Append success")
    }

    let upload = client
        .post(format!("{}/files/import/{}", api_endpoint, id))
        .header("Authorization", &api_key)
        .send()
        .await?;

    if !upload.status().is_success() {
        eprintln!("Failed to upload")
    } else {
        println!("Upload success")
    }

    let csrf = client
        .get(format!("{}/csrf", api_endpoint))
        .header("Origin", "https://nekoweb.org")
        .header("Host", "nekoweb.org")
        .header("User-Agent", "ivrry.rs/0.1")
        .header("Cookie", format!("token={}", cookie))
        .send()
        .await?;

    let mut csrf_token: String = String::new();
    if !csrf.status().is_success() {
        eprintln!("Failed to retrieve csrf token: {}", csrf.status());
        eprintln!("response body: {}", csrf.text().await?);
    } else {
        println!("Csrf success");
        csrf_token = csrf.text().await?;
    }
    
    let form = Form::new()
        .part("csrf", Part::text(csrf_token))
        .part("site", Part::text("ivrry"))
        .part("pathname", Part::text("build.html"))
        .part("content", Part::text(diagnostic::page_html().await));

    let edit = client
        .post(format!("{}/files/edit", api_endpoint))
        .header("Origin", "https://nekoweb.org")
        .header("Host", "nekoweb.org")
        .header("User-Agent", "ivrry.rs/0.1")
        .header("Cookie", format!("token={}", cookie))
        .multipart(form)
        .send()
        .await?;
    
    if !edit.status().is_success() {
        eprintln!("Failed to edit, try checking token")
    } else {
        println!("Edit success")
    }

    let end_time = std::time::SystemTime::now().duration_since(start_time);
    println!("Done in {}s", end_time.unwrap().as_secs_f32());
    Ok(())
}

fn zip_dir(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    let path = Path::new(src_dir);
    let file = File::create(dst_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.strip_prefix(Path::new(src_dir)).unwrap();

        if path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let bytes = std::fs::read(path)?;
            let _ = zip.write_all(&bytes);
        } else if name.as_os_str().len() != 0 {
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;
    Ok(())
}