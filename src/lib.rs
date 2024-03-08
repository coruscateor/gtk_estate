//mod node_contents;

//mod application_node;

//mod container_node;

mod scoped_signal_handler_id;

pub use scoped_signal_handler_id::*; 

mod scoped_signal_handler_id_hashmap;

pub use scoped_signal_handler_id_hashmap::*; 

//mod GTK;

//mod storage_container;

mod state_containers;

pub use state_containers::*; 

//mod container_vec;

//pub use container_vec::*; 

//mod container_map;

//pub use container_map::*; 

//mod object_container;

//pub use object_container::*; 

pub extern crate gtk4;

#[cfg(feature="adw")]
pub extern crate adw;

pub extern crate corlib;

pub mod gtk_enums;

pub mod adw_enums;

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

//mod hash_by_widget;

//pub use hash_by_widget::*;

//mod partail_enums;

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
