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
