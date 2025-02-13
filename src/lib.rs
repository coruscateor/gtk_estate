#![doc = include_str!("../README.md")]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use cfg_if::cfg_if;

mod scoped_signal_handler_id;

pub use scoped_signal_handler_id::*; 

mod scoped_signal_handler_id_hashmap;

pub use scoped_signal_handler_id_hashmap::*; 

mod state_containers;

pub use state_containers::*;

pub extern crate gtk;

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

cfg_if!
{

    if #[cfg(feature = "strong_widget_state")]
    {

        mod strong_adapters;

        pub use strong_adapters::*;

        mod strong_widget_state_containers;

        pub use strong_widget_state_containers::*;

    }

}

//mod gtk_window_state;

//pub use gtk_window_state::*;

mod widget_container;

pub use widget_container::*;

mod contents;

pub use contents::*;

//Disabled

//mod clear_state_containers_on_drop;

//pub use clear_state_containers_on_drop::*;

mod adapters;

pub use adapters::*;

mod widget_state_containers;

pub use widget_state_containers::*;

/*
cfg_if!
{

    if #[cfg(feature = "strong_widget_state")]
    {

        mod strong_gtk_window_state;

        pub use strong_gtk_window_state::*;

    }

}

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

cfg_if!
{

    if #[cfg(all(feature = "strong_widget_state", feature = "adw"))]
    {

        mod strong_adw_application_window_state;

        pub use strong_adw_application_window_state::*;

        mod strong_adw_window_state;

        pub use strong_adw_window_state::*;

    }

}
*/

pub mod rc_conversions;

mod widget_upgrade_error_handlers;

pub use widget_upgrade_error_handlers::*;
