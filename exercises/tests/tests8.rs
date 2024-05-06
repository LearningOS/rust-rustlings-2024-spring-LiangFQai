//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
fn main() {
    // Set up environment variable "TEST_FOO" for tests7
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_foo_command = format!("TEST_FOO={}", timestamp + 5);
    println!("cargo:rustc-env={}", test_foo_command);

    // Enable "pass" feature for tests8
    let pass_feature_command = "FEATURES=pass";
    println!("cargo:rustc-env={}", pass_feature_command);
}

