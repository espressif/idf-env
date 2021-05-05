use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
#[cfg(windows)]
use std::collections::HashMap;

use crate::package::prepare_package;
use crate::config;

#[cfg(windows)]
use core::ptr::null_mut;

#[cfg(windows)]
pub mod windows;

use std::{thread, time};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(unix)]
fn get_driver_property(_property_name: String, _filter: String) -> Result<()> {
    Ok(())
}

#[cfg(windows)]
fn get_driver_property(property_name: String, filter: String) -> Result<()> {
    use wmi::*;
    use wmi::Variant;

    let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMV2", COMLibrary::new()?.into())?;
    let query = format!("SELECT {} FROM Win32_PnPEntity WHERE {}", property_name, filter);
    // println!("Query: {}", query);
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query(query).unwrap();

    for driver_item in results {
        match property_name == "*" {
            true => println!("{:#?}", driver_item),
            _ => {
                let property_value = &driver_item[&property_name];

                if let Variant::String(value) = property_value {
                    println!("{}", value)
                }
            }
        }
    }
    Ok(())
}

fn get_installed_driver_property(property_name: String) -> Result<()> {
    // Driver classes: https://docs.microsoft.com/en-us/windows-hardware/drivers/install/system-defined-device-setup-classes-available-to-vendors?redirectedfrom=MSDN
    return get_driver_property(property_name, "ClassGuid=\"{4d36e978-e325-11ce-bfc1-08002be10318}\"".to_string());
}

fn get_missing_driver_property(property_name: String) -> Result<()> {
    // https://stackoverflow.com/questions/11367639/get-a-list-of-devices-with-missing-drivers-using-powershell
    return get_driver_property(property_name, "ConfigManagerErrorCode>0".to_string());
}

pub fn get_cmd<'a>() -> Command<'a, str> {
    Command::new("get")
        .description("Get information about drivers")
        .options(|app| {
            app.arg(
                Arg::with_name("property")
                    .short("p")
                    .long("property")
                    .help("Filter result for property name")
                    .takes_value(true)
                    .default_value("*"),
            )
                .arg(
                    Arg::with_name("missing")
                        .short("m")
                        .long("missing")
                        .help("Display missing drivers")
                )
        })
        .runner(|_args, matches| {
            let property_name = matches.value_of("property").unwrap().to_string();
            if matches.is_present("missing") {
                get_missing_driver_property(property_name).unwrap();
            } else {
                get_installed_driver_property(property_name).unwrap();
            }
            Ok(())
        })
}

#[cfg(unix)]
fn install_driver(driver_inf: String, driver_url: String, _driver_archive: String) {}

use widestring::WideCString;
#[cfg(windows)]
fn install_driver(driver_inf: String) {
    // Reference: https://github.com/microsoft/Windows-driver-samples/tree/master/setup/devcon
    // SetupCopyOEMInf(SourceInfFileName,
    //     NULL,
    //     SPOST_PATH,
    //     0,
    //     DestinationInfFileName,
    //     ARRAYSIZE(DestinationInfFileName),
    //     NULL,
    //     &DestinationInfFileNameComponent))
    // Rust: https://docs.rs/winapi/0.3.9/winapi/um/setupapi/fn.SetupCopyOEMInfW.html
    let driver_inf = driver_inf.replace("/", "\\");
    print!("Installing driver with INF {} ", driver_inf);
    let mut destination_inf_filename_vec: Vec<winapi::um::winnt::WCHAR> = vec![0; 255];
    let destination_inf_filename:winapi::um::winnt::PWSTR = destination_inf_filename_vec.as_mut_ptr();
    let destination_inf_filename_len:winapi::um::winnt::FLONG = 250;
    let mut v: Vec<u16> = Vec::with_capacity(255);
    let mut a: winapi::um::winnt::PWSTR = v.as_mut_ptr();

    let source_inf_filename = WideCString::from_str(&driver_inf).unwrap();
    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/setupapi/nf-setupapi-setupcopyoeminfw
        let result = winapi::um::setupapi::SetupCopyOEMInfW(
            source_inf_filename.as_slice_with_nul().as_ptr(),
            null_mut(),
            winapi::um::setupapi::SPOST_PATH,
            winapi::um::setupapi::SP_COPY_NOOVERWRITE,
            destination_inf_filename,
            destination_inf_filename_len,
            null_mut(),
            &mut a as *mut _);
        let error_code = winapi::um::errhandlingapi::GetLastError();
        let destination_oem = WideCString::from_vec_with_nul(destination_inf_filename_vec).unwrap().to_string_lossy();
        if destination_oem.len() != 0 {
            print!("-> {} ", destination_oem);
        }
        print!("... ");

        match (result, error_code) {
            (1, 0) => { println!("Ok"); }
            (0, 2) => { println!("File not found"); }
            (0, 80) => { println!("Already installed"); }
            (0, 87) => { println!("Invalid parameter"); }
            (0, 122) => { println!("Insufficient buffer"); }
            (0, 1630) => { println!("Unsupported type"); }
            _ => {
                println!("Exit codes: {:#}, {:#}", result, error_code);
                println!("{:#?}", source_inf_filename);
            }
        }

    }
}

