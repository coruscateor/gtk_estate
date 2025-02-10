
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{impl_strong_widget_state_container_traits, impl_weak_self_methods, impl_widget_state_container_traits, scs_add, scs_strong_add, DynStrongWidgetStateContainer, StateContainers, StrongWidgetAdapter, StrongWidgetObject, StrongWidgetStateContainers, WidgetAdapter};

use gtk::Window;

use corlib::convert::AsAnyRef;

use gtk::glib::object::IsA;
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

#[derive(Clone, Debug)]
pub struct StrongGtkWindowState
{

    widget_adapter: Rc<StrongWidgetAdapter<Window, StrongGtkWindowState>>

}

impl StrongGtkWindowState
{

    pub fn new<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce()-> Window
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                widget_adapter: StrongWidgetAdapter::new(&window_fn(), weak_self)

            }

        });

        #[cfg(feature = "thread_local_state")]
        scs_strong_add!(this);

        this

    }
    
    impl_weak_self_methods!(widget_adapter);

    /*
    pub fn window_adapter(&self) -> Rc<WidgetAdapter<Window, GtkWindowState>>
    {

        self.window_adapter.clone()

    }

    pub fn window_adapter_ref(&self) -> &WidgetAdapter<Window, GtkWindowState>
    {

        self.window_adapter.as_ref()

    }
    */

    pub fn child(&self) -> Option<Rc<dyn DynStrongWidgetStateContainer>>
    {

        if let Some(widget) = self.widget_adapter.widget().child()
        {

            return StateContainers::get().strong_widget_state_ref().find_widget_state(&widget);
            
        }

        None

    }

    /*
    pub fn set_child(&self, child_state: Option<&Rc<dyn WidgetStateContainer>>)
    {

        if let Some(state) = child_state
        {

            self.window_adapter.widget().set_child(Some(&state.dyn_adapted_widget().widget()))
            
        }

    }
    */

    pub fn set_child(&self, child_state: &Rc<dyn DynStrongWidgetStateContainer>)
    {

        self.widget_adapter.widget().set_child(Some(&child_state.dyn_widget_adapter().widget()))

    }

}

impl_strong_widget_state_container_traits!(Window, StrongGtkWindowState);

//impl<T> GtkWindowState<T>
//    where T: GtkWindowExt + WidgetExt + IsA<T>, //+ MayDowncastTo<Widget>,
          //P: WidgetStateContainer
//{
//}

/*
impl<T> AsAny for GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt,
          //P: WidgetStateContainer
{

    fn as_any(&self) -> &dyn std::any::Any
    {
        
        self

    }

}
*/

//impl_widget_state_container!(GtkWindowState<T>);

/*
impl DynWidgetStateContainer for GtkWindowState
{

    fn dyn_widget_adapter(&self) -> Rc<dyn StoredWidgetObject>
    {
        
        self.window_adapter.clone()

    }

    fn dyn_widget_adapter_ref(&self) -> &dyn StoredWidgetObject
    {

        self.window_adapter.as_ref()

    }

}
*/


