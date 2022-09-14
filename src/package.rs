use crate::config::{get_dist_path, get_tool_path};
use crate::emoji;
use anyhow::Context;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};
use tar::Archive;
use tokio::runtime::Handle;
use xz2::read::XzDecoder;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn unzip(file_path: &str, output_directory: &str) -> Result<()> {
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

        if file.name().ends_with('/') {
            // println!("* extracted: \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!(
            //     "* extracted: \"{}\" ({} bytes)",
            //     outpath.display(),
            //     file.size()
            // );
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

pub fn unzip_strip_prefix(
    file_path: &str,
    output_directory: &str,
    strip_prefix: &str,
) -> Result<()> {
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

        // Skip files in top level directories which are not under directory with prefix
        if !file_outpath.starts_with(strip_prefix) {
            println!("* skipped: \"{}\"", file_outpath.display());
            continue;
        }

        let stripped_file_outpath = file_outpath.strip_prefix(strip_prefix).unwrap();
        outpath.push(stripped_file_outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if file.name().ends_with('/') {
            if !Path::new(file.name()).exists() {
                println!("* created: \"{}\"", outpath.display());
                fs::create_dir_all(&outpath).unwrap();
            }
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

pub fn untarxz_strip_prefix(
    file_path: &str,
    output_directory: &str,
    strip_prefix: &str,
) -> Result<()> {
    let tar_xz = File::open(file_path)?;
    let tar = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(strip_prefix)?.to_owned();
            let full_path = format!("{}/{}", output_directory, path.display());
            entry.unpack(&full_path)?;
            Ok(full_path.parse().unwrap())
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}

pub fn untarxz(file_path: &str, output_directory: &str) -> Result<()> {
    let tar_xz = File::open(file_path)?;
    let tar = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.to_owned();
            let full_path = format!("{}/{}", output_directory, path.display());
            entry.unpack(&full_path)?;
            Ok(full_path.parse().unwrap())
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}

pub fn untargz_strip_prefix(
    file_path: &str,
    output_directory: &str,
    strip_prefix: &str,
) -> Result<()> {
    let tar_gz = File::open(file_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(strip_prefix)?.to_owned();
            let full_path = format!("{}/{}", output_directory, path.display());
            entry.unpack(&full_path)?;
            Ok(full_path.parse().unwrap())
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}

pub fn untargz(file_path: &str, output_directory: &str) -> Result<()> {
    let tar_gz = File::open(file_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.to_owned();
            let full_path = format!("{}/{}", output_directory, path.display());
            entry.unpack(&full_path)?;
            Ok(full_path.parse().unwrap())
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));
    Ok(())
}

async fn fetch_url(url: &str, output: &str) -> Result<()> {
    let response = reqwest::get(url).await;
    if let Ok(r) = response {
        let mut file = std::fs::File::create(output)?;
        let mut content = Cursor::new(r.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
    } else {
        return Err(format!("Download of {url} failed").into());
    };
    Ok(())
}

async fn download_zip(url: &str, output: &str) -> Result<()> {
    if Path::new(&output).exists() {
        println!("Using cached archive: {}", output);
        return Ok(());
    }
    println!("{} Downloading {} to {}", emoji::DOWNLOAD, url, output);
    fetch_url(url, output).await
}

pub fn download_package(package_url: String, package_archive: String) -> Result<()> {
    let handle = Handle::current();
    let th = std::thread::spawn(move || {
        handle
            .block_on(download_zip(&package_url, &package_archive))
            .unwrap();
    });
    th.join().unwrap();
    Ok(())
}

pub fn prepare_package(
    package_url: &str,
    package_archive: &str,
    output_directory: &str,
) -> Result<()> {
    if Path::new(&output_directory).exists() {
        println!(
            "{} Using cached directory: {}",
            emoji::INFO,
            output_directory
        );
        return Ok(());
    }

    let dist_path = get_dist_path("");
    if !Path::new(&dist_path).exists() {
        println!("{} Creating dist directory at {}", emoji::WRENCH, dist_path);
        fs::create_dir_all(&dist_path)?;
    }
    let package_archive = get_dist_path(package_archive);
    println!(
        "{} Downloading file {} from {}",
        emoji::DOWNLOAD,
        package_archive,
        package_url
    );
    download_package(package_url.to_string(), package_archive.to_string())?;

    println!("{} Extracting to {}", emoji::WRENCH, output_directory);
    let extension = Path::new(&package_archive)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    match extension {
        "zip" => {
            unzip(&package_archive, output_directory).unwrap();
        }
        "gz" => {
            match fs::create_dir_all(&output_directory) {
                Ok(_) => {
                    println!("Creating {} - Ok", output_directory);
                }
                Err(_e) => {
                    println!("Creating {} - Failed", output_directory);
                }
            }
            untargz(&package_archive, output_directory).unwrap();
        }
        "xz" => {
            untarxz(&package_archive, output_directory).unwrap();
        }
        _ => {
            println!("Unsuported file extension.");
        }
    }

    Ok(())
}

pub fn prepare_single_binary(
    package_url: &str,
    binary_name: &str,
    output_directory: &str,
) -> String {
    let tool_path = get_tool_path(output_directory);
    let binary_path = format!("{}/{}", tool_path, binary_name);

    if Path::new(&binary_path).exists() {
        println!("{} Using cached tool: {}", emoji::WARN, binary_path);
        return binary_path;
    }

    if !Path::new(&tool_path).exists() {
        println!("{} Creating tool directory: {}", emoji::WRENCH, tool_path);
        if let Err(_e) = fs::create_dir_all(&tool_path) {
            println!("{} Creating direcory {} failed", emoji::ERROR, tool_path);
        }
    }

    if let Err(_e) = download_package(package_url.to_string(), binary_path.to_string()) {
        println!("{} Download of {} failed", emoji::ERROR, package_url);
    }
    binary_path
}

pub fn prepare_package_strip_prefix(
    package_url: &str,
    package_archive: &str,
    output_directory: &str,
    strip_prefix: &str,
) -> Result<()> {
    if Path::new(&output_directory).exists() {
        println!("Using cached directory: {}", output_directory);
        return Ok(());
    }

    let dist_path = get_dist_path("");
    if !Path::new(&dist_path).exists() {
        println!("Creating dist directory: {}", dist_path);
        match fs::create_dir_all(&dist_path) {
            Ok(_) => {
                println!("Ok");
            }
            Err(_e) => {
                println!("Failed");
            }
        }
    }

    let package_archive = get_dist_path(package_archive);

    match download_package(package_url.to_string(), package_archive.to_string()) {
        Ok(_) => {
            println!("Downloaded");
        }
        Err(_e) => {
            println!("Unable to download package");
        }
    }
    if !Path::new(&output_directory).exists() {
        let extension = Path::new(package_archive.as_str())
            .extension()
            .unwrap()
            .to_str()
            .unwrap();

        match extension {
            "zip" => {
                unzip_strip_prefix(&package_archive, output_directory, strip_prefix).unwrap();
            }
            "gz" => {
                untargz_strip_prefix(&package_archive, output_directory, strip_prefix).unwrap();
            }
            "xz" => {
                untarxz_strip_prefix(&package_archive, output_directory, strip_prefix).unwrap();
            }
            _ => {
                println!("Unsuported file extension.");
            }
        }
    }
    Ok(())
}

pub fn remove_package(package_archive: &str, output_directory: &str) -> Result<()> {
    if Path::new(package_archive).exists() {
        fs::remove_file(package_archive)
            .with_context(|| format!("Unable to delete `{}`", package_archive))?;
    }
    if Path::new(output_directory).exists() {
        fs::remove_dir_all(output_directory)
            .with_context(|| format!("Unable to delete `{}`", output_directory))?;
    }
    Ok(())
}
