use std::fmt::Debug;
use std::{time::Duration, rc::Weak};

use std::rc::Rc;

use std::cell::{Cell, RefCell, Ref};

//use adw::glib::clone::{self, Downgrade};

use gtk::glib::{timeout_add_local_full, timeout_add_seconds_local, Priority};

//use gtk4 as gtk;

use gtk::{glib::source::{timeout_add_local, SourceId}};

use gtk::{glib::ControlFlow};

use corlib::events::{ListEvent, SenderEventFunc};

use corlib::{get_some, impl_get_ref, impl_rfc_borrow_get};

use corlib::cell::{RcRefCellStore, RefCellStore};

use gtk::glib::clone;

//use crate::impl_weak_self_methods;

#[derive(Clone, Copy, Debug)]
pub enum TimeOutRunType
{

    Seconds(u32),
    Milliseconds(Duration),
    Full(Duration, Priority)

}

pub type TimeOutFn<P> = dyn Fn(Rc<P>) -> bool;

pub type RcTimeOutFn<P> = Rc<TimeOutFn<P>>;

//pub type RcTimeOut<P> = Rc<TimeOut<P>>;

//pub type TimeOutFnMut<P> = dyn FnMut(&RcTimeOut<P>, Rc<P>) -> bool;

//pub type BoxTimeOutFnMut<P> = Box<TimeOutFnMut<P>>;

/*
pub type RcTimeOut = Rc<TimeOut>;

pub type TimeOutFn = dyn Fn() -> bool;

pub type BoxTimeOutFn = Box<TimeOutFn>;
*/

//pub type RcTimeOutFn<P> = Rc<TimeOutFn<P>>;

///
/// Wraps a glib::source::timeout_add_local function and a glib::source::SourceId struct into a single easy to use object.
/// 
/// On timeout it calls a function which returns a value indicating wheher or not the timeout should continue.
/// 
pub struct TimeOut<P>
    where P: 'static
{

    source_id:  RcRefCellStore<Option<SourceId>>,
    time_out_fn: RefCellStore<Option<RcTimeOutFn<P>>>,
    run_type: Cell<TimeOutRunType>,
    weak_parent: Weak<P>

}

