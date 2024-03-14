
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

use adw::builders::WindowBuilder;
use adw::prelude::{AdwWindowExt, AdwApplicationWindowExt};
use adw::Window;
use corlib::AsAny;
use gtk::glib::object::{IsA, MayDowncastTo};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

pub struct AdwWindowState<T>
    where T: GtkWindowExt + WidgetExt
{

    weak_self: Weak<Self>,
    window: WidgetAdapter<T>

}

impl<T> AdwWindowState<T>
    where T: GtkWindowExt + WidgetExt + MayDowncastTo<Widget> + IsA<Widget> + AdwWindowExt
{

    pub fn weak_self(&self) -> Weak<Self>
    {

        self.weak_self.clone()

    }

    pub fn window(&self) -> WidgetAdapter<T>
    {

        self.window.clone()

    }

    pub fn content(&self) -> Option<Rc<dyn WidgetStateContainer>>
    {

        if let Some(widget) = self.window.widget().content()
        {

            return StateContainers::get().find_widget_state(&widget);
            
        }

        None

    }

    pub fn dyn_set_content(&self, child_state: Option<&Rc<dyn WidgetStateContainer>>)
    {

        if let Some(state) = child_state
        {

            self.window.widget().set_content(Some(&state.widget().widget()))
            
        }

    }

    pub fn set_content<WSC: WidgetStateContainer>(&self, child_state: Option<&Rc<WSC>>)
    {

        if let Some(state) = child_state
        {

            self.window.widget().set_content(Some(&state.widget().widget()))
            
        }

    }

}


impl<T> AdwWindowState<T>
    where T: GtkWindowExt + WidgetExt + AdwWindowExt + IsA<T> + MayDowncastTo<Widget>,
{

    pub fn new<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce()-> T
    {

        let aws = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                weak_self: weak_self.clone(),
                window: WidgetAdapter::new(window_fn(), weak_self)

            }

        });

        //let any_this: &dyn Any = &aws;

        //let wsc = any_this.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        //StateContainers::get().add(wsc);

        StateContainers::get().add(&aws);

        aws

    }

    pub fn with_content<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce()-> T,
              WSC: WidgetStateContainer
    {

        let sc = Self::new(window_fn);

        sc.set_content(Some(content_state));

        sc

    }

    pub fn builder<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce(WindowBuilder)-> T
    {

        let builder = Window::builder();

        let aws = Rc::new_cyclic(|weak_self|
        {
            Self
            {

                weak_self: weak_self.clone(),
                window: WidgetAdapter::new(window_fn(builder), weak_self)

            }

        });

        //let any_this: &dyn Any = &aws;

        //let wsc = any_this.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        //StateContainers::get().add(wsc);

        StateContainers::get().add(&aws);

        aws

    }

    pub fn builder_with_content<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce(WindowBuilder)-> T,
              WSC: WidgetStateContainer
    {

        let sc = Self::builder(window_fn);

        sc.set_content(Some(content_state));

        sc

    }

}

/*
impl<T> AdwWindowState<T>
    where T: GtkWindowExt + WidgetExt + AdwApplicationWindowExt+ IsA<T> + MayDowncastTo<Widget>,
{

    pub fn new<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce()-> T
    {

        let aws = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                weak_self: weak_self.clone(),
                window: WidgetAdapter::new(window_fn(), weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        });

        let any_this: &dyn Any = &aws;

        let wsc = any_this.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        StateContainers::get().add(wsc);

        aws

    }

}
*/

impl<T> AsAny for AdwWindowState<T>
    where T: GtkWindowExt + WidgetExt
{

    fn as_any(&self) -> &dyn std::any::Any
    {
        
        self

    }

}

impl<T> WidgetStateContainer for AdwWindowState<T>
    where T: GtkWindowExt + WidgetExt + MayDowncastTo<Widget> + IsA<T>
{

    fn widget(&self) -> &(dyn crate::StoredWidgetObject)
    {
        
        &self.window

    }

}


