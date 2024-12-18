use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell, Ref};

use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

use corlib::{impl_get_ref, impl_rfc_borrow_get};

use crate::impl_weak_self_methods; 

pub struct PrivateSimpleTimeOutFileds
{

    pub interval: Duration,
    pub source_id: Option<SourceId>

}

impl PrivateSimpleTimeOutFileds
{

    pub fn new(interval: Duration) -> Self
    {

        Self
        {

            interval,
            source_id: None

        }

    }

}

pub type RcSimpleTimeOut<T = ()> = Rc<SimpleTimeOut<T>>;

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
    on_time_out_fn: RefCell<Option<Box<SimpleTimeOutFn<T>>>>,
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
                on_time_out_fn: RefCell::new(None),
                weak_self: weak_self.clone(),
                state: T::default()
    
            }

        })

    }

    pub fn with_fn<F>(interval: Duration, on_time_out_fn: F) -> Rc<Self>
        where F: Fn(&Rc<Self>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                on_time_out_fn: RefCell::new(Some(Box::new(on_time_out_fn))),
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
                on_time_out_fn: RefCell::new(None),
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
                on_time_out_fn: RefCell::new(None),
                weak_self: weak_self.clone(),
                state: state.clone()
    
            }

        })

    }

    pub fn with_fn_and_state<F>(interval: Duration, on_time_out_fn: F, state: T) -> Rc<Self>
        where F: Fn(&Rc<Self>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                on_time_out_fn: RefCell::new(Some(Box::new(on_time_out_fn))),
                weak_self: weak_self.clone(),
                state
    
            }

        })

    }

    pub fn with_fn_and_state_ref<F>(interval: Duration, on_time_out_fn: F, state: &T) -> Rc<Self>
        where F: Fn(&Rc<Self>) -> bool + 'static,
              T: Clone
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)),
                on_time_out_fn: RefCell::new(Some(Box::new(on_time_out_fn))),
                weak_self: weak_self.clone(),
                state: state.clone()

            }

        })

    }

    impl_weak_self_methods!();

    impl_rfc_borrow_get!(fields, interval, Duration);

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

    pub fn set_on_time_out_fn<F>(&self, on_time_out_fn: F)
        where F: Fn(&Rc<Self>) -> bool + 'static
    {

        let mut fn_mut = self.on_time_out_fn.borrow_mut();

        *fn_mut = Some(Box::new(on_time_out_fn));

    }

    pub fn has_on_time_out_fn(&self) -> bool
    {

        self.on_time_out_fn.borrow().is_some()

    }

    pub fn remove_on_time_out_fn(&self) -> bool
    {

        let val = self.has_on_time_out_fn();

        self.remove_on_time_out_fn_only();

        val

    }

    pub fn remove_on_time_out_fn_only(&self)
    {

        let mut fn_mut = self.on_time_out_fn.borrow_mut();

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

                    if let Some(on_time_out_fn) = this.on_time_out_fn.borrow().as_ref()
                    {
                        
                        if on_time_out_fn(&this)
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


