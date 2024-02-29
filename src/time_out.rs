use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell};

use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}}; //, prelude::Continue};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

//use corlib::{NonOption, rc_self_setup, impl_rfc_set_rfc_field_delegate, impl_rfc_deref_get_rfc_field_delegate, impl_rfc_deref_get_set_rfc_field_delegate, impl_rfc_field_subscription_delegate, impl_rfc_field_subscribe_delegate, impl_rfc_field_unsubscribe_delegate}; //, impl_rfc_field_subscribe_move_delegate, impl_rfc_field_unsubscribe_move_delegate, impl_rfc_field_subscription_move_delegate}; //impl_rfc_get_rfc_field_delegate, 

use corlib::{NonOption, rc_self_setup, impl_rfc_borrow_get, impl_rfc_borrow_mut_set, impl_rfc_borrow_mut_get_set, impl_rfc_borrow_mut_subscribe, impl_rfc_borrow_mut_unsubscribe, impl_rfc_borrow_mut_subscription, impl_rfc_get_weak_self}; 

//

use corlib::rc_default::RcDefault;

use paste::paste;

//use gtk::glib;

//use glib::clone;

struct PrivateTimeOutFileds //InternalTimeOut
{

    pub interval: Duration,
    pub source_id: Option<SourceId>,
    pub on_time_out: ListEvent<Rc<TimeOut>>,
    //pub reoccurs: bool,
    //pub is_active: bool

}

impl PrivateTimeOutFileds //InternalTimeOut
{

    pub fn new(interval: Duration) -> Self //, reoccurs: bool) -> Self
    {

        Self
        {

            interval,
            source_id: None,
            on_time_out: ListEvent::new(),
            //reoccurs,  //: false,
            //is_active: false

        }

    }

}

pub type RcTimeOut = Rc<TimeOut>;

///
/// Wraps a glib::source::timeout_add_local function and a glib::source::SourceId struct into a single easy to use object.
/// 
/// On timeout it raises an event.
/// 
pub struct TimeOut
{

    fields: RefCell<PrivateTimeOutFileds>, //internal_timeout InternalTimeOut>,
    reoccurs: Cell<bool>,
    weak_self: RefCell<NonOption<Weak<TimeOut>>>

}

impl TimeOut
{

    pub fn new(interval: Duration, reoccurs: bool) -> Rc<Self>
    {

        let rc_self = Rc::new(Self
        {

            fields: RefCell::new(PrivateTimeOutFileds::new(interval)), //, reoccurs)), //internal_timeout
            reoccurs: Cell::new(reoccurs),
            weak_self: RefCell::new(NonOption::invalid())

            /*
            interval,
            source_id: None,
            on_time_out: ListEvent::new(),
            reoccurs: false
            */
        });

        rc_self_setup!(rc_self, weak_self);

        rc_self

    }

    pub fn new_once(interval: Duration) -> Rc<Self>
    {

        let rc_self = Rc::new(Self
        {

            fields: RefCell::new(PrivateTimeOutFileds::new(interval)), //, false)), //internal_timeout
            reoccurs: Cell::new(false),
            weak_self: RefCell::new(NonOption::invalid())

        });

        rc_self_setup!(rc_self, weak_self);

        rc_self

    }

    //impl_rfc_deref_get_rfc_field_delegate!(fields, interval, Duration);

    /*
    pub fn get_weak_self(&self) -> Weak<Self>
    {

        self.weak_self.borrow().get_ref().clone()

    }
    */

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

    //impl_rfc_deref_get_rfc_field_delegate!(fields, is_active, bool);

    //impl_rfc_borrow_get!(fields, is_active, bool);

    pub fn is_active(&self) -> bool
    {

        self.fields.borrow().source_id.is_some()

    }

    //impl_rfc_deref_get_set_rfc_field_delegate!(fields, reoccurs, bool);

    //impl_rfc_borrow_mut_get_set!(fields, reoccurs, bool);

    pub fn get_reoccurs(&self) -> bool
    {

        self.reoccurs.get()

    }

    pub fn set_reoccurs(&self, reoccurs: bool)
    {

        self.reoccurs.set(reoccurs);

    }

    pub fn set_reoccurs_t(&self)
    {

        self.reoccurs.set(true)

    }

    pub fn set_reoccurs_f(&self)
    {

        self.reoccurs.set(false)

    }

    /*
    pub fn subscribe_on_time_out(&self)
    {

        let mut fields_mut = self.fields.borrow_mut();

        fields_mut.on_time_out.subscribe(f)

    }
    */

    //impl_rfc_field_subscription_delegate!(fields, on_time_out, SenderEventFunc<Rc<Self>>);

    impl_rfc_borrow_mut_subscription!(fields, on_time_out, SenderEventFunc<Rc<Self>>);

    //impl_rfc_field_subscription_move_delegate!(fields, on_time_out, SenderEventFunc<Rc<Self>>);

    pub fn start(&self) -> bool
    {

        let mut fields_mut = self.fields.borrow_mut();

        if let None = fields_mut.source_id
        {

            let weak_self = self.weak_self.borrow().get_ref().clone();

            fields_mut.source_id = Some(timeout_add_local(fields_mut.interval, move || {  
            
                if let Some(this) = weak_self.upgrade()
                {

                    //let reoccurs;

                    {

                        let mut fields_mut = this.fields.borrow_mut();

                        fields_mut.on_time_out.raise(&this);
    
                        //reoccurs = fields_mut.reoccurs;

                    }

                    if this.get_reoccurs() //reoccurs
                    {

                        //Continue(true)

                        ControlFlow::Continue

                    }
                    else
                    {

                        //this.fields.borrow_mut().is_active = false;

                        //The presence source_id indecates whether or not the TimeOut is active

                        this.fields.borrow_mut().source_id = None;

                        //Continue(false)

                        ControlFlow::Break

                    }

                    //Continue(fields_ref.reoccurs)

                }
                else
                {

                    //Continue(false)

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

        /*
        if let Some(src_id) = &mut self.source_id
        {

            src_id.remove();

        }
        */

        let mut fields_mut = self.fields.borrow_mut();

        if fields_mut.source_id.is_some()//&& fields_mut.is_active
        {

            //let src_id = self.source_id.unwrap(); //expect("Can'f find source_id");

            let src_id_taken = fields_mut.source_id.take();

            let src_id = src_id_taken.unwrap();

            src_id.remove();

            //fields_mut.is_active = false;

            true

        }
        else {
            
            false

        }

    }

}

impl Drop for TimeOut
{

    fn drop(&mut self)
    {
        
        self.stop();
        
    }

}

impl RcDefault for TimeOut
{

    fn rc_default() -> Rc<Self>
    {

        Self::new(Duration::new(1, 0), false)
        
    }

}

/*
struct TimeOutInternal
{

    continue_: bool

}
*/

