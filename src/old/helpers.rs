use gtk4 as gtk;

use gtk::{Widget, Paned, prelude::IsA, traits::WidgetExt};


pub fn set_hvexpand<T: WidgetExt>(widget: &T, expand: bool) // IsA<Widget>>
{
 
    widget.set_hexpand(expand);

    widget.set_vexpand(expand);
    
}

pub fn set_hvexpand_t<T: WidgetExt>(widget: &T) // IsA<Widget>>
{
 
    widget.set_hexpand(true);

    widget.set_vexpand(true);
    
}

pub fn set_hvexpand_f<T: WidgetExt>(widget: &T)
{
 
    widget.set_hexpand(false);

    widget.set_vexpand(false);
    
}

///Center the panes posion using the given value.
pub fn set_paned_position_halved(paned: &Paned, value: i32)
{

    paned.set_position(value / 2);

}


