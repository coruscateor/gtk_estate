
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{impl_widget_state_container_traits, scs_add, DynWidgetStateContainer, StateContainers, WidgetAdapter, WidgetObject, WidgetStateContainers, WidgetUpgradeResult};

//impl_weak_self_methods, 

use gtk::Window;

use corlib::convert::AsAnyRef;

use gtk::glib::object::IsA; //{IsA, MayDowncastTo};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

#[derive(Clone, Debug)]
pub struct GtkWindowState
{

    //weak_self: Weak<Self>,
    widget_adapter: Rc<WidgetAdapter<Window, GtkWindowState>>
    //window_title: WindowTitle,
    //hb: HeaderBar,
    //contents: Box 

}

impl GtkWindowState
{

    pub fn new<F>(window_fn: F) -> Rc<Self> //app: &Application
        where F: FnOnce()-> Window
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            //let wsc = weak_self.clone();

            //let wwsc: &Weak<dyn WidgetStateContainer> = weak_self;

            //let wwsc: &dyn Any = weak_self;

            Self
            {

                //weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window_fn(), weak_self) //wwsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().unwrap()) //weak_self)

            }

        });

        #[cfg(feature = "thread_local_state")]
        scs_add!(this);

        this

    }

    //impl_weak_self_methods!(widget_adapter);

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

    pub fn child(&self) -> WidgetUpgradeResult<Option<Rc<dyn DynWidgetStateContainer>>>
    {

        if let Some(widget) = self.widget_adapter.widget()?.child()
        {

            return Ok(StateContainers::get().widget_state_ref().find_widget_state(&widget));
            
        }

        Ok(None)

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

    pub fn set_child(&self, child_state: &Rc<dyn DynWidgetStateContainer>) -> WidgetUpgradeResult
    {

        self.widget_adapter.widget()?.set_child(Some(&child_state.dyn_widget_adapter().widget()?));

        Ok(())

    }

}

impl_widget_state_container_traits!(Window, GtkWindowState);

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


