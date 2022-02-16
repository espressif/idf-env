mod exclusion;

use clap::{Arg, App};
#[cfg(windows)]
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(unix)]
pub fn get_antivirus_property(_property_name: String, _include_inactive: bool) -> Result<()> {
    println!("None");
    Ok(())
}

pub fn get_antivirus_name() -> Vec<String> {
    get_antivirus_property("displayName", false)
}

#[cfg(windows)]
fn get_antivirus_property(property_name: &str, include_inactive: bool) -> Vec<String> {
    use wmi::*;
    use wmi::Variant;

    let mut result:Vec<String> = Vec::new();
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\SecurityCenter2", COMLibrary::new().unwrap().into()).unwrap();
    let query = format!("SELECT * FROM AntiVirusProduct");
    let products = wmi_con.raw_query(query).unwrap();
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


        let property_value = &prod[property_name];

        if let Variant::String(value) = property_value {
            result.push(value.clone());
        }
}
    result
}

pub fn get_cmd<'a>() -> App<'a> {
    App::new("get")
        .about("Get information about antivirus")
        .arg(
            Arg::new("property")
                .short('p')
                .long("property")
                .help("Filter result for property name")
                .takes_value(true)
                .default_value("*"),
        )
        .arg(
            Arg::new("include-inactive")
                .short('i')
                .long("include-inactive")
                .help("Include all antivirus registration"),
        )

        // .runner(|_args, matches| {
        //     let property_name = matches.value_of("property").unwrap().to_string();
        //     let include_inactive = matches.is_present("include-inactive");
        //     match get_antivirus_property(property_name, include_inactive) {
        //         Err(error) => {
        //             println!("Error: {:?}", error);
        //         }
        //         _ => {}
        //     };
        //     Ok(())
        // })
}

pub fn get_multi_cmd<'a>() -> App<'a> {
    App::new("antivirus")
        .about("Detection of Antivirus and handling exception registration.")
        .subcommand(get_cmd())
        .subcommand(exclusion::get_multi_cmd())
}