#[cfg(unix)]
fn get_install_runner(_args: &str, _matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    Ok(())
}

pub fn get_driver_path(driver_name:String) -> String {
    let drivers_path = config::get_tool_path("idf-driver".to_string());
    format!("{}/{}", drivers_path, driver_name)
}

#[cfg(unix)]
fn download_drivers(_args: &str, _matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    Ok(())
}

#[cfg(windows)]
fn download_drivers(_args: &str, _matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    if _matches.is_present("silabs") {
        prepare_package("https://www.silabs.com/documents/public/software/CP210x_Universal_Windows_Driver.zip".to_string(),
                        "cp210x.zip".to_string(),
                        get_driver_path("silabs-2021-05-03".to_string()));
    }
    if _matches.is_present("ftdi") {
        prepare_package("https://www.ftdichip.com/Drivers/CDM/CDM%20v2.12.28%20WHQL%20Certified.zip".to_string(),
                        "ftdi.zip".to_string(),
                        get_driver_path("ftdi-2021-05-03".to_string()));
    }
    if _matches.is_present("espressif") {
        prepare_package("https://dl.espressif.com/dl/idf-driver/idf-driver-esp32-c3-2021-04-21.zip".to_string(),
                        "idf-driver-esp32-c3.zip".to_string(),
                        get_driver_path("espressif-esp32-c3-2021-04-21".to_string()));
    }
    Ok(())
}


#[cfg(windows)]
fn get_install_runner(_args: &str, _matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {

    // Download drivers, if app is self-elevated this flag serves to avoid downloading in elevated mode.
    if !_matches.is_present("no-download") {
        download_drivers(_args, _matches);
    }

    if windows::is_app_elevated() {
        if _matches.is_present("silabs") {
            install_driver(get_driver_path("silabs-2021-05-03/silabser.inf".to_string()));
        }

        if _matches.is_present("ftdi") {
            install_driver(get_driver_path("ftdi-2021-05-03/ftdiport.inf".to_string()));
        }

        if _matches.is_present("espressif") {
            install_driver(get_driver_path("espressif-esp32-c3-2021-04-21/usb_jtag_debug_unit.inf".to_string()));
        }

        if _matches.is_present("wait") {
            println!("Process finished...");
            thread::sleep(time::Duration::from_millis(100000));
        }
    } else {
        if !windows::is_app_elevated() {
            windows::run_self_elevated_with_extra_argument("--no-download".to_string());
            return Ok(());
        }
    }
    Ok(())
}

pub fn get_install_cmd<'a>() -> Command<'a, str> {
    Command::new("install")
        .description("Install driver - requires elevated privileges")
        .options(|app| {
            app.arg(
                Arg::with_name("ftdi")
                    .short("f")
                    .long("ftdi")
                    .help("Install FTDI driver"),
            )
                .arg(
                    Arg::with_name("silabs")
                        .short("s")
                        .long("silabs")
                        .help("Install Silabs driver"),
                )
                .arg(
                    Arg::with_name("espressif")
                        .short("e")
                        .long("espressif")
                        .help("Install Espressif driver"),
                )
                .arg(
                    Arg::with_name("wait")
                        .short("w")
                        .long("wait")
                        .help("Wait after the installation for user confirmation"),
                )
                .arg(
                    Arg::with_name("no-download")
                        .short("x")
                        .long("no-download")
                        .help("Do not attempt to download files"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("m")
                        .long("verbose")
                        .takes_value(false)
                        .help("display diagnostic log after installation"))
        })
        .runner(|_args, matches| get_install_runner(_args, matches)
        )
}


pub fn get_download_cmd<'a>() -> Command<'a, str> {
    Command::new("download")
        .description("Download drivers")
        .options(|app| {
            app.arg(
                Arg::with_name("ftdi")
                    .short("f")
                    .long("ftdi")
                    .help("Install FTDI driver"),
            )
                .arg(
                    Arg::with_name("silabs")
                        .short("s")
                        .long("silabs")
                        .help("Install Silabs driver"),
                )
                .arg(
                    Arg::with_name("espressif")
                        .short("e")
                        .long("espressif")
                        .help("Install Espressif driver"),
                )

        })
        .runner(|_args, matches| download_drivers(_args, matches)
        )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .add_cmd(get_install_cmd())
        .add_cmd(get_download_cmd())
        .into_cmd("driver")

        // Optionally specify a description
        .description("Detection of Antivirus and handling exception registration.");

    return multi_cmd;
}
