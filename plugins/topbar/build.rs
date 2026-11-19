//! Embeds the exe icon (topbar.ico) via windres. Skips gracefully when no windres is available
//! (e.g. a cargo-xwin build without mingw tools): the exe simply has no icon then.
use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rerun-if-changed=topbar.ico");
    let target = env::var("TARGET").unwrap_or_default();
    let windres = env::var("WINDRES").unwrap_or_else(|_| match target.as_str() {
        t if t.starts_with("x86_64-pc-windows-gnu") => "x86_64-w64-mingw32-windres".to_string(),
        t if t.starts_with("i686-pc-windows-gnu") => "i686-w64-mingw32-windres".to_string(),
        _ => "windres".to_string(),
    });
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let obj = out.join("icon.o");
    let st = Command::new(&windres).args(["-c65001", "-i", "app.rc", "-o"]).arg(&obj).status();
    match st {
        Ok(s) if s.success() => {
            // rustc-link-obj is silently dropped for some target/linker combos; rustc-link-arg always lands
            println!("cargo:rustc-link-arg={}", obj.display());
        }
        _ => println!("cargo:warning=icon not embedded: {} unavailable or failed", windres),
    }
}
