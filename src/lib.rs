#![doc = include_str!("../README.md")]

mod scoped_signal_handler_id;

pub use scoped_signal_handler_id::*; 

mod scoped_signal_handler_id_hashmap;

pub use scoped_signal_handler_id_hashmap::*; 

mod state_containers;

pub use state_containers::*;

pub extern crate gtk4;

#[cfg(feature="adw")]
pub extern crate adw;

pub extern crate corlib;

pub mod time_out;

pub mod helpers;

mod simple_time_out;

pub use simple_time_out::*;

mod diy;

pub use diy::*;

mod scoped_source_id;

pub use scoped_source_id::*;

mod adapters;

pub use adapters::*;

mod widget_state_containers;

pub use widget_state_containers::*;

mod gtk_window_state;

pub use gtk_window_state::*;

#[cfg(feature="adw")]
mod adw_window_state;

#[cfg(feature="adw")]
pub use adw_window_state::*;

#[cfg(feature="adw")]
mod adw_application_window_state;

#[cfg(feature="adw")]
pub use adw_application_window_state::*;

pub mod rc_conversions;
