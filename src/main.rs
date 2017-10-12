extern crate clap;
extern crate notify;

use std::io;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{Watcher, RecursiveMode, watcher};
use notify::DebouncedEvent::Write;

use clap::{App, Arg};

fn main() {
    let matches = App::new("watch-file")
        .arg(Arg::with_name("filename")
             .index(1)
             .required(true))
        .arg(Arg::with_name("command")
             .index(2)
             .required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let command = matches.value_of("command").unwrap();

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1))
        .expect("Failed to create watcher");

    watcher.watch(filename, RecursiveMode::NonRecursive)
        .expect(&format!("Failed to watch {}", filename));

    while let Ok(e) = rx.recv() {
        if let Write(_) = e {
            //println!("{} modified, running {:?}", filename, command);

            let output = run_command(command)
                .expect("Failed to execute process");

            print!("{}", String::from_utf8(output).unwrap());
        }
    }
}

fn run_command(command: &str) -> Result<Vec<u8>, io::Error> {
    spawn_command(command)
        .output()
        .map(|o| o.stdout)
}

#[cfg(target_os = "windows")]
fn spawn_command(command: &str) -> Command {
    let mut cmd = Command::new("cmd");
    cmd.args(&["/C", command]);
    cmd
}

#[cfg(not(target_os = "windows"))]
fn spawn_command(command: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    cmd
}
