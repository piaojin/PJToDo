extern crate cc;

use std::process::Command;

fn main() {
    let current_path_output = Command::new("pwd")
        .output()
        .expect("pwd command failed to start");
    println!(
        "current_rust_path_output: {}",
        String::from_utf8_lossy(&current_path_output.stdout).trim()
    );

    let pal_path_output = Command::new("sh")
        .arg("get_xcode_var_pal_path.sh")
        .output()
        .expect("sh get_xcode_var_pla_path.sh command failed to start");

    let pal_path = format!(
        "{:}",
        String::from_utf8_lossy(&pal_path_output.stdout).trim()
    );

    println!(
        "pla_file_path: {}",
        String::from_utf8_lossy(&pal_path_output.stdout).trim()
    );
    println!(
        "pal_path_output_stdout: {}",
        String::from_utf8_lossy(&pal_path_output.stdout).trim()
    );
    println!(
        "pal_path_output_stderr: {}",
        String::from_utf8_lossy(&pal_path_output.stderr).trim()
    );

    if !pal_path.is_empty() {
        cc::Build::new().file(pal_path).compile("PJToDoCoreLibPAL");
    } else {
        println!(
            "Warning❌❌❌❌❌❌pal path is empty, pal used for rust to call Swift func cannot be empty!❌❌❌❌❌❌"
        );
    }
}
