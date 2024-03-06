use std::any::TypeId;

use std::time::Duration;

use std::{rc::*, any::Any};

use std::cell::{RefCell, Ref, RefMut};

use corlib::{impl_rfc_borrow_call, AsAny};

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use paste::paste;

use std::collections::{HashMap, HashSet};

use corlib::
{
    NonOption,
    RcByPtr,
    collections::UniqueItemList,
    impl_rfc_borrow,
    impl_rfc_borrow_mut,
    impl_rfc_borrow_and_mut

};

use gtk4 as gtk;

use gtk::{glib::object::ObjectExt};

use crate::{adapters::*, RcSimpleTimeOut, WidgetStateContainers, SimpleTimeOut};

/*
object.rs(27, 7): for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
only auto traits can be used as additional traits in a trait object
consider creating a new trait with all of these as supertraits and using that trait here instead: `trait NewTrait: std::any::Any + ApplicationExt {}`
*/

///
/// Indicates that the implementing object stores application related data.
/// 
pub trait ApplicationStateContainer : AsAny + Any //<'a>
{

    //fn application() -> &'a (dyn Any + ApplicationExt);

    fn application(&self) -> &(dyn StoredApplicationObject); //'a //Any +

}

///
/// Indicates that the implementing object stores widget related data.
/// 
pub trait WidgetStateContainer : AsAny + Any //<'a>
{

    fn widget(&self) -> &(dyn StoredWidgetObject); //'a  //Any + WidgetExt

}

static mut STATE_CONTAINERS: NonOption<Rc<StateContainers>> = NonOption::invalid(); 

fn get_state_containers() -> Rc<StateContainers>
{

    //check is correct thread?

    unsafe
    {

        STATE_CONTAINERS.get_ref().clone()

    }


}

fn try_get_state_containers() -> Option<Rc<StateContainers>>
{

    //check is correct thread?

    unsafe
    {

        let opt = STATE_CONTAINERS.try_get_ref();

        if let Some(sc) = opt
        {

            return Some(sc.clone());

        }

    }

    None

}

fn set_state_containers(state_containers: &Rc<StateContainers>)
{

    //check is correct thread?

    unsafe
    {

        STATE_CONTAINERS.set(state_containers.clone())

    }

}

struct InternalNonCollectionStateContainers
{

    pub application_state: NonOption<Rc<dyn ApplicationStateContainer>>,
    //widget_state: HashMap<TypeId, HashSet<RcByPtr<Rc<dyn WidgetStateContainer>>>>,
    pub weak_self: Weak<StateContainers>, //Self is a Reference Type!
    pub widget_states_to_remove: HashSet<RcByPtr<dyn WidgetStateContainer>> //Vec<Rc<dyn WidgetStateContainer>>

}

impl InternalNonCollectionStateContainers
{

    pub fn new(weak_self: &Weak<StateContainers>) -> Self
    {

        Self
        {

            application_state: NonOption::invalid(),
            //widget_state: HashMap::new(),
            weak_self: weak_self.clone(),
            widget_states_to_remove: HashSet::new()

        }

    }

}

pub struct StateContainers //<'a>
{

    nc_internals: RefCell<InternalNonCollectionStateContainers>,
    widget_state: RefCell<WidgetStateContainers>, //RefCell<HashMap<TypeId, HashSet<RcByPtr<Rc<dyn WidgetStateContainer>>>>>
    widget_state_removal_timeout: RcSimpleTimeOut,

    //application_state: NonOption<Rc<dyn ApplicationStateContainer>>, //<'a>
    //widget_state: HashMap<TypeId, HashSet<RcByPtr<Rc<dyn WidgetStateContainer>>>>,
    //weak_self: Weak<StateContainers> //Self is a Reference Type!

}

impl StateContainers //<'a>
{

