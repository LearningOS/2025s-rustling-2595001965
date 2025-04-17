//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // 使用当前时间戳来设置 TEST_FOO
// In build.rs
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // Replace the placeholder command with valid feature activation
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}