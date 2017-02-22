use std::process::Command;

use tera::{Context, Tera};


pub fn safari_furl() -> String {
    run_applescript("tell application \"Safari\" to get URL of document 1")
}


pub fn safari_2url() -> String {
    run_applescript("tell application \"Safari\" to get URL of document 2")
}


pub fn safari_closetabs(urls: Vec<&str>) -> String {
    let clean_tabs_template = include_str!("scripts/clean-tabs.scpt");

    let mut context = Context::new();
    context.add("urls", &urls);

    let script = Tera::one_off(&clean_tabs_template, context, false).unwrap();
    run_applescript(&script)
}


/// Runs an AppleScript and returns the stdout.
fn run_applescript(script: &str) -> String {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("failed to execute AppleScript");

    // Strip the trailing newline and return a String
    let mut output = output.stdout;
    output.pop();
    String::from_utf8(output).unwrap()
}
