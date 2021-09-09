use std::{ffi::OsStr, process::{Command, Stdio}};
use std::io::prelude::*;

use crate::error::GdlError;

pub fn dot_exec(args: Vec<impl AsRef<OsStr>>, dot_script: &str) -> Result<(), GdlError> {

    // Check if dot exists
    which::which("rustc")?;

    let process;

    // Windows
    if cfg!(target_os = "windows") {
        process = match Command::new("cmd")
            .arg("/C")
            .arg("dot")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
                Err(why) => return Err(GdlError::IoError(why)),
                Ok(process) => process,
            };
    } else { // *nix systems
        process = match Command::new("dot")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
                Err(why) => return Err(GdlError::IoError(why)),
                Ok(process) => process,
            };
    }

    // Send dotscript as stdin
    if let Err(why) = process.stdin.unwrap().write_all(dot_script.as_bytes()) {
        return Err(GdlError::IoError(why));
    }

    // NOTE why saving output to string
    // https://doc.rust-lang.org/rust-by-example/std_misc/process/pipe.html
    //
    // Looks like stdin is dropped if stdout is not called
    // not 100% sure though
    let mut s = String::new();
    if let Err(why) =  process.stdout.unwrap().read_to_string(&mut s) {
        return Err(GdlError::IoError(why));
    }

    Ok(())
}
