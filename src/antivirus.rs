mod exclusion;

use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
#[cfg(windows)]
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(unix)]
pub fn get_antivirus_property(_property_name: String, _include_inactive: bool) -> Result<()> {
    println!("None");
    Ok(())
}

#[cfg(windows)]
pub fn get_antivirus_property(property_name: String, include_inactive: bool) -> Result<()> {
    use wmi::Variant;
    use wmi::*;

    let wmi_con =
        WMIConnection::with_namespace_path("ROOT\\SecurityCenter2", COMLibrary::new()?.into())?;
    let query = format!("SELECT * FROM AntiVirusProduct");
    let products = wmi_con.raw_query(query)?;
    let mut is_first = true;
    for antivirus_product in products {
        let prod: HashMap<String, Variant> = antivirus_product;

        // Filter only active products
        if !include_inactive {
            let product_state = &prod["productState"];

            if let Variant::I8(value) = product_state {
                // magic number of antivirus state: https://mcpforlife.com/2020/04/14/how-to-resolve-this-state-value-of-av-providers/
                if value & 0b1000000000000 == 0 {
                    continue;
                }
            }
        }

        match property_name == "*" {
            true => {
                println!("{:#?}", prod)
            }
            _ => {
                let property_value = &prod[&property_name];

                if let Variant::String(value) = property_value {
                    if is_first {
                        is_first = false;
                    } else {
                        print!(", ");
                    }
                    print!("{}", value)
                }
            }
        }
    }
    Ok(())
}

pub fn get_cmd<'a>() -> Command<'a, str> {
    Command::new("get")
        .description("Get information about antivirus")
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
                Arg::with_name("include-inactive")
                    .short("i")
                    .long("include-inactive")
                    .help("Include all antivirus registration"),
            )
        })
        .runner(|_args, matches| {
            let property_name = matches.value_of("property").unwrap().to_string();
            let include_inactive = matches.is_present("include-inactive");
            match get_antivirus_property(property_name, include_inactive) {
                Err(error) => {
                    println!("Error: {:?}", error);
                }
                _ => {}
            };
            Ok(())
        })
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .add_cmd(exclusion::get_multi_cmd())
        .into_cmd("antivirus")
        // Optionally specify a description
        .description("Detection of Antivirus and handling exception registration.");

    return multi_cmd;
}
