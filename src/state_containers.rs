use std::any::TypeId;

use std::{rc::*, any::Any};

use std::cell::{RefCell, Ref, RefMut};

use corlib::impl_rfc_borrow_call;
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

use crate::{adapters::*, WidgetStateContainers};

/*
object.rs(27, 7): for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
only auto traits can be used as additional traits in a trait object
consider creating a new trait with all of these as supertraits and using that trait here instead: `trait NewTrait: std::any::Any + ApplicationExt {}`
*/

pub trait ApplicationStateContainer : Any //<'a>
{

    //fn application() -> &'a (dyn Any + ApplicationExt);

    fn application(&self) -> &(dyn ApplicationObject); //'a //Any +

}

pub trait WidgetStateContainer : Any //<'a>
{

    fn widget(&self) -> &(dyn WidgetObject); //'a  //Any + WidgetExt

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

    application_state: NonOption<Rc<dyn ApplicationStateContainer>>,
    //widget_state: HashMap<TypeId, HashSet<RcByPtr<Rc<dyn WidgetStateContainer>>>>,
    weak_self: Weak<StateContainers> //Self is a Reference Type!

}

impl InternalNonCollectionStateContainers
{

    pub fn new(weak_self: &Weak<StateContainers>) -> Self
    {

        Self
        {

            application_state: NonOption::invalid(),
            //widget_state: HashMap::new(),
            weak_self: weak_self.clone()

        }

    }

    pub fn set_application_state(&mut self, state: &Rc<dyn ApplicationStateContainer>)
    {

        self.application_state.set(state.clone());

    }

    pub fn application_state(&self) -> &Rc<dyn ApplicationStateContainer>
    {

        self.application_state.get_ref()

    }

    pub fn has_application_state(&self) -> bool
    {

        self.application_state.is_valid()

    }

}

pub struct StateContainers //<'a>
{

    nc_internals: RefCell<InternalNonCollectionStateContainers>,
    widget_state: RefCell<WidgetStateContainers> //RefCell<HashMap<TypeId, HashSet<RcByPtr<Rc<dyn WidgetStateContainer>>>>>

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

        let sc = Rc::new_cyclic(|weak_self|
        {

            let isc = InternalNonCollectionStateContainers::new(weak_self);

            Self
            {

                nc_internals: RefCell::new(isc),
                widget_state: RefCell::new(WidgetStateContainers::new(weak_self)) //HashMap::new())

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

    pub fn remove(&self, sc: &Rc<dyn WidgetStateContainer>)
    {

        self.widget_state.borrow_mut().remove(sc);
        
    }

    impl_rfc_borrow_and_mut!(widget_state, WidgetStateContainers);



}

