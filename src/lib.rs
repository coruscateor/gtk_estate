#![doc = include_str!("../README.md")]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use cfg_if::cfg_if;

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

mod time_out;

pub use time_out::*;

pub mod helpers;

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

mod widget_container;

pub use widget_container::*;

mod contents;

pub use contents::*;

mod clear_state_containers_on_drop;

pub use clear_state_containers_on_drop::*;

mod weak_adapters;

pub use weak_adapters::*;

mod weak_widget_state_containers;

pub use weak_widget_state_containers::*;

cfg_if!
{

    if #[cfg(feature = "adw")]
    {

        mod adw_window_state;

        pub use adw_window_state::*;

        mod adw_application_window_state;
        
        pub use adw_application_window_state::*;

    }

}

pub mod rc_conversions;
