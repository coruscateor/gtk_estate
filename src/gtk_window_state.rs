
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{gtk4 as gtk, StateContainers, WidgetAdapter, WidgetStateContainer};

//use crate::gtk4::prelude::{BoxExt, WidgetExt};

//use crate::{HasObject, impl_has_application_window, impl_has_object, StateContainers};

//use crate::gtk4::{self as gtk, Box, Orientation};

//use crate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle, prelude::AdwApplicationWindowExt, gtk::prelude::ApplicationWindowExt, gtk::prelude::GtkWindowExt};

use crate::corlib::{NonOption, rc_self_rfc_setup};

//use crate::window_contents_state::WindowContentsState;

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
    adapter: Rc<WidgetAdapter<T, GtkWindowState<T>>>
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
                adapter: WidgetAdapter::new(&window_fn(), weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        })

    }

    pub fn weak_self(&self) -> Weak<Self>
    {

        self.weak_self.clone()

    }

    pub fn adapter(&self) -> Rc<WidgetAdapter<T, GtkWindowState<T>>>
    {

        self.adapter.clone()

    }

    pub fn child(&self) -> Option<Rc<dyn WidgetStateContainer>>
    {

        if let Some(widget) = self.adapter.widget().child()
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

        self.adapter.widget().set_child(Some(&child_state.dyn_adapter().widget()))

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

impl<T> WidgetStateContainer for GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt //+ IsA<T>, //+ MayDowncastTo<Widget>
          //P: WidgetStateContainer
{

    fn dyn_adapter(&self) -> Rc<dyn crate::StoredWidgetObject>
    {
        
        self.adapter.clone()

    }

}


