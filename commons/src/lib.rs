#![warn(unused_crate_dependencies)]
#![warn(clippy::pedantic)]

pub mod env_command;
pub mod gem_list;
pub mod gem_version;
pub mod gemfile_lock;
pub mod in_app_dir_cache;
pub mod rake_detect;

mod in_app_dir_cache_layer;
