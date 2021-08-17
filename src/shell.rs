use std::process::Stdio;
use std::io::{Write};

#[cfg(windows)]
pub fn run_command(shell: String, arguments: Vec<String>, command: String) -> std::result::Result<(), clap::Error> {
    // println!("arguments = {:?}", arguments);
    let mut child_process = std::process::Command::new(shell)
        .args(arguments)
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

    // println!("output = {:?}", output);

    Ok(())
}


#[cfg(unix)]
pub fn run_command(shell: &str, arguments: Vec<&str>, command: &str) -> std::result::Result<(), clap::Error> {
    // Unix - pass command as parameter for initializer
    let mut arguments = arguments.clone();
    if !command.is_empty() {
        arguments.push(command);
    }

    //println!("arguments = {:?}", arguments);
    let mut child_process = std::process::Command::new(shell)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    {

    }
    let output = child_process.wait_with_output()?;
    //println!("output = {:?}", output);
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn start_terminal(command: &str) {

    let script = format!("tell application \"iTerm2\"
    set newWindow to (create window with default profile)
    tell current session of newWindow
        write text \"{}\"
    end tell
end tell", command);
    let mut arguments: Vec<&str> = [].to_vec();
    arguments.push("-e");
    run_command("/usr/bin/osascript", arguments, script.as_str());
}

#[cfg(target_os = "linux")]
pub fn start_terminal() {

}

#[cfg(target_os = "windows")]
pub fn start_terminal() {

}
