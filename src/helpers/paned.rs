//!
//! Helper functions for the Paned widget. 
//!

use gtk::{Widget, Paned, prelude::IsA, prelude::WidgetExt};

///
/// Center the panes position using the given value.
/// 
pub fn set_paned_position_halved(paned: &Paned, value: i32)
{

    paned.set_position(value / 2);

}
