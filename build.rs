use std::{env, process::Command};

fn declare_cfg(cfg: &str) {
    println!("cargo:rustc-cfg={}", cfg);
    println!("cargo:rustc-check-cfg=cfg({})", cfg);
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let target_arch = target.split_once("-").unwrap().0;
    let target_features = std::env::var("CARGO_CFG_TARGET_FEATURE")
        .unwrap_or_default();

    if
        (target_arch == "x86" || target_arch == "x86_64")
        && target_features.contains("sse2")
        && target_features.contains("pclmulqdq")
        && target_features.contains("sse4.1")
    {
        declare_cfg("always_have_specialized");
    }
    
    if let Some(minor_version) = minor_rustc_version() {
        // rustc 1.80 stabilized ARM CRC32 intrinsics:
        // https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.__crc32d.html
        if minor_version >= 80 {
            declare_cfg("stable_arm_crc32_intrinsics");

            if target_arch == "aarch64"
                && target_features.contains("crc")
            {
                declare_cfg("always_have_specialized");
            }
        }
    }
}

fn minor_rustc_version() -> Option<u32> {
    Command::new(env::var_os("RUSTC")?)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            std::str::from_utf8(&output.stdout).ok().and_then(|output| {
                output
                    .split('.')
                    .nth(1)
                    .and_then(|minor_version| minor_version.parse().ok())
            })
        })
}
