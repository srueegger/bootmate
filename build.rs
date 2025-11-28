// SPDX-License-Identifier: GPL-2.0-only

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Compile GResources to src directory so include_bytes! can find it
    let src_target = PathBuf::from("src/bootmate.gresource");
    let out = Command::new("glib-compile-resources")
        .args([
            "--sourcedir=data",
            "--sourcedir=data/ui",
            &format!("--target={}", src_target.display()),
            "data/bootmate.gresource.xml",
        ])
        .output();

    if let Ok(output) = out {
        if !output.status.success() {
            eprintln!("Failed to compile GResources:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        } else {
            // Also copy to OUT_DIR for fallback
            let out_target = out_dir.join("bootmate.gresource");
            let _ = fs::copy(&src_target, &out_target);
        }
    }

    println!("cargo:rerun-if-changed=data/bootmate.gresource.xml");
    println!("cargo:rerun-if-changed=data/ui/window.ui");
}
