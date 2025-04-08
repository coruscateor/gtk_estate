use gtk::Widget;

///
/// For when your state-container directly contains the widget object.
/// 
/// Use with sub-contents objects.
/// 
/// Sub-contents objects are objects referenced by contents-state objects or other sub-contents objects.
///
/// They are a way to reuse user interface components and state in GTK Estate.
///
/// They contain strong widget references as they are only intended to be referenced in contents-state and other sub-contents objects.
/// 
pub trait WidgetContainer
{

    fn widget(&self) -> Widget;

    fn widget_ref(&self) -> &Widget;

}

///
/// Implements WidgetContainer on an object.
/// 
#[macro_export]
macro_rules! impl_widget_container
{

    ($widget_field:ident, $widget_state_container_type:ty) =>
    {

        impl WidgetContainer for $widget_state_container_type
        {

            fn widget(&self) -> Widget
            {
    
                self.$widget_field.upcast_ref::<Widget>().clone()
                
            }
    
            fn widget_ref(&self) -> &Widget
            {
    
                self.$widget_field.upcast_ref::<Widget>()
                
            }

        }

    }

}