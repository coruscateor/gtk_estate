use gtk::Widget;

///
/// For when your state-container directly contains the widget object.
/// 
/// Use with sub-contents objects.
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