use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell, Ref};

use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}}; //, prelude::Continue};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

use corlib::{NonOption, rc_self_setup, impl_rfc_borrow_get, impl_rfc_borrow_mut_set, impl_rfc_borrow_mut_get_set, impl_rfc_borrow_mut_subscribe, impl_rfc_borrow_mut_unsubscribe, impl_rfc_borrow_mut_subscription, impl_rfc_get_weak_self}; 

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

pub type RcSimpleTimeOut = Rc<SimpleTimeOut>;

//pub trait SimpleTimeOutFn = Fn(&SimpleTimeOut) -> bool;

pub type SimpleTimeOutFn = dyn Fn(&Rc<SimpleTimeOut>) -> bool;

///
/// Wraps a glib::source::timeout_add_local function and a glib::source::SourceId struct into a single easy to use object.
/// 
/// On timeout it calls a function which returns a value indicating wheher or not the timeout should continue.
/// 
pub struct SimpleTimeOut //<F>
    //where F: Fn(&Rc<Self>) -> bool + 'static
{

    fields: RefCell<PrivateSimpleTimeOutFileds>, //<F>
    function: RefCell<Option<Box<SimpleTimeOutFn>>>,
    weak_self: RefCell<NonOption<Weak<Self>>>,
    
}

impl SimpleTimeOut //<F>
    //where F: Fn(&Rc<Self>) -> bool + 'static
{

    pub fn new(interval: Duration) -> Rc<Self> //, opt_function: Option<F>) -> Rc<Self>
    {

        /*
        let function: F;

        if let Some(val) = opt_function
        {

            function = val;

        }
        else {
            
            function = |_| false;

        }
        */

        let rc_self = Rc::new(Self
        {

            fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)), //, function)),
            function: RefCell::new(None),
            weak_self: RefCell::new(NonOption::invalid()),

        });

        rc_self_setup!(rc_self, weak_self);

        rc_self

    }

    pub fn with_fn<F>(interval: Duration, function: F) -> Rc<Self>
        where F: Fn(&Rc<Self>) -> bool + 'static
    {
        let rc_self = Rc::new(Self
        {

            fields: RefCell::new(PrivateSimpleTimeOutFileds::new(interval)), //with_fn(interval, function)),
            function: RefCell::new(Some(Box::new(function))),
            weak_self: RefCell::new(NonOption::invalid()),

        });

        rc_self_setup!(rc_self, weak_self);

        rc_self

    }

    impl_rfc_get_weak_self!();

    pub fn set_interval(&self, value: Duration) -> bool
    {

        {

            let mut fields_mut = self.fields.borrow_mut();

            fields_mut.interval = value;

        }

        //if fields_mut.source_id.is_some()

        //retart the timer if it was active

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

    /* 
    pub fn get_function(&self) -> F
    {

        self.fields.borrow().function

    }
    */

    /*
    pub fn get_private_fields_ref(&self) -> Ref<PrivateSimpleTimeOutFileds<F>>
    {

        self.fields.borrow()

    }
    */

    pub fn set_function<F>(&self, function: F)
        where F: Fn(&Rc<Self>) -> bool + 'static
    {

        //let mut fields = self.fields.borrow_mut();

        //fields.function = function;

        //fields.opt_function = Some(function);

        let mut fn_mut = self.function.borrow_mut();

        *fn_mut = Some(Box::new(function));

    }

    /*
    pub fn try_get_function_ref(&self) -> Option<&Box<SimpleTimeOutFn>>
    {

        self.function.borrow().as_ref()

    }
    */

    pub fn has_function(&self) -> bool
    {

        //self.fields.borrow().opt_function.is_some()

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

        if fields_mut.source_id.is_none() //let None = fields_mut.source_id //&& fields_mut.opt_function.is_some()
        {

            /*
            if self.function.borrow().is_some()
            {
            */

            let weak_self = self.weak_self.borrow().get_ref().clone();

            fields_mut.source_id = Some(timeout_add_local(fields_mut.interval, move || {  
            
                if let Some(this) = weak_self.upgrade()
                {

                    if let Some(function) = this.function.borrow().as_ref() //&this.fields.borrow().opt_function
                    {
                        
                        if function(&this) //if (this.fields.borrow().function)(&this)
                        {

                            //Continue(true)

                            ControlFlow::Continue

                        }
                        else
                        {

                            //The presence source_id indecates whether or not the TimeOut is active

                            this.fields.borrow_mut().source_id = None;

                            //Continue(false)

                            ControlFlow::Break

                        }

                    }
                    else
                    {

                        //Continue(false)
                        
                        ControlFlow::Break

                    }

                }
                else
                {

                    //Continue(false)

                    ControlFlow::Break

                }
            
            }));

            true

            /*
            }
            else
            {
    
                false
    
            }
            */

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

impl Drop for SimpleTimeOut //<F>
    //where F: Fn(&Rc<Self>) -> bool
{

    fn drop(&mut self)
    {
        
        self.stop();
        
    }

}


