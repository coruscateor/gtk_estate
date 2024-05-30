use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell};

use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

use corlib::{impl_get_ref, impl_get_weak_self_ref, impl_rfc_borrow_get, impl_rfc_borrow_mut_set, impl_rfc_borrow_mut_subscribe, impl_rfc_borrow_mut_subscription, impl_rfc_borrow_mut_unsubscribe}; 

use paste::paste;

struct PrivateTimeOutFileds<T>
    where T: 'static
{

    pub interval: Duration,
    pub source_id: Option<SourceId>,
    pub on_time_out: ListEvent<Rc<TimeOut<T>>>

}

impl<T> PrivateTimeOutFileds<T>
    where T: 'static
{

    pub fn new(interval: Duration) -> Self
    {

        Self
        {

            interval,
            source_id: None,
            on_time_out: ListEvent::new(),

        }

    }

}

pub type RcTimeOut<T = ()> = Rc<TimeOut<T>>;

///
/// Wraps a glib::source::timeout_add_local function and a glib::source::SourceId struct into a single easy to use object.
/// 
/// On timeout it raises an event.
/// 
pub struct TimeOut<T = ()> // = ()
    where T: 'static
{

    fields: RefCell<PrivateTimeOutFileds<T>>,
    reoccurs: Cell<bool>,
    weak_self: Weak<Self>,
    state: T

}

impl<T> TimeOut<T>
    where T: Default + 'static
{

    pub fn new(interval: Duration, reoccurs: bool) -> Rc<Self>
    {

        let res = Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateTimeOutFileds::new(interval)),
                reoccurs: Cell::new(reoccurs),
                weak_self: weak_self.clone(),
                state: T::default()
    
            }

        });

        res

    }

    pub fn new_once(interval: Duration) -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                fields: RefCell::new(PrivateTimeOutFileds::new(interval)),
                reoccurs: Cell::new(false),
                weak_self: weak_self.clone(),
                state: T::default()

            }

        })

    }

}

impl<T> TimeOut<T>
    where T: 'static
{

    pub fn with_state(interval: Duration, reoccurs: bool, state: T) -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateTimeOutFileds::new(interval)),
                reoccurs: Cell::new(reoccurs),
                weak_self: weak_self.clone(),
                state
    
            }

        })

    }

    pub fn with_state_ref(interval: Duration, reoccurs: bool, state: &T) -> Rc<Self>
        where T: Clone
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                fields: RefCell::new(PrivateTimeOutFileds::new(interval)),
                reoccurs: Cell::new(reoccurs),
                weak_self: weak_self.clone(),
                state: state.clone()
    
            }

        })

    }

    pub fn new_once_with_state(interval: Duration, state: T) -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                fields: RefCell::new(PrivateTimeOutFileds::new(interval)),
                reoccurs: Cell::new(false),
                weak_self: weak_self.clone(),
                state

            }

        })

    }

    pub fn new_once_with_state_ref(interval: Duration, state: &T) -> Rc<Self>
        where T: Clone
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                fields: RefCell::new(PrivateTimeOutFileds::new(interval)),
                reoccurs: Cell::new(false),
                weak_self: weak_self.clone(),
                state: state.clone()

            }

        })

    }

    impl_get_weak_self_ref!();

    impl_rfc_borrow_get!(fields, interval, Duration);
    
    pub fn set_interval(&self, value: Duration) -> bool
    {

        {

            let mut fields_mut = self.fields.borrow_mut();

            fields_mut.interval = value;

        }

        //If fields_mut.source_id.is_some()

        //Restart the timer if it was active

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

    pub fn reoccurs(&self) -> bool
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

    impl_get_ref!(state, T);

    impl_rfc_borrow_mut_subscription!(fields, on_time_out, SenderEventFunc<Rc<Self>>);

    pub fn start(&self) -> bool
    {

        let mut fields_mut = self.fields.borrow_mut();
        
        let weak_self = self.weak_self.clone();

        if fields_mut.source_id.is_none()
        {

            fields_mut.source_id = Some(timeout_add_local(fields_mut.interval, move ||
            {  
            
                if let Some(this) = weak_self.upgrade()
                {

                    {

                        let mut fields_mut = this.fields.borrow_mut();

                        fields_mut.on_time_out.raise(&this);
    
                    }

                    if this.reoccurs()
                    {

                        ControlFlow::Continue

                    }
                    else
                    {

                        //The presence source_id indicates whether or not the TimeOut is active.

                        this.fields.borrow_mut().source_id = None;

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
        else
        {
            
            false

        }

    }

}

impl<T> Drop for TimeOut<T>
    where T: 'static
{

    fn drop(&mut self)
    {
        
        self.stop();
        
    }

}

/*
impl<T> RcDefault for TimeOut<T>
{

    fn rc_default() -> Rc<Self>
    {

        Self::new(Duration::new(1, 0), false)
        
    }

}
*/

