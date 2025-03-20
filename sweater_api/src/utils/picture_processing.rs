use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use base64::Engine;
use base64::engine::general_purpose;
use regex::Regex;
use crate::utils::DATA_STORAGE_PATH;

pub fn get_images(text: &str) -> Option<Vec<String>> {
    let re = Regex::new("base64,(.+?)\"").unwrap();

    let base64_vec = re.captures_iter(text)
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .collect::<Vec<String>>();

    if base64_vec.is_empty() {
        None
    } else {
        Some(base64_vec)
    }
}

pub fn save_image(user_id: &str, pictures_str: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let path = image_path();

    let mut index = 0;
    for picture in pictures_str {
        let image_data = general_purpose::STANDARD.decode(picture).unwrap();
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let file_name = format!("{}_{}_{}.png", user_id, time, index);

        result.push(path.join(&file_name).to_str().unwrap().to_string());

        save_file(image_data, &path, &file_name);

        index += 1;
    }

    result
}

fn image_path() -> PathBuf {
    let storage_path = Path::new(DATA_STORAGE_PATH.as_str());

    if !storage_path.exists() {
        match std::fs::create_dir_all(&storage_path) {
            Ok(_) => println!("Successfully created the directory: {:?}", storage_path),
            Err(e) => println!("Failed to create directory: {}", e),
        }
    } else {
        println!("Directory already exists: {:?}", storage_path);
    }

    PathBuf::from(storage_path)
}

fn save_file(image_data: Vec<u8>, image_path: &PathBuf, file_name: &str) {
    let image_path = image_path.join(file_name);

    match File::create(&image_path) {
        Ok(mut file) => {
            file.write_all(image_data.as_slice()).unwrap();
        }
        Err(err) => {
            println!("Failed to save image: {}", err);
        }
    }
}

