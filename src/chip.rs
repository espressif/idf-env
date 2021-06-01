use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};

fn get_identify_runner(_args: &str, matches: &clap::ArgMatches<'_>) -> std::result::Result<(), clap::Error> {
    let image_file = matches.value_of("image").unwrap().to_string();

    println!("Decoding: {}", image_file);
    let img = image::open(image_file).unwrap();

    // Use default decoder
    let decoder = bardecoder::default_decoder();

    let results = decoder.decode(&img);

    for result in results {
        println!("{}", result.unwrap());
    }

    Ok(())
}


pub fn get_indentify_cmd<'a>() -> Command<'a, str> {
    Command::new("identify")
        .description("Identify chip")
        .options(|app| {
            app.arg(
                Arg::with_name("image")
                    .short("i")
                    .long("image")
                    .help("Picture file with chip")
                    .takes_value(true)
            )
        })
        .runner(|_args, matches| get_identify_runner(_args, matches) )
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_indentify_cmd())
        .into_cmd("chip")

        // Optionally specify a description
        .description("Identify chip");

    return multi_cmd;
}
