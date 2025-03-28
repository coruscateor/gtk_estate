use std::time::Duration;

//use gtk4 as gtk;

use gtk::glib::{ControlFlow, idle_add_local, timeout_add_seconds_local}; //SourceId, 

use gtk::glib::source::timeout_add_local;

//https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/source/index.html

use super::ScopedSourceId;

///
/// Converts a ControlFlow instance into a bool.
/// 
pub fn should_flow(flow: ControlFlow) -> bool
{

    match flow
    {

        ControlFlow::Continue => true,
        ControlFlow::Break => false

    }

}

///
/// Converts a bool instance into a ControlFlow.
/// 
pub fn should_continue(should_flow: bool) -> ControlFlow
{

    match should_flow
    {

        true => ControlFlow::Continue,
        false => ControlFlow::Break

    }

}

///
/// Makes using gtk::glib::idle_add_local a bit easier.
/// 
pub fn idle_add_local_diy<F>(mut func: F) -> ScopedSourceId
where
    F: FnMut() -> bool + 'static
{

    ScopedSourceId::new(idle_add_local(move || should_continue(func())))

}

///
/// Calls glib::source::timeout_add_local and returns a ScopedSourceId.
/// 
pub fn timeout_add_local_diy<F>(interval: Duration, mut func: F) -> ScopedSourceId
where
    F: FnMut() -> bool + 'static //Continue + 'static,
{

    ScopedSourceId::new(timeout_add_local(interval, move || should_continue(func())))

}

///
/// Calls glib::source::timeout_add_seconds_local and returns a ScopedSourceId.
/// 
pub fn timeout_add_seconds_local_diy<F>(interval: u32, mut func: F) -> ScopedSourceId
where
    F: FnMut() -> bool + 'static
{

    ScopedSourceId::new(timeout_add_seconds_local(interval, move || should_continue(func())))

}

//Unix

