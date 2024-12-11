
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{gtk4 as gtk, impl_widget_state_container, StateContainers, WidgetAdapter, WidgetStateContainer, StoredWidgetObject};

use corlib::AsAny;
use gtk::glib::object::IsA; //{IsA, MayDowncastTo};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

#[derive(Clone)]
pub struct GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt,
          //P: WidgetStateContainer
{

    weak_self: Weak<Self>,
    widget_adapter: Rc<WidgetAdapter<T, GtkWindowState<T>>>
    //window_title: WindowTitle,
    //hb: HeaderBar,
    //contents: Box 

}

impl<T> GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt + Clone //IsA<Widget>, //MayDowncastTo<Widget> +
          //P: WidgetStateContainer + Clone
{

    pub fn new<F>(window_fn: F) -> Rc<Self> //app: &Application
        where F: FnOnce()-> T
    {

        Rc::new_cyclic(|weak_self|
        {

            //let wsc = weak_self.clone();

            //let wwsc: &Weak<dyn WidgetStateContainer> = weak_self;

            //let wwsc: &dyn Any = weak_self;

            Self
            {

                weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window_fn(), weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        })

    }

    pub fn weak_self(&self) -> Weak<Self>
    {

        self.weak_self.clone()

    }

    pub fn window(&self) -> Rc<WidgetAdapter<T, GtkWindowState<T>>>
    {

        self.widget_adapter.clone()

    }

    pub fn window_ref(&self) -> &WidgetAdapter<T, GtkWindowState<T>>
    {

        self.widget_adapter.as_ref()

    }

    pub fn child(&self) -> Option<Rc<dyn WidgetStateContainer>>
    {

        if let Some(widget) = self.widget_adapter.widget().child()
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

        self.widget_adapter.widget().set_child(Some(&child_state.dyn_adapter().widget()))

    }

}


//impl<T> GtkWindowState<T>
//    where T: GtkWindowExt + WidgetExt + IsA<T>, //+ MayDowncastTo<Widget>,
          //P: WidgetStateContainer
//{
//}

impl<T> AsAny for GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt,
          //P: WidgetStateContainer
{

    fn as_any(&self) -> &dyn std::any::Any
    {
        
        self

    }

}

//impl_widget_state_container!(GtkWindowState<T>);

impl<T> WidgetStateContainer for GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt //+ IsA<T>, //+ MayDowncastTo<Widget>
          //P: WidgetStateContainer
{

    fn dyn_adapter(&self) -> Rc<dyn crate::StoredWidgetObject>
    {
        
        self.widget_adapter.clone()

    }

    fn dyn_adapter_ref(&self) -> &dyn StoredWidgetObject
    {

        self.widget_adapter.as_ref()

    }

}


