
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{impl_widget_state_container_traits, scs_add, DynWidgetStateContainer, StateContainers, WidgetAdapter, WidgetObject, WidgetUpgradeResult, WidgetStateContainer}; //DynApplicationStateContainer, 

//impl_weak_self_methods, StrongWidgetObject, StrongWidgetStateContainers,

use adw::builders::{ApplicationWindowBuilder, WindowBuilder};
//use adw::ffi::AdwApplicationWindow;
use adw::ApplicationWindow;

use corlib::convert::AsAnyRef;

use gtk::glib::object::IsA; //{IsA, MayDowncastTo};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

use adw::prelude::AdwApplicationWindowExt;

static ERROR_APPLICATION_WINDOW_EXPECTED: &str = "Error: ApplicationWindow expected";

#[derive(Clone, Debug)]
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

    pub fn new(window: &ApplicationWindow) -> Rc<Self>
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                //weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window, weak_self)

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

    pub fn new_visible(window: &ApplicationWindow) -> Rc<Self>
    {

        let sc = Self::new(window);

        window.set_visible(true);

        sc

    }

    pub fn with_content<WSC>(window: &ApplicationWindow, content_state: &Rc<WSC>) -> WidgetUpgradeResult<Rc<Self>>
        where WSC: DynWidgetStateContainer
    {

        let widget = content_state.dyn_widget_adapter().widget()?;

        let sc = Self::new(window);

        window.set_content(Some(&widget)); //(content_state); //Some(content_state));

        Ok(sc)

    }

    pub fn with_content_visible<WSC>(window: &ApplicationWindow, content_state: &Rc<WSC>) -> WidgetUpgradeResult<Rc<Self>>
        where WSC: DynWidgetStateContainer
    {

        let sc = Self::with_content(window, content_state)?;

        window.set_visible(true);

        //sc.widget_adapter.widget().expect(ERROR_APPLICATION_WINDOW_EXPECTED).set_visible(true);

        Ok(sc)

    }

    pub fn builder<F>(window_fn: F) -> (Rc<Self>, ApplicationWindow)
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindowBuilder
    {

        let builder = ApplicationWindow::builder();

        let window = window_fn(builder).build();

        let aws = Rc::new_cyclic(|weak_self|
        {
            Self
            {

                //weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window, weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        });

        //let any_this: &dyn Any = &aws;

        //let wsc = any_this.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        #[cfg(feature = "thread_local_state")]
        let _ = StateContainers::get().widget_state_ref().add(&aws);//wsc);

        (aws, window)

    }

    pub fn builder_visible<F>(window_fn: F) -> (Rc<Self>, ApplicationWindow)
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindowBuilder,
    {
        
        let sc_window= Self::builder(window_fn);

        sc_window.1.set_visible(true);

        //sc.widget_adapter.widget().expect(ERROR_APPLICATION_WINDOW_EXPECTED).set_visible(true);

        sc_window

    }

    pub fn builder_with_content<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> WidgetUpgradeResult<(Rc<Self>, ApplicationWindow)>
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindowBuilder,
            WSC: DynWidgetStateContainer
    {

        let widget = content_state.dyn_widget_adapter().widget()?;

        let sc_window = Self::builder(window_fn);

        sc_window.1.set_content(Some(&widget)); //(Some(content_state)); //Some(content_state));

        Ok(sc_window)

    }

    pub fn builder_with_content_visible<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> WidgetUpgradeResult<(Rc<Self>, ApplicationWindow)>
        where F: FnOnce(ApplicationWindowBuilder) -> ApplicationWindowBuilder,
            WSC: DynWidgetStateContainer
    {

        let sc_window = Self::builder_with_content(window_fn, content_state)?;

        sc_window.1.set_visible(true);

        //sc.widget_adapter.widget().expect(ERROR_APPLICATION_WINDOW_EXPECTED).set_visible(true);

        Ok(sc_window)

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

        #[cfg(feature = "thread_local_state")]
        let _ = StateContainers::get().widget_state_ref().add(&aws);

        aws

    }
    
    //

    //impl_weak_self_methods!(widget_adapter);

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

    pub fn content(&self) -> WidgetUpgradeResult<Option<Rc<dyn DynWidgetStateContainer>>>
    {

        if let Some(widget) = self.widget_adapter.widget()?.content()
        {

            return Ok(StateContainers::get().widget_state_ref().find_widget_state(&widget));
            
        }

        Ok(None)

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

    pub fn set_content<WSC: DynWidgetStateContainer>(&self, child_state: &Rc<WSC>) -> WidgetUpgradeResult
    {

        /*
        if let Some(state) = child_state
        {

            self.window_adapter.widget().set_content(Some(&state.dyn_adapted_widget().widget()))
            
        }
        */

        self.widget_adapter.widget()?.set_content(Some(&child_state.dyn_widget_adapter().widget()?));

        Ok(())

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

