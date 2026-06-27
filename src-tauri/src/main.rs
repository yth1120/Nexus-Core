// Nexus Core — Binary Entry Point
//
// Prevents a console window from appearing on Windows in release builds.
// All initialization logic is in nexus_core_lib::app::run().

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    nexus_core_lib::app::run();
}