impl<P> TimeOut<P>
    where P: 'static
{

    pub fn new(run_type: TimeOutRunType, weak_parent: &Weak<P>) -> Self
    {  

        Self
        {

            source_id:  Rc::new(RefCellStore::new(None)),
            time_out_fn: RefCellStore::new(None),
            run_type: Cell::new(run_type),
            weak_parent: weak_parent.clone()

        }

    }

    pub fn seconds(interval: u32, weak_parent: &Weak<P>) -> Self
    {   

      Self::new(TimeOutRunType::Seconds(interval), weak_parent)

    }

    pub fn milliseconds(interval: Duration, weak_parent: &Weak<P>) -> Self
    {   

      Self::new(TimeOutRunType::Milliseconds(interval), weak_parent)

    }

    pub fn full(interval: Duration, priority: Priority, weak_parent: &Weak<P>) -> Self
    {   

      Self::new(TimeOutRunType::Full(interval, priority), weak_parent)

    }

    pub fn with_fn<F>(run_type: TimeOutRunType, weak_parent: &Weak<P>, time_out_fn: Rc<F>) -> Self
        where F: Fn(Rc<P>) -> bool + 'static
    {

        Self
        {

            source_id:  Rc::new(RefCellStore::new(None)),
            time_out_fn: RefCellStore::new(Some(time_out_fn)),
            run_type: Cell::new(run_type),
            weak_parent: weak_parent.clone()

        }

    }

    pub fn with_parent(run_type: TimeOutRunType, parent: &Rc<P>) -> Self
    {   

        Self
        {

            source_id:  Rc::new(RefCellStore::new(None)),
            time_out_fn: RefCellStore::new(None),
            run_type: Cell::new(run_type),
            weak_parent: Rc::downgrade(parent)

        }

    }

    pub fn with_parent_and_fn<F>(run_type: TimeOutRunType, parent: &Rc<P>, time_out_fn: Rc<F>) -> Self
        where F: Fn(Rc<P>) -> bool + 'static
    {

        Self
        {

            source_id:  Rc::new(RefCellStore::new(None)),
            time_out_fn: RefCellStore::new(Some(time_out_fn)),
            run_type: Cell::new(run_type),
            weak_parent: Rc::downgrade(parent)

        }

    }

    //No more borrowing from RefCells directly.

    //impl_rfc_borrow_get!(fields, interval, Duration);

    pub fn restart(&self) -> bool
    {

        if self.stop()
        {

            self.start()

        }
        else
        {
         
            false
            
        }

    }

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

        self.restart()

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

        self.source_id.borrow(|state|
        {

            state.is_some()

        })

    }

    //impl_get_ref!(state, T);

    pub fn set_time_out_fn<F>(&self, time_out_fn: Rc<F>) -> bool
        where F: Fn(Rc<P>) -> bool  + 'static
    {

        self.time_out_fn.set(Some(time_out_fn));

        /*
        self.time_out_fn.borrow_mut_with_param(time_out_fn, |mut state, time_out_fn|
        {

            state.time_out_fn = Some(Box::new(time_out_fn));
            
        });
        */

        self.restart()

    }

    pub fn set_run_type_and_time_out_fn<F>(&self, value: TimeOutRunType, time_out_fn: Rc<F>) -> bool
        where F: Fn(Rc<P>) -> bool  + 'static
    {

        self.run_type.set(value);

        self.time_out_fn.set(Some(time_out_fn));

        //Retart the timer if it was active

        self.restart()

    }

    pub fn has_time_out_fn(&self) -> bool
    {

        self.time_out_fn.borrow(|state|
        {

            state.is_some()

        })

    }

    pub fn time_out_fn<F>(&self) -> Option<Rc<dyn Fn(Rc<P>) -> bool  + 'static>>
    {

        self.time_out_fn.get()

    }

    pub fn remove_time_out_fn<F>(&self)
    {

        self.time_out_fn.set(None);

    }

    pub fn take_time_out_fn<F>(&self) -> Option<Rc<dyn Fn(Rc<P>) -> bool  + 'static>>
    {

        let opt_time_out_fn = self.time_out_fn.borrow_mut(|mut state|
        {

            state.take()

        });

        if opt_time_out_fn.is_some()
        {

            self.stop();

        }

        opt_time_out_fn

    }

    /*
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
    */

    ///
    /// Starts the TimeOut.
    /// 
    /// Moves Rc<Self> into the scope of a glib::source::timeout_add_local call and attepts to upgrade weak_parent
    /// 
    pub fn start(&self) -> bool
    {

        if !self.is_active()
        {

            let time_out_fn;

            if let Some(val) = self.time_out_fn.get()
            {

                time_out_fn = val;

            }
            else
            {

                return false;
                
            }

            if self.weak_parent.strong_count() == 0
            {

                return false;

            }

            let weak_parent = self.weak_parent.clone();

            let source_id = self.source_id.clone();

            let func = clone!( #[strong] source_id, move ||
            {

                let parent;

                let mut control_flow = ControlFlow::Break;

                if let Some(this_parent) = weak_parent.upgrade()
                {

                    parent = this_parent;

                }
                else
                {

                    source_id.set(None);

                    return control_flow;
                    
                }

                if time_out_fn(parent)
                {

                    control_flow = ControlFlow::Continue

                }
                else
                {

                    control_flow = ControlFlow::Break;
                    
                }

                if control_flow.is_break()
                {

                    source_id.set(None);

                }

                control_flow

            });

            match self.run_type() //state.run_type
            {

                TimeOutRunType::Seconds(seconds) =>
                {

                    source_id.set(Some(timeout_add_seconds_local(seconds, func)));

                }
                TimeOutRunType::Milliseconds(duration) =>
                {

                    source_id.set(Some(timeout_add_local(duration, func)));

                }
                TimeOutRunType::Full(duration, priority) =>
                {

                    source_id.set(Some(timeout_add_local_full(duration, priority, func)));

                }

            }

            true

        }
        else
        {

            false

        }


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

        self.source_id.borrow_mut(|mut state|
        {

            if let Some(source_id) = state.take()
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

impl<P> Debug for TimeOut<P>
    where P: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("TimeOut")
        .field("source_id", &self.source_id)
        //.field("time_out_fn", &self.time_out_fn)
        .field("time_out_fn", &"Exculded")
        .field("run_type", &self.run_type)
        .field("weak_parent", &self.weak_parent)
        .finish()
    }
}

impl<P> Drop for TimeOut<P>
{

    fn drop(&mut self)
    {
        
        self.stop();
        
    }

}


