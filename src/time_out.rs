use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell, Ref};

use adw::glib::clone::Downgrade;
use adw::glib::{timeout_add_local_full, timeout_add_seconds_local, Priority};
use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

use corlib::{get_some, impl_get_ref, impl_rfc_borrow_get};

use corlib::cell::RefCellStore;

use crate::impl_weak_self_methods;

#[derive(Clone, Copy)]
pub enum TimeOutRunType
{

    Seconds(u32),
    Milliseconds(Duration),
    Full(Duration, Priority)

}

pub type RcTimeOut<P> = Rc<TimeOut<P>>;

pub type TimeOutFnMut<P> = dyn FnMut(&RcTimeOut<P>, Rc<P>) -> bool;

pub type BoxTimeOutFnMut<P> = Box<TimeOutFnMut<P>>;

/*
pub type RcTimeOut = Rc<TimeOut>;

pub type TimeOutFn = dyn Fn() -> bool;

pub type BoxTimeOutFn = Box<TimeOutFn>;
*/

//pub type RcTimeOutFn<P> = Rc<TimeOutFn<P>>;

pub struct TimeOutMutState<P>
    where P: 'static
{

    //pub run_type: TimeOutRunType,
    //pub interval: Duration,
    pub source_id: Option<SourceId>,
    pub time_out_fn: Option<BoxTimeOutFnMut<P>> //Option<Box<dyn FnMut(Rc<Self>, Rc<P>)>>

}

