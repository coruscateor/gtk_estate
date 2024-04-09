use gtk4 as gtk;

use gtk::{Widget, Paned, prelude::IsA, prelude::WidgetExt};


//pub fn set_hvexpand<T: WidgetExt>(widget: &T, expand: bool) // IsA<Widget>>

pub fn set_hvexpand(widget: &impl WidgetExt, expand: bool)
{
 
    widget.set_hexpand(expand);

    widget.set_vexpand(expand);
    
}

//pub fn set_hvexpand_t<T: WidgetExt>(widget: &T) // IsA<Widget>>

pub fn set_hvexpand_t(widget: &impl WidgetExt)
{
 
    widget.set_hexpand(true);

    widget.set_vexpand(true);
    
}

//pub fn set_hvexpand_f<T: WidgetExt>(widget: &T)

pub fn set_hvexpand_f(widget: &impl WidgetExt)
{
 
    widget.set_hexpand(false);

    widget.set_vexpand(false);
    
}

pub fn set_margin_start_and_end(widget: &impl WidgetExt, margin: i32)
{

    widget.set_margin_start(margin);

    widget.set_margin_end(margin);

}

pub fn set_margin_top_and_bottom(widget: &impl WidgetExt, margin: i32)
{

    widget.set_margin_top(margin);

    widget.set_margin_bottom(margin);

}

pub fn set_margin_sides_and_bottom(widget: &impl WidgetExt, margin: i32)
{

    widget.set_margin_start(margin);

    widget.set_margin_end(margin);

    widget.set_margin_bottom(margin);

}

pub fn set_margin_all(widget: &impl WidgetExt, margin: i32)
{

    widget.set_margin_start(margin);

    widget.set_margin_end(margin);

    widget.set_margin_top(margin);

    widget.set_margin_bottom(margin);

}





