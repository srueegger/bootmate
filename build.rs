// SPDX-License-Identifier: GPL-2.0-only

use std::process::Command;

fn main() {
    // Compile GResources to src directory so include_bytes! can find it
    let out = Command::new("glib-compile-resources")
        .args([
            "--sourcedir=data",
            "--sourcedir=data/ui",
            "--target=src/bootmate.gresource",
            "data/bootmate.gresource.xml",
        ])
        .output();

    if let Ok(output) = out {
        if !output.status.success() {
            eprintln!("Failed to compile GResources:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    println!("cargo:rerun-if-changed=data/bootmate.gresource.xml");
    println!("cargo:rerun-if-changed=data/ui/window.ui");
}
