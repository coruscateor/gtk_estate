use gtk4 as gtk;

use gtk::{Widget, Paned, prelude::IsA, prelude::WidgetExt};

///
///Center the panes posion using the given value.
/// 
pub fn set_paned_position_halved(paned: &Paned, value: i32)
{

    paned.set_position(value / 2);

}
