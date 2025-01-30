use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell, Ref};

use adw::glib::clone::Downgrade;
use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

use corlib::{impl_get_ref, impl_rfc_borrow_get};

use corlib::cell::RefCellStore;

use crate::impl_weak_self_methods; 

pub type RcTimeOut<P> = Rc<TimeOut<P>>;

pub type TimeOutFn<P> = dyn Fn(Rc<TimeOut<P>>, Rc<P>) -> bool;

pub type BoxTimeOutFn<P> = Box<TimeOutFn<P>>;

//pub type RcTimeOutFn<P> = Rc<TimeOutFn<P>>;

struct MutState<P>
    where P: 'static
{

    pub interval: Duration,
    pub source_id: Option<SourceId>,
    pub time_out_fn: Option<BoxTimeOutFn<P>> //Option<Box<dyn FnMut(Rc<Self>, Rc<P>)>>

}

impl<P> MutState<P>
    where P: 'static
{

    pub fn new(interval: Duration, time_out_fn: Option<BoxTimeOutFn<P>>) -> Self
    {

        Self
        {

            interval,
            source_id: None,
            time_out_fn

        }

    }

}

///
/// Wraps a glib::source::timeout_add_local function and a glib::source::SourceId struct into a single easy to use object.
/// 
/// On timeout it calls a function which returns a value indicating wheher or not the timeout should continue.
/// 
pub struct TimeOutWithParent<P>
    where P: 'static
{

    mut_state: RefCellStore<MutState<P>>,
    //on_time_out_fn: RefCell<Option<Box<dyn FnMut(T)>>>,
    weak_self: Weak<Self>,
    weak_parent: Weak<P>

}

impl<P> TimeOutWithParent<P>
    where P: 'static
{

    pub fn new(interval: Duration, weak_parent: &Weak<P>) -> Rc<Self>
    {   

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                mut_state: RefCellStore::new(MutState::new(interval, None)),
                weak_self: weak_self.clone(),
                weak_parent: weak_parent.clone()
    
            }

        })

    }

    pub fn with_fn<F>(interval: Duration, weak_parent: &Weak<P>, time_out_fn: F) -> Rc<Self>
        where F: Fn(Rc<TimeOut<P>>, Rc<P>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                mut_state: RefCellStore::new(MutState::new(interval, Some(Box::new(time_out_fn)))),
                weak_self: weak_self.clone(),
                weak_parent: weak_parent.clone()

            }

        })

    }

    pub fn with_parent(interval: Duration, parent: &Rc<P>) -> Rc<Self>
    {   

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                mut_state: RefCellStore::new(MutState::new(interval, None)),
                weak_self: weak_self.clone(),
                weak_parent: parent.downgrade()
    
            }

        })

    }

    pub fn with_parent_and_fn<F>(interval: Duration, parent: &Rc<P>, time_out_fn: F) -> Rc<Self>
        where F: Fn(Rc<TimeOut<P>>, Rc<P>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                mut_state: RefCellStore::new(MutState::new(interval, Some(Box::new(time_out_fn)))),
                weak_self: weak_self.clone(),
                weak_parent: parent.downgrade()

            }

        })

    }

    impl_weak_self_methods!();

    //No more borrowing from RefCells directly.

    //impl_rfc_borrow_get!(fields, interval, Duration);

    pub fn set_interval(&self, value: Duration) -> bool
    {

        self.mut_state.borrow_mut(|mut state|
        {

            state.interval = value;

        });

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

        self.mut_state.borrow(|state|
        {

            state.source_id.is_some()

        })

    }

    //impl_get_ref!(state, T);

    pub fn set_time_out_fn<F>(&self, time_out_fn: F)
        where F: Fn(Rc<TimeOut<P>>, Rc<P>) -> bool  + 'static
    {

        self.mut_state.borrow_mut_with_param(time_out_fn, |mut state, time_out_fn|
        {

            state.time_out_fn = Some(Box::new(time_out_fn));
            
        });

    }

    pub fn has_time_out_fn(&self) -> bool
    {

        self.mut_state.borrow(|state|
        {

            state.source_id.is_some()

        })

    }

    pub fn remove_time_out_fn(&self) -> bool
    {

        self.mut_state.borrow_mut(|mut state|
        {

            let has_time_out_fn = state.time_out_fn.is_some();

            if has_time_out_fn
            {

                state.time_out_fn = None;

            }

            has_time_out_fn
            
        })

    }

    pub fn start(&self) -> bool
    {

        //self, 

        self.mut_state.borrow_mut(|mut state| //, this
        {

            if state.source_id.is_none()
            {

                let weak_self = self.weak_self.clone();

                state.source_id = Some(timeout_add_local(state.interval, move ||
                {

                    if let Some(this) = weak_self.upgrade()
                    {

                        let parent;

                        if let Some(this_parent) = this.weak_parent.upgrade()
                        {
    
                            parent = this_parent;

                        }
                        else
                        {

                            ControlFlow::Break
                            
                        }

                        this.mut_state.borrow_mut(|state|
                        {

                            if let Some(mut time_out_fn) = state.time_out_fn.as_mut()
                            {

                                if time_out_fn(this, parent)
                                {

                                    ControlFlow::Continue

                                }
                                else
                                {

                                    ControlFlow::Break
                                    
                                }

                            }

                        })

                    }

                }));


            }

        })


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

impl<T> Drop for TimeOut<T>
    where T: 'static
{

    fn drop(&mut self)
    {
        
        self.stop();
        
    }

}


