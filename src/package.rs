use anyhow::Context;
use std::{fs, io};
use std::path::Path;
use std::io::Cursor;
use std::fs::File;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;

use tokio::runtime::Handle;
use crate::config::get_dist_path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn unzip(file_path: String, output_directory: String) -> Result<()> {
    let file_name = std::path::Path::new(&file_path);
    let file = fs::File::open(&file_name).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Add path prefix to extract the file
        let mut outpath = std::path::PathBuf::new();
        outpath.push(&output_directory);
        outpath.push(file_outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("* extracted: \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "* extracted: \"{}\" ({} bytes)",
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    Ok(())
}

pub fn unzip_strip_prefix(file_path: String, output_directory: String, strip_prefix: &str) -> Result<()> {
    let file_name = std::path::Path::new(&file_path);
    let file = fs::File::open(&file_name).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Add path prefix to extract the file
        let mut outpath = std::path::PathBuf::new();
        outpath.push(&output_directory);
        let stripped_file_outpath = file_outpath.strip_prefix(strip_prefix).unwrap();
        outpath.push(stripped_file_outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("* extracted: \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "* extracted: \"{}\" ({} bytes)",
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    Ok(())
}

pub fn untar_strip_prefix(file_path: String, output_directory: String, strip_prefix: &str) -> Result<()> {
    let tar_xz = File::open(file_path)?;
    let tar = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    archive.entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(strip_prefix)?.to_owned();
            let full_path = format!("{}/{}", output_directory, path.display().to_string());
            entry.unpack(&full_path)?;
            Ok(full_path.parse().unwrap())
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}

async fn fetch_url(url: String, output: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(output)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

async fn download_zip(url: String, output: String) -> Result<()> {
    if Path::new(&output).exists() {
        println!("Using cached archive: {}", output);
        return Ok(());
    }
    println!("Downloading {} to {}", url, output);
    fetch_url(url, output).await
}

pub fn download_package(package_url: String, package_archive: String) -> Result<()> {
    let handle = Handle::current().clone();
    let th = std::thread::spawn(move || {
        handle.block_on(download_zip(package_url, package_archive)).unwrap();
    });
    Ok(th.join().unwrap())
}

pub fn prepare_package(package_url: String, package_archive: String, output_directory: String) -> Result<()> {
    let package_archive = get_dist_path(package_archive);
    download_package(package_url, package_archive.clone());
    if !Path::new(&output_directory).exists() {
        unzip(package_archive, output_directory).unwrap();
    }
    Ok(())
}

pub fn prepare_package_strip_prefix(package_url: &str, package_archive: &str, output_directory: String, strip_prefix: &str) -> Result<()> {
    download_package(package_url.to_string(), package_archive.to_string());
    if !Path::new(&output_directory).exists() {
        let package_archive = package_archive.to_string();
        if package_archive.ends_with(".zip") {
            unzip_strip_prefix(package_archive, output_directory, strip_prefix).unwrap();
        } else {
            untar_strip_prefix(package_archive, output_directory, strip_prefix).unwrap();
        }
    }
    Ok(())
}

pub fn remove_package(package_archive: &str, output_directory: &str) -> Result<()> {
    if Path::new(package_archive).exists() {
        fs::remove_file(package_archive).with_context(|| format!("Unable to delete `{}`", package_archive))?;
    }
    if Path::new(output_directory).exists() {
        fs::remove_dir_all(output_directory).with_context(|| format!("Unable to delete `{}`", output_directory))?;
    }
    Ok(())
}
