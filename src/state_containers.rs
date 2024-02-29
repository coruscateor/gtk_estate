use std::any::TypeId;

use std::{rc::*, any::Any};

use std::cell::{RefCell, Ref, RefMut};

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

use crate::adapters::*;

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

struct InternalStateContainers
{

    application_state: NonOption<Rc<dyn ApplicationStateContainer>>,
    widget_state: HashMap<TypeId, HashSet<RcByPtr<Rc<dyn WidgetStateContainer>>>>,
    weak_self: Weak<StateContainers> //Self is a Reference Type!

}

impl InternalStateContainers
{

    pub fn new(weak_self: &Weak<StateContainers>) -> Self
    {

        Self
        {

            application_state: NonOption::invalid(),
            widget_state: HashMap::new(),
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

    internals: RefCell<InternalStateContainers>

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

            let isc = InternalStateContainers::new(weak_self);

            Self
            {

                internals: RefCell::new(isc)

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

        self.internals.borrow().weak_self.clone()

    }

    pub fn add(&self)
    {



    }

}