impl<P> TimeOutMutState<P>
    where P: 'static
{

    pub fn new(time_out_fn: Option<BoxTimeOutFnMut<P>>) -> Self //run_type: TimeOutRunType,  //interval: Duration,
    {

        Self
        {

            //run_type,
            //interval,
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
pub struct TimeOut<P>
    where P: 'static
{

    mut_state: RefCellStore<TimeOutMutState<P>>,
    run_type: Cell<TimeOutRunType>,
    //on_time_out_fn: RefCell<Option<Box<dyn FnMut(T)>>>,
    weak_self: Weak<Self>,
    weak_parent: Weak<P>

}

impl<P> TimeOut<P>
    where P: 'static
{

    pub fn new(run_type: TimeOutRunType, weak_parent: &Weak<P>) -> Rc<Self>
    {   

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                mut_state: RefCellStore::new(TimeOutMutState::new( None)), //run_type,
                run_type: Cell::new(run_type),
                weak_self: weak_self.clone(),
                weak_parent: weak_parent.clone()
    
            }

        })

    }

    pub fn seconds(interval: u32, weak_parent: &Weak<P>) -> Rc<Self>
    {   

      Self::new(TimeOutRunType::Seconds(interval), weak_parent)

    }

    pub fn milliseconds(interval: Duration, weak_parent: &Weak<P>) -> Rc<Self>
    {   

      Self::new(TimeOutRunType::Milliseconds(interval), weak_parent)

    }

    pub fn full(interval: Duration, priority: Priority, weak_parent: &Weak<P>) -> Rc<Self>
    {   

      Self::new(TimeOutRunType::Full(interval, priority), weak_parent)

    }

    pub fn with_fn<F>(run_type: TimeOutRunType, weak_parent: &Weak<P>, time_out_fn: F) -> Rc<Self>
        where F: Fn(&Rc<TimeOut<P>>, Rc<P>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                mut_state: RefCellStore::new(TimeOutMutState::new(Some(Box::new(time_out_fn)))), //run_type, 
                run_type: Cell::new(run_type),
                weak_self: weak_self.clone(),
                weak_parent: weak_parent.clone()

            }

        })

    }

    pub fn with_parent(run_type: TimeOutRunType, parent: &Rc<P>) -> Rc<Self>
    {   

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                mut_state: RefCellStore::new(TimeOutMutState::new(None)), //run_type, 
                run_type: Cell::new(run_type),
                weak_self: weak_self.clone(),
                weak_parent: parent.downgrade()
    
            }

        })

    }

    pub fn with_parent_and_fn<F>(run_type: TimeOutRunType, parent: &Rc<P>, time_out_fn: F) -> Rc<Self>
        where F: Fn(&Rc<TimeOut<P>>, Rc<P>) -> bool + 'static
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                mut_state: RefCellStore::new(TimeOutMutState::new(Some(Box::new(time_out_fn)))), //run_type,
                run_type: Cell::new(run_type),
                weak_self: weak_self.clone(),
                weak_parent: parent.downgrade()

            }

        })

    }

    impl_weak_self_methods!();

    //No more borrowing from RefCells directly.

    //impl_rfc_borrow_get!(fields, interval, Duration);

    pub fn set_run_type(&self, value: TimeOutRunType) -> bool
    {

        self.run_type.set(value);

        /*
        self.mut_state.borrow_mut(|mut state|
        {

            state.run_type = value;

        });
        */

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

    pub fn run_type(&self) -> TimeOutRunType
    {

        self.run_type.get()

        /*
        self.mut_state.borrow(|state|
        {

            state.run_type

        })
        */

    }

    /*
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
    */

    pub fn is_active(&self) -> bool
    {

        //Wrong, use a Cell<bool>.

        self.mut_state.borrow(|state|
        {

            state.source_id.is_some()

        })

    }

    //impl_get_ref!(state, T);

    pub fn set_time_out_fn<F>(&self, time_out_fn: F)
        where F: Fn(&Rc<TimeOut<P>>, Rc<P>) -> bool  + 'static
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

            state.time_out_fn.is_some()

        })

    }

    pub fn remove_time_out_fn(&self)
    {

        self.mut_state.borrow_mut(|mut state|
        {

            state.time_out_fn.take();

        });

    }

    pub fn try_remove_time_out_fn(&self) -> bool
    {

        self.mut_state.borrow_mut(|mut state|
        {

            let time_out_fn_opt = state.time_out_fn.take();

            if let Some(_time_out_fn) = time_out_fn_opt
            {

                true

            }
            else
            {

                false   

            }
            
        })

    }

    ///
    /// Starts the TimeOut.
    /// 
    /// Moves Rc<Self> into the scope of a glib::source::timeout_add_local call and attepts to upgrade weak_parent
    /// 
    pub fn start(&self) -> bool
    {

        self.mut_state.borrow_mut(|mut state|
        {

            if state.source_id.is_none()
            {

                let weak_self = self.weak_self.clone();

                //let weak_parent = self.weak_parent.clone();

                let func = move ||
                {

                    let this;

                    if let Some(val) =  weak_self.upgrade()
                    {

                        this = val;

                    }
                    else
                    {

                        return ControlFlow::Break;
                        
                    }

                    this.mut_state.borrow_mut(|mut state|
                    {

                        let parent;

                        let mut control_flow = ControlFlow::Break;

                        if let Some(this_parent) = this.weak_parent.upgrade()
                        {
    
                            parent = this_parent;
    
                        }
                        else
                        {

                            state.source_id = None;

                            return control_flow;
                            
                        }
                        
                        if let Some(time_out_fn) = state.time_out_fn.as_mut()
                        {

                            if time_out_fn(&this, parent)
                            {

                                control_flow = ControlFlow::Continue

                            }

                        }

                        if control_flow.is_break()
                        {

                            state.source_id = None;

                        }

                        control_flow

                    })

                };

                match self.run_type() //state.run_type
                {

                    TimeOutRunType::Seconds(seconds) =>
                    {

                        state.source_id = Some(timeout_add_seconds_local(seconds, func));

                    }
                    TimeOutRunType::Milliseconds(duration) =>
                    {

                        state.source_id = Some(timeout_add_local(duration, func));

                    }
                    TimeOutRunType::Full(duration, priority) =>
                    {

                        state.source_id = Some(timeout_add_local_full(duration, priority, func));

                    }

                }

                true

            }
            else
            {

                false

            }

        })

        /*
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
        */

    }

    pub fn stop(&self) -> bool
    {

        self.mut_state.borrow_mut(|mut state|
        {

            if let Some(source_id) = state.source_id.take()
            {

                source_id.remove();

                true
                
            }
            else
            {

                false
                
            }

        })

        /*
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
        */

    }

}

impl<P> Drop for TimeOut<P>
{

    fn drop(&mut self)
    {
        
        self.stop();
        
    }

}