    ///
    /// Call this once: prefereably in the main function.
    /// 
    pub fn init() -> Rc<Self>
    {

        //Check if StateContainers  has already been intialised.

        if let Some(state_containers) = try_get_state_containers()
        {

            return state_containers;

        }

        let sc = Rc::new_cyclic(|weak_self|
        {

            let isc = InternalNonCollectionStateContainers::new(weak_self);

            Self
            {

                nc_internals: RefCell::new(isc),
                widget_state: RefCell::new(WidgetStateContainers::new(weak_self)),

                //Delays removal of widget state so that it can be used in all connect_destroy signal handlers. 

                widget_state_removal_timeout: SimpleTimeOut::with_fn(Duration::new(1, 0), |_rc_self|
                {

                    let sc = StateContainers::get();

                    {

                        let mut nc_internals_mut = sc.nc_internals.borrow_mut();

                        for state in nc_internals_mut.widget_states_to_remove.drain()
                        {
    
                            sc.remove_by_rc_by_ptr(&state);
    
                        }

                    }

                    false

                })

            }

            /*
            Self
            {

                application_state: NonOption::invalid(),
                widget_state: HashMap::new(),
                weak_self: weak_self.clone()

            }
            */

        });

        set_state_containers(&sc);

        sc

    }

    ///
    /// Get the StateContainers singleton.
    /// 
    /// WARNING: will panic if it hasn't been initalised.
    /// 
    pub fn get() -> Rc<StateContainers>
    {

        get_state_containers()

    }

    ///
    /// Try to get the StateContainers singleton.
    /// 
    pub fn try_get() -> Option<Rc<StateContainers>>
    {

        try_get_state_containers()

    }

    pub fn my_weak_self(&self) -> Weak<StateContainers>
    {

        self.nc_internals.borrow().weak_self.clone()

    }

    pub fn set_application_state(&mut self, state: &Rc<dyn ApplicationStateContainer>)
    {

        self.nc_internals.borrow_mut().application_state.set(state.clone());

    }

    pub fn application_state(&self) -> Rc<dyn ApplicationStateContainer>
    {

        self.nc_internals.borrow().application_state.get_ref().clone()

    }

    pub fn has_application_state(&self) -> bool
    {

        self.nc_internals.borrow().application_state.is_valid()

    }

    pub fn add(&self, sc: &Rc<dyn WidgetStateContainer>)
    {

        self.widget_state.borrow_mut().add(sc);
        
    }

    /* 
        no rules expected the token `&`
        no rules expected this token in macro callrustcClick for full compiler diagnostic
        getters_setters_callers.rs(855, 56): while trying to match `param_type`
        No quick fixes available
    */

    //impl_rfc_borrow_call!(widget_state, add, sc: &Rc<dyn WidgetStateContainer>);

    /*
    pub fn remove(&self, sc: &Rc<dyn WidgetStateContainer>) -> bool
    {

        self.widget_state.borrow_mut().remove(sc)
        
    }
    */

    pub fn delyed_removal(&self, sc: &Rc<dyn WidgetStateContainer>) -> bool
    {

        if self.nc_internals.borrow_mut().widget_states_to_remove.insert(RcByPtr::new(sc))
        {

            //Make sure the timer has started.

            self.widget_state_removal_timeout.start()

        }
        else
        {
            
            false

        }

    }

    pub fn remove_by_rc_by_ptr(&self, rbp_sc: &RcByPtr<dyn WidgetStateContainer>) -> bool
    {

        self.widget_state.borrow_mut().remove_by_rc_by_ptr(rbp_sc)

    }

    impl_rfc_borrow_and_mut!(widget_state, WidgetStateContainers);

    pub fn has_widget_state<T: WidgetExt + Eq + ObjectExt + Clone>(&self, widget: &T) -> bool
    {

        let lwa = LookupWidgetAdapter::new(widget);

        {

            let ws = self.widget_state.borrow();

            ws.dyn_has_state(&lwa)

        }

    }

    pub fn find_widget_state<T: WidgetExt + Eq + ObjectExt + Clone>(&self, widget: &T) -> Option<Rc<dyn WidgetStateContainer>>
    {

        let lwa = LookupWidgetAdapter::new(widget);

        {

            let ws = self.widget_state.borrow();

            ws.dyn_find_state(&lwa)

        }

    }

}

