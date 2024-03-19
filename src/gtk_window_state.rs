
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
    window: WidgetAdapter<T, GtkWindowState<T>>
    //window_title: WindowTitle,
    //hb: HeaderBar,
    //contents: Box 

}

impl<T> GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt + Clone //IsA<Widget>, //MayDowncastTo<Widget> +
          //P: WidgetStateContainer + Clone
{

    pub fn weak_self(&self) -> Weak<Self>
    {

        self.weak_self.clone()

    }

    pub fn window(&self) -> WidgetAdapter<T, GtkWindowState<T>>
    {

        self.window.clone()

    }

    pub fn child(&self) -> Option<Rc<dyn WidgetStateContainer>>
    {

        if let Some(widget) = self.window.widget().child()
        {

            return StateContainers::get().find_widget_state(&widget);
            
        }

        None

    }

    pub fn set_child(&self, child_state: Option<&Rc<dyn WidgetStateContainer>>)
    {

        if let Some(state) = child_state
        {

            self.window.widget().set_child(Some(&state.adapted_widget().widget()))
            
        }

    }

}


impl<T> GtkWindowState<T>
    where T: GtkWindowExt + WidgetExt + IsA<T>, //+ MayDowncastTo<Widget>,
          //P: WidgetStateContainer
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
                window: WidgetAdapter::new(&window_fn(), weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        })

    }

}

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

    fn adapted_widget(&self) -> &(dyn crate::StoredWidgetObject)
    {
        
        &self.window

    }

}


