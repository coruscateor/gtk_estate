
use std::any::Any;
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use crate::{impl_weak_self_methods, impl_widget_state_container_traits, scs_add, DynWidgetStateContainer, StateContainers, StrongWidgetObject, WidgetAdapter, WidgetObject, WidgetStateContainers, WidgetUpgradeError, WidgetUpgradeResult};

use adw::builders::WindowBuilder;
use adw::prelude::{AdwWindowExt, AdwApplicationWindowExt};
use adw::Window;

use corlib::convert::AsAnyRef;

use gtk::glib::object::IsA; //{IsA, MayDowncastTo};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::Widget;

#[derive(Clone, Debug)]
pub struct AdwWindowState //<T>
    //where T: GtkWindowExt + WidgetExt,
          //P: WidgetStateContainer
{

    //weak_self: Weak<Self>,
    widget_adapter: Rc<WidgetAdapter<Window, AdwWindowState>> //<T>>>

}

impl AdwWindowState //<T>
    //where T: GtkWindowExt + WidgetExt + IsA<Widget> + AdwWindowExt //+ IsA<Widget>, //MayDowncastTo<Widget> +
          //P: WidgetStateContainer + Clone
{

    pub fn new<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce()-> Window
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

    pub fn with_content<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce()-> Window,
            WSC: DynWidgetStateContainer
    {

        let sc = Self::new(window_fn);

        sc.set_content(content_state);//Some(content_state));

        sc

    }

    pub fn builder<F>(window_fn: F) -> Rc<Self>
        where F: FnOnce(WindowBuilder) -> Window
    {

        let builder = Window::builder();

        let aws = Rc::new_cyclic(|weak_self|
        {
            Self
            {

                //weak_self: weak_self.clone(),
                widget_adapter: WidgetAdapter::new(&window_fn(builder), weak_self)

            }

        });

        //let any_this: &dyn Any = &aws;

        //let wsc = any_this.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        //StateContainers::get().add(wsc);

        StateContainers::get().widget_state_ref().add(&aws);

        aws

    }

    pub fn builder_with_content<F, WSC>(window_fn: F, content_state: &Rc<WSC>) -> Rc<Self>
        where F: FnOnce(WindowBuilder) -> Window,
            WSC: DynWidgetStateContainer
    {

        let sc = Self::builder(window_fn);

        sc.set_content(content_state); //Some(content_state));

        sc

    }

    impl_weak_self_methods!(widget_adapter);

    /*
    pub fn window(&self) -> Rc<WidgetAdapter<Window, AdwWindowState>> //<T>>>
    {
        
        self.window_adapter.clone()

    }

    pub fn window_ref(&self) -> &WidgetAdapter<Window, AdwWindowState>
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
    pub fn dyn_set_content<WSC: WidgetStateContainer>(&self, child_state: Option<&Rc<WSC>>) //(&self, child_state: Option<&Rc<dyn WidgetStateContainer>>)
    {

        if let Some(state) = child_state
        {

            self.widget_adapter.widget().set_content(Some(&state.dyn_widget_adapter().widget()))
            
        }
        /*
        else
        {

            self.window.widget().set_content(None); //Option::<&_>::None);

        }
        */
    }
    */

    /*
    pub fn set_content<WSC: WidgetStateContainer>(&self, child_state: Option<&Rc<WSC>>)
    {

        if let Some(state) = child_state
        {

            self.window_adapter.widget().set_content(Some(&state.dyn_adapted_widget().widget()))
            
        }*/
        /*
        else
        {

            self.window.widget().set_content(Option::None);

        }
        */

    //}

    pub fn set_content<WSC: DynWidgetStateContainer>(&self, child_state: &Rc<WSC>) -> WidgetUpgradeResult
    {

        self.widget_adapter.widget()?.set_content(Some(&child_state.dyn_widget_adapter().widget()?));

        Ok(())

    }

}

impl_widget_state_container_traits!(Window, AdwWindowState);

//impl<T> AdwWindowState<T>
//    where T: GtkWindowExt + WidgetExt + AdwWindowExt, //+ IsA<T>, //+ MayDowncastTo<Widget>,
          //P: WidgetStateContainer
//{
//}

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

/*
impl AsAny for AdwWindowState //<T>
    //where T: GtkWindowExt + WidgetExt,
          //P: WidgetStateContainer
{

    fn as_any(&self) -> &dyn std::any::Any
    {
        
        self

    }

}
*/

/*
impl DynWidgetStateContainer for AdwWindowState //<T>
    //where T: GtkWindowExt + WidgetExt, //+ MayDowncastTo<Widget> + IsA<T>
          //P: WidgetStateContainer
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

