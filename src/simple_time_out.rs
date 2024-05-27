use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell, Ref};

use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}}; //, prelude::Continue};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

use corlib::{impl_get_ref, impl_get_weak_self_ref, impl_rfc_borrow_get, impl_rfc_borrow_mut_set, impl_rfc_borrow_mut_subscription, impl_rfc_borrow_mut_unsubscribe, rc_self_setup, NonOption}; 

//impl_rfc_borrow_mut_get_set, impl_rfc_borrow_mut_subscribe, impl_rfc_get_weak_self

pub struct PrivateSimpleTimeOutFileds //<F>
    //where F: Fn(&Rc<SimpleTimeOut>) -> bool + 'static //<F>
{

    pub interval: Duration,
    pub source_id: Option<SourceId>,
    //pub opt_function: Option<F>

}

impl PrivateSimpleTimeOutFileds //<F>
    //where F: Fn(&Rc<SimpleTimeOut>) -> bool + 'static //<F>
{

    pub fn new(interval: Duration) -> Self //, function: F) -> Self
    {

        Self
        {

            interval,
            source_id: None,
            //opt_function: None

        }

    }

    /*
    pub fn with_fn(interval: Duration, function: F) -> Self
    {

        Self
        {

            interval,
            source_id: None,
            //opt_function: Some(function)

        }

    }
    */

}

pub type RcSimpleTimeOut<T = ()> = Rc<SimpleTimeOut<T>>;

//pub trait SimpleTimeOutFn = Fn(&SimpleTimeOut) -> bool;

pub type SimpleTimeOutFn<T> = dyn Fn(&Rc<SimpleTimeOut<T>>) -> bool;

///
/// Wraps a glib::source::timeout_add_local function and a glib::source::SourceId struct into a single easy to use object.
/// 
/// On timeout it calls a function which returns a value indicating wheher or not the timeout should continue.
/// 
pub struct SimpleTimeOut<T = ()>
    where T: 'static
{

    fields: RefCell<PrivateSimpleTimeOutFileds>,
    function: RefCell<Option<Box<SimpleTimeOutFn<T>>>>,
    weak_self: Weak<Self>,
    state: T

}

impl<T> SimpleTimeOut<T>
    where T: Default + 'static
{

    pub fn new(interval: Duration) -> Rc<Self>
    {   

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                function: RefCell::new(None),
                weak_self: weak_self.clone(),
                state: T::default()
    
            }

        })

    }

    pub fn with_fn<F>(interval: Duration, function: F) -> Rc<Self>
        where F: Fn(&Rc<Self>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                function: RefCell::new(Some(Box::new(function))),
                weak_self: weak_self.clone(),
                state: T::default()

            }

        })

    }

}

impl<T> SimpleTimeOut<T>
{

    pub fn with_state(interval: Duration, state: T) -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                function: RefCell::new(None),
                weak_self: weak_self.clone(),
                state
    
            }

        })

    }

    pub fn with_state_ref(interval: Duration, state: &T) -> Rc<Self>
        where T: Clone
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                function: RefCell::new(None),
                weak_self: weak_self.clone(),
                state: state.clone()
    
            }

        })

    }

    pub fn with_fn_and_state<F>(interval: Duration, function: F, state: T) -> Rc<Self>
        where F: Fn(&Rc<Self>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                function: RefCell::new(Some(Box::new(function))),
                weak_self: weak_self.clone(),
                state
    
            }

        })

    }

    pub fn with_fn_and_state_ref<F>(interval: Duration, function: F, state: &T) -> Rc<Self>
        where F: Fn(&Rc<Self>) -> bool + 'static,
              T: Clone
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                function: RefCell::new(Some(Box::new(function))),
                weak_self: weak_self.clone(),
                state: state.clone()

            }

        })

    }

    //impl_rfc_get_weak_self!();

    impl_get_weak_self_ref!();

    pub fn set_interval(&self, value: Duration) -> bool
    {

        {

            let mut fields_mut = self.fields.borrow_mut();

            fields_mut.interval = value;

        }

        //If fields_mut.source_id.is_some()

        //Retart the timer if it was active

        if self.stop()
        {

            self.start()

        }
        else
        {
         
            false
            
        }

    }

    pub fn is_active(&self) -> bool
    {

        self.fields.borrow().source_id.is_some()

    }

    impl_get_ref!(state, T);

    pub fn set_function<F>(&self, function: F)
        where F: Fn(&Rc<Self>) -> bool + 'static
    {

        let mut fn_mut = self.function.borrow_mut();

        *fn_mut = Some(Box::new(function));

    }

    pub fn has_function(&self) -> bool
    {

        self.function.borrow().is_some()

    }

    pub fn remove_function(&self) -> bool
    {

        let val = self.has_function();

        self.remove_function_only();

        val

    }

    pub fn remove_function_only(&self)
    {

        let mut fn_mut = self.function.borrow_mut();

        *fn_mut = None;

    }

    pub fn start(&self) -> bool
    {

        let mut fields_mut = self.fields.borrow_mut();

        if fields_mut.source_id.is_none()
        {

            let weak_self = self.weak_self.clone();

            fields_mut.source_id = Some(timeout_add_local(fields_mut.interval, move ||
            {  
            
                if let Some(this) = weak_self.upgrade()
                {

                    if let Some(function) = this.function.borrow().as_ref()
                    {
                        
                        if function(&this)
                        {

                            ControlFlow::Continue

                        }
                        else
                        {

                            //The presence source_id indecates whether or not the TimeOut is active

                            this.fields.borrow_mut().source_id = None;

                            ControlFlow::Break

                        }

                    }
                    else
                    {
                        
                        ControlFlow::Break

                    }

                }
                else
                {

                    ControlFlow::Break

                }
            
            }));

            true

        }
        else
        {

            false

        }


    }

    pub fn stop(&self) -> bool
    {

        let mut fields_mut = self.fields.borrow_mut();

        if fields_mut.source_id.is_some()
        {

            let src_id_taken = fields_mut.source_id.take();

            let src_id = src_id_taken.unwrap();

            src_id.remove();

            true

        }
        else {
            
            false

        }

    }

}

impl<T> Drop for SimpleTimeOut<T>
    where T: 'static
{

    fn drop(&mut self)
    {
        
        self.stop();
        
    }

}


