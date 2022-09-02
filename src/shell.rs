use crate::emoji;
use clap::Arg;
use clap_nested::{Command, Commander, MultiCommand};
#[cfg(windows)]
use std::io::Write;
use std::process::Stdio;

#[cfg(windows)]
pub fn run_command(
    shell: &str,
    arguments: Vec<String>,
    command: String,
) -> std::result::Result<std::process::Output, clap::Error> {
    // println!("arguments = {:?}", arguments);
    let mut child_process = std::process::Command::new(shell)
        .args(&arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    {
        let child_stdin = child_process.stdin.as_mut().unwrap();
        child_stdin.write_all(&*command.into_bytes())?;
        // Close stdin to finish and avoid indefinite blocking
        drop(child_stdin);
    }
    let output = child_process.wait_with_output()?;
    if !output.status.success() {
        println!(
            "{} Command {} with args {:?} failed. Output: {:#?}",
            emoji::ERROR,
            shell,
            arguments,
            output
        );
        return Err(clap::Error::with_description(
            "Command failed",
            clap::ErrorKind::InvalidValue,
        ));
    }
    Ok(output)
}

#[cfg(unix)]
pub fn run_command(
    shell: &str,
    arguments: Vec<String>,
    command: String,
) -> std::result::Result<std::process::Output, clap::Error> {
    // Unix - pass command as parameter for initializer
    let mut arguments = arguments;
    if !command.is_empty() {
        arguments.push(command);
    }

    // println!("arguments = {:?}", arguments);
    let child_process = std::process::Command::new(shell)
        .args(&arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    {}
    let output = child_process.wait_with_output()?;
    if !output.status.success() {
        println!(
            "{} Command {} with args {:?} failed. Output: {:#?}",
            emoji::ERROR,
            shell,
            arguments,
            output
        );
        return Err(clap::Error::with_description(
            "Command failed",
            clap::ErrorKind::InvalidValue,
        ));
    }
    Ok(output)
}

#[cfg(windows)]
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

#[cfg(windows)]
pub fn set_env_variable(key: &str, value: String) {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
    env.set_value(key, &value).unwrap();
    // It's necessary to notify applications about update of the environment
    // https://stackoverflow.com/questions/19705401/how-to-set-system-environment-variable-in-c/19705691#19705691
    let param = wide_null("Environment").as_ptr() as winapi::shared::minwindef::LPARAM;
    unsafe {
        winapi::um::winuser::SendNotifyMessageW(
            winapi::um::winuser::HWND_BROADCAST,
            winapi::um::winuser::WM_SETTINGCHANGE,
            0,
            param,
        );
    }
}

#[cfg(windows)]
fn append_path(original_path: &str, new_path: &str) -> String {
    if original_path.is_empty() {
        return new_path.to_string();
    }

    // Make sure that only proper path delimeters are used for the platform
    #[cfg(windows)]
    let normalized_path = new_path.replace("/", "\\");
    #[cfg(windows)]
    let new_path = normalized_path.as_str();

    if original_path.contains(new_path) {
        return original_path.to_string();
    }

    if !original_path.ends_with(';') {
        return format!("{};{};", original_path, new_path);
    }

    format!("{}{};", original_path, new_path)
}

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    use crate::shell::append_path;

    #[test]
    #[cfg(windows)]
    fn test_append_path() {
        assert_eq!(append_path("", ""), "");
        assert_eq!(append_path("a", ""), "a");
        assert_eq!(append_path("a", "b"), "a;b;");
        assert_eq!(append_path("", "b"), "b");
        assert_eq!(append_path("a;b;", "b"), "a;b;");
        assert_eq!(append_path("a;c;", "b"), "a;c;b;");
    }
}

#[cfg(windows)]
pub fn update_env_variable(variable_name: &str, value: &str) {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey("Environment").unwrap();
    let env_path: String = env.get_value(variable_name).unwrap();
    let updated_env_path = append_path(env_path.as_str(), value);
    set_env_variable(variable_name, updated_env_path);
}

#[cfg(windows)]
pub fn update_env_path(value: &str) {
    update_env_variable("PATH", value);
}

#[cfg(unix)]
pub fn update_env_variable(_variable_name: &str, _value: &str) {
    todo!();
}

#[cfg(unix)]
pub fn update_env_path(_value: &str) {
    todo!();
}

// #[cfg(unix)]
// pub fn set_env_variable(_key: &str, _value: &str) {
//     todo!();
// }

pub fn get_cmd<'a>() -> Command<'a, str> {
    Command::new("append")
        .description("Append path to environment variable")
        .options(|app| {
            app.arg(
                Arg::with_name("variable")
                    .short("v")
                    .long("variable")
                    .help("Name of environment variable")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .help("Path which should be added to the variable")
                    .takes_value(true),
            )
        })
        .runner(|_args, matches| {
            let variable_name = matches.value_of("variable").unwrap().to_string();
            let path_value = matches.value_of("path").unwrap().to_string();
            update_env_variable(variable_name.as_str(), path_value.as_str());
            Ok(())
        })
}

pub fn get_multi_cmd<'a>() -> MultiCommand<'a, str, str> {
    let multi_cmd: MultiCommand<str, str> = Commander::new()
        .add_cmd(get_cmd())
        .into_cmd("shell")
        // Optionally specify a description
        .description("Detection of Antivirus and handling exception registration.");

    multi_cmd
}
