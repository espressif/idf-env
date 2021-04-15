use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
#[cfg(windows)]
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(unix)]
fn get_antivirus_property(_property_name: String) -> Result<()> {
    println!("None");
    Ok(())
}

#[cfg(windows)]
fn get_antivirus_property(property_name: String) -> Result<()> {
    use wmi::*;
    use wmi::Variant;

    let wmi_con = WMIConnection::with_namespace_path("ROOT\\SecurityCenter2", COMLibrary::new()?.into())?;
    let query = format!("SELECT {} FROM AntiVirusProduct", property_name);
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query(query).unwrap();
    for antivirus_product in results {
        match property_name == "*" {
            true => println!("{:#?}", antivirus_product),
            _ => {
                let property_value = &antivirus_product[&property_name];

                if let Variant::String(value) = property_value {
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
        })
        .runner(|_args, matches| {
            let property_name = matches.value_of("property").unwrap().to_string();
            get_antivirus_property(property_name).unwrap();
            Ok(())
        })
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .into_cmd("antivirus")

        // Optionally specify a description
        .description("Detection of Antivirus and handling exception registration.");

    return multi_cmd;
}
