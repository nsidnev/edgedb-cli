pub mod options;
mod main;
mod metadata;
pub mod methods;
pub mod detect;
pub mod distribution;
pub mod remote;
pub mod version;
pub mod os_trait;
mod debian_like;

// OSs
mod unix;
pub mod linux;
mod debian;
mod ubuntu;
mod centos;
pub mod macos;
mod windows;
mod unknown_os;

// Methods
mod docker;
pub mod package;

// commands
pub mod control;
pub mod create;
pub mod destroy;
pub mod errors;
mod info;
pub mod install;
pub mod link;
mod list_versions;
pub mod reset_password;
mod revert;
mod status;
mod uninstall;
pub mod upgrade;

pub use main::{main, instance_main};


pub fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    for c in chars {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return false;
        }
    }
    return true;
}
