
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{gtk4 as gtk, impl_weak_self_methods, impl_widget_state_container_traits, scs_add, DynApplicationStateContainer, DynWidgetStateContainer, StateContainers, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use adw::builders::{ApplicationWindowBuilder, WindowBuilder};
//use adw::ffi::AdwApplicationWindow;
use adw::ApplicationWindow;

use corlib::convert::AsAnyRef;

use gtk::glib::object::IsA; //{IsA, MayDowncastTo};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

use adw::prelude::AdwApplicationWindowExt;

#[derive(Clone)]
pub struct AdwApplicationWindowState //<T>
    //where T: GtkWindowExt + AdwApplicationWindowExt + IsA<Widget>, //+ IsA<ApplicationWindow> //+ WidgetExt
          //P: WidgetStateContainer
{

    //weak_self: Weak<Self>,
    //window_adapter: Rc<WidgetAdapter<T, AdwApplcationWindowState<T>>>
    widget_adapter: Rc<WidgetAdapter<ApplicationWindow, AdwApplicationWindowState>>
}

impl AdwApplicationWindowState //<T>
    //where T: GtkWindowExt + AdwApplicationWindowExt + IsA<Widget>, // + MayDowncastTo<Widget> //+ IsA<ApplicationWindow>  // + WidgetExt
          //P: WidgetStateContainer + Clone
{

    pub fn new<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce()-> ApplicationWindow
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                //weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window_fn(), weak_self)

            }

        });

        #[cfg(feature = "thread_local_state")]
        scs_add!(this);

        this

        //let any_this: &dyn Any = &aws;

        //let wsc = any_this.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        //StateContainers::get().add(wsc);

        //StateContainers::get().add(&aws);

        //aws

    }

    pub fn new_visible<F, WSC>(window_fn: F) -> Rc<Self>
        where F: FnOnce() -> ApplicationWindow
    {

        let sc = Self::new(window_fn);

        sc.widget_adapter.widget().set_visible(true);

        sc

    }

    pub fn with_content<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce() -> ApplicationWindow,
            WSC: DynWidgetStateContainer
    {

        let sc = Self::new(window_fn);

        sc.set_content(content_state); //Some(content_state));

        sc

    }

    pub fn with_content_visible<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce() -> ApplicationWindow,
            WSC: DynWidgetStateContainer
    {

        let sc = Self::with_content(window_fn, content_state);

        sc.widget_adapter.widget().set_visible(true);

        sc

    }

    pub fn builder<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindow
    {

        let builder = ApplicationWindow::builder();

        let aws = Rc::new_cyclic(|weak_self|
        {
            Self
            {

                //weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window_fn(builder), weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        });

        //let any_this: &dyn Any = &aws;

        //let wsc = any_this.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        StateContainers::get().add(&aws);//wsc);

        aws

    }

    pub fn builder_visible<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindow,
    {
        
        let sc = Self::builder(window_fn);

        sc.widget_adapter.widget().set_visible(true);

        sc

    }

    pub fn builder_with_content<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindow,
            WSC: DynWidgetStateContainer
    {

        let sc = Self::builder(window_fn);

        sc.set_content(content_state); //Some(content_state));

        sc

    }

    pub fn builder_with_content_visible<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindow,
            WSC: DynWidgetStateContainer
    {

        let sc = Self::builder_with_content(window_fn, content_state);

        sc.widget_adapter.widget().set_visible(true);

        sc

    }

    pub fn from_exisiting(window: &ApplicationWindow) -> Rc<Self>
    {

        let aws = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                //weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window, weak_self)

            }

        });

        StateContainers::get().add(&aws);

        aws

    }

    //

    impl_weak_self_methods!(widget_adapter);

    /*
    pub fn window(&self) -> Rc<WidgetAdapter<ApplicationWindow, AdwApplcationWindowState>> //<T>>>
    {

        self.window_adapter.clone()

    }
    
    pub fn window_ref(&self) -> &WidgetAdapter<ApplicationWindow, AdwApplcationWindowState>
    {

        self.window_adapter.as_ref()

    }
    */

    pub fn content(&self) -> Option<Rc<dyn DynWidgetStateContainer>>
    {

        if let Some(widget) = self.widget_adapter.widget().content()
        {

            return StateContainers::get().find_widget_state(&widget);
            
        }

        None

    }

    /*
    pub fn dyn_set_content(&self, child_state: Option<&Rc<dyn WidgetStateContainer>>)
    {

        if let Some(state) = child_state
        {

            self.window.widget().set_content(Some(&state.widget().widget()))
            
        }

    }
    */

    pub fn set_content<WSC: DynWidgetStateContainer>(&self, child_state: &Rc<WSC>) //Option<&Rc<WSC>>)
    {

        /*
        if let Some(state) = child_state
        {

            self.window_adapter.widget().set_content(Some(&state.dyn_adapted_widget().widget()))
            
        }
        */

        self.widget_adapter.widget().set_content(Some(&child_state.dyn_widget_adapter().widget()))

    }

}

impl_widget_state_container_traits!(ApplicationWindow, AdwApplicationWindowState);


//impl<T> AdwApplcationWindowState<T>
//    where T: GtkWindowExt + AdwApplicationWindowExt + IsA<Widget>, //MayDowncastTo<Widget> + //IsA<T> +  //WidgetExt +
          //P: WidgetStateContainer
//{  
//}

/*
impl AsAny for AdwApplcationWindowState //<T>
    //where T: GtkWindowExt + AdwApplicationWindowExt + IsA<Widget>, //WidgetExt +
          //P: WidgetStateContainer
{

    fn as_any(&self) -> &dyn std::any::Any
    {
        
        self

    }

}
*/

/*

mismatched types
expected reference `&std::rc::Rc<(dyn adapters::StoredWidgetObject + 'static)>`
   found reference `&std::rc::Rc<adapters::WidgetAdapter<T, AdwApplcationWindowState<T>>>`rustcClick for full compiler diagnostic
adw_application_window_state.rs(225, 37): expected `&std::rc::Rc<(dyn adapters::StoredWidgetObject + 'static)>` because of return type
gtk_estate::adw_application_window_state::AdwApplcationWindowState

 */

 /*
impl DynWidgetStateContainer for AdwApplcationWindowState
    //where T: GtkWindowExt + AdwApplicationWindowExt + IsA<Widget>, //MayDowncastTo<Widget> + //IsA<T> + //WidgetExt +
          //P: WidgetStateContainer
{

    /*
    fn adapted_widget(&self) -> &(dyn crate::StoredWidgetObject)
    {
        
        &self.window

    }
    */

    fn dyn_widget_adapter(&self) -> Rc<dyn StoredWidgetObject>
    {
        
        self.window_adapter.clone()

        //&self.window_adapter

    }

    fn dyn_widget_adapter_ref(&self) -> &dyn StoredWidgetObject
    {

        self.window_adapter.as_ref()

    }

}
*/

