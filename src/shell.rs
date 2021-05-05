use std::process::Stdio;
use std::io::{Write};
use std::io::Read;

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
pub fn run_command(shell: String, arguments: Vec<String>, command: String) {
    // Unix - pass command as parameter for initializer
    arguments.push(command);
    let mut child_process = std::process::Command::new(shell)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    {

    }
    let output = child_process.wait_with_output()?;
}
