
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{gtk4 as gtk, impl_weak_self_methods, impl_widget_state_container, StateContainers, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use gtk4::Window;
use corlib::AsAny;
use gtk::glib::object::IsA; //{IsA, MayDowncastTo};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

#[derive(Clone)]
pub struct GtkWindowState
{

    weak_self: Weak<Self>,
    window_adapter: Rc<WidgetAdapter<Window, GtkWindowState>>
    //window_title: WindowTitle,
    //hb: HeaderBar,
    //contents: Box 

}

impl GtkWindowState
{

    pub fn new<F>(window_fn: F) -> Rc<Self> //app: &Application
        where F: FnOnce()-> Window
    {

        Rc::new_cyclic(|weak_self|
        {

            //let wsc = weak_self.clone();

            //let wwsc: &Weak<dyn WidgetStateContainer> = weak_self;

            //let wwsc: &dyn Any = weak_self;

            Self
            {

                weak_self: weak_self.clone(),
                window_adapter: WidgetAdapter::new(&window_fn(), weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        })

    }

    impl_weak_self_methods!(window_adapter);

    pub fn window_adapter(&self) -> Rc<WidgetAdapter<Window, GtkWindowState>>
    {

        self.window_adapter.clone()

    }

    pub fn window_adapter_ref(&self) -> &WidgetAdapter<Window, GtkWindowState>
    {

        self.window_adapter.as_ref()

    }

    pub fn child(&self) -> Option<Rc<dyn WidgetStateContainer>>
    {

        if let Some(widget) = self.window_adapter.widget().child()
        {

            return StateContainers::get().find_widget_state(&widget);
            
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

    pub fn set_child(&self, child_state: &Rc<dyn WidgetStateContainer>)
    {

        self.window_adapter.widget().set_child(Some(&child_state.dyn_widget_adapter().widget()))

    }

}


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

impl WidgetStateContainer for GtkWindowState
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


