//!
//! Helper functions for widgets that implement the gtk4::auto::widget::WidgetExt trait. 
//!

use gtk::{Widget, Paned, prelude::IsA, prelude::WidgetExt};

use gtk::glib::{object::{ObjectExt, Cast}, types::StaticType};

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

///
/// Try to find a direct or an indirect parent of the provided child widget which is of the type T. 
/// 
pub fn try_find_parent<T>(child_widget: &(impl WidgetExt + ObjectExt + StaticType + Cast)) -> Result<T, Widget>
    where T: WidgetExt + ObjectExt + StaticType + Cast
{

    let cw = child_widget.clone();

    let mut next_child: Widget = cw.into();

    loop {
        
        let opt_parent = next_child.parent();

        if let Some(parent) = opt_parent
        {

            if parent.is::<T>()
            {

                return parent.downcast::<T>();

            }

            next_child = parent;

        }
        else
        {

            return Err(next_child);

        }

    }

}

///
/// Find a direct or an indirect parent of the provided child widget which is of the type T or panic.
/// 
pub fn find_parent<T>(child_widget: &(impl WidgetExt + ObjectExt + StaticType + Cast)) -> T
    where T: WidgetExt + ObjectExt + StaticType + Cast
{

    try_find_parent::<T>(child_widget).expect("Error: A parent Widget of the specified type has not been found.")

}





