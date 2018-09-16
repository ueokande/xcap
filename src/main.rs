use std::error::Error;
use std::process::{Command, ExitStatus};

mod xwininfo;

fn run(args: Vec<String>) -> Result<ExitStatus, Box<Error>> {
    let info = xwininfo::run()?;
    let mut child = Command::new("ffmpeg")
        .args(&[
            "-video_size".to_string(),
            format!("{}x{}", info.width, info.height),
        ])
        .args(&["-f", "x11grab"])
        .args(&["-i".to_string(), format!(":0.0+{},{}", info.x, info.y)])
        .args(args)
        .spawn()?;

    let code = child.wait()?;
    Ok(code)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(args) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}
