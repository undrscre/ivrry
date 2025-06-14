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
use log::{info, error};

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
        error!("Failed to retrieve bigfile")
    } else {
        info!("Id success");
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
        error!("Failed to append")
    } else {
        info!("Append success")
    }

    let upload = client
        .post(format!("{}/files/import/{}", api_endpoint, id))
        .header("Authorization", &api_key)
        .send()
        .await?;

    if !upload.status().is_success() {
        error!("Failed to upload")
    } else {
        info!("Upload success")
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
        error!("Failed to retrieve csrf token: {}", csrf.status());
        error!("response body: {}", csrf.text().await?);
    } else {
        info!("Csrf success");
        csrf_token = csrf.text().await?;
    }
    
    let form = Form::new()
        .part("csrf", Part::text(csrf_token))
        .part("site", Part::text("ivrry"))
        .part("pathname", Part::text("aaaaaaaaaaaaaaaaaaaaaaaaaa.html"))
        .part("content", Part::text(format!("Mreoww mreow mreow hello! hello! hello update :) uppdaaaate this is an update update hello guys! hello my friends! this is an update! welcome to my update. {:#?}", std::time::SystemTime::UNIX_EPOCH)));

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
        error!("Failed to edit, try checking token")
    } else {
        info!("Edit success")
    }

    let end_time = std::time::SystemTime::now().duration_since(start_time);
    info!("Done in {}s", end_time.unwrap().as_secs_f32());
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