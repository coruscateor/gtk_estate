use std::any::TypeId;

use std::time::Duration;

use std::{rc::*, any::Any};

use std::cell::{Ref, RefCell, RefMut, UnsafeCell};

use corlib::{get_some, impl_rfc_borrow_2, impl_rfc_borrow_and_mut_2, impl_rfc_borrow_call, impl_rfc_borrow_mut_2, impl_rfc_borrow_mut_call, AsAny};

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use gtk::Widget;

use paste::paste;

use std::collections::{HashMap, HashSet};

use corlib::
{
    NonOption,
    RcByPtr,
    collections::UniqueItemList,
    impl_rfc_borrow,
    impl_rfc_borrow_mut,
    //impl_rfc_borrow_and_mut

};

use gtk4 as gtk;

use gtk::glib::object::{IsA, ObjectExt}; //MayDowncastTo, 

use crate::rc_conversions::to_rc_dyn_wsc;

use crate::{adapters::*, RcSimpleTimeOut, WidgetStateContainers, SimpleTimeOut};

use cfg_if::cfg_if;

/*
object.rs(27, 7): for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
only auto traits can be used as additional traits in a trait object
consider creating a new trait with all of these as supertraits and using that trait here instead: `trait NewTrait: std::any::Any + ApplicationExt {}`
*/

///
/// Indicates that the implementing object stores application related data.
/// 
pub trait DynApplicationStateContainer //: AsAny + Any //<'a>
{

    //fn application() -> &'a (dyn Any + ApplicationExt);

    //fn adapted_application(&self) -> &(dyn StoredApplicationObject); //'a //Any +

    //fn dyn_adapted_application(&self) -> &(dyn Any);

    //fn dyn_adapted_application(&self) -> Rc<dyn StoredApplicationObject>;

    fn dyn_application_adapter(&self) -> Rc<dyn StoredApplicationObject>;

    fn dyn_application_adapter_ref(&self) -> &dyn StoredApplicationObject;

}

pub trait ApplicationStateContainer<T, P>
    where T: ApplicationExt + Eq + ObjectExt + Clone,
          P: DynApplicationStateContainer
{

    fn application_adapter(&self) -> Rc<ApplicationAdapter<T, P>>;

    fn application_adapter_ref(&self) -> &ApplicationAdapter<T, P>;

}


//gtk_estate::

#[macro_export]
macro_rules! impl_application_state_container_traits
{

    () =>
    {

        impl DynApplicationStateContainer for ApplicationState
        {

            fn dyn_application_adapter(&self) -> Rc<dyn StoredApplicationObject>
            {

                self.application_adapter.clone()

            }

            fn dyn_application_adapter_ref(&self) -> &dyn StoredApplicationObject
            {

                self.application_adapter.as_ref()

            }

        }
        
        impl ApplicationStateContainer<Application, ApplicationState> for ApplicationState
        {

            fn application_adapter(&self) -> Rc<ApplicationAdapter<Application, ApplicationState>>
            {

                self.application_adapter.clone()

            }

            fn application_adapter_ref(&self) -> &ApplicationAdapter<Application, ApplicationState>
            {

                self.application_adapter.as_ref()

            }

        }

    };
    ($application_type:ty, $application_state_type:ty) =>
    {

        impl DynApplicationStateContainer for $application_state_type
        {

            fn dyn_application_adapter(&self) -> Rc<dyn StoredApplicationObject>
            {

                self.application_adapter.clone()

            }

            fn dyn_application_adapter_ref(&self) -> &dyn StoredApplicationObject
            {

                self.application_adapter.as_ref()

            }

        }

        impl ApplicationStateContainer<$application_type, $application_state_type> for $application_state_type
        {

            fn application_adapter(&self) -> Rc<ApplicationAdapter<$application_type, $application_state_type>>
            {

                self.application_adapter.clone()

            }

            fn application_adapter_ref(&self) -> &ApplicationAdapter<$application_type, $application_state_type>
            {

                self.application_adapter.as_ref()

            }

        }

    };
    ($application_type:ty, $application_state_type:ty, $application_adapter:ident) =>
    {

        impl DynApplicationStateContainer for $application_state_type
        {

            fn dyn_application_adapter(&self) -> Rc<dyn StoredApplicationObject>
            {

                self.$application_adapter.clone()

            }

            fn dyn_application_adapter_ref(&self) -> &dyn StoredApplicationObject
            {

                self.$application_adapter.as_ref()

            }

        }

        impl ApplicationStateContainer<$application_type, $application_state_type> for $application_state_type
        {

            fn application_adapter(&self) -> Rc<ApplicationAdapter<$application_type, $application_state_type>>
            {

                self.$application_adapter.clone()

            }

            fn application_adapter_ref(&self) -> &ApplicationAdapter<$application_type, $application_state_type>
            {

                self.$application_adapter.as_ref()

            }

        }

    };
    /*
    ($application_state_container_type:ty) =>
    {

        impl ApplicationStateContainer for $application_state_container_type
        {

            fn dyn_adapter(&self) -> Rc<dyn StoredApplicationObject>
            {

                self.application_adapter.clone()

            }

            fn dyn_adapter_ref(&self) -> &dyn StoredApplicationObject
            {

                self.application_adapter.as_ref()

            }

        }

    };
    */

}

///
/// Indicates that the implementing object stores widget related data.
/// 
pub trait DynWidgetStateContainer //: AsAny + Any //<'a>
{

    //fn adapted_widget(&self) -> &(dyn StoredWidgetObject); //'a  //Any + WidgetExt

    //fn dyn_adapted_widget(&self) -> &(dyn Any);

    //fn dyn_adapted_widget(&self) -> Rc<dyn StoredWidgetObject>;

    fn dyn_widget_adapter(&self) -> Rc<dyn StoredWidgetObject>;

    fn dyn_widget_adapter_ref(&self) -> &dyn StoredWidgetObject;

}

pub trait WidgetStateContainer<T, P>
    where T: Eq + ObjectExt + Clone,
          P: DynWidgetStateContainer
{

    fn widget_adapter(&self) -> Rc<WidgetAdapter<T, P>>;

    fn widget_adapter_ref(&self) -> &WidgetAdapter<T, P>;

}

#[macro_export]
macro_rules! impl_widget_state_container_traits
{

    /*
    ($widget_state_container_type:ty) =>
    {

        impl WidgetStateContainer for $widget_state_container_type
        {

            fn dyn_adapter(&self) -> Rc<dyn StoredWidgetObject>
            {

                self.widget_adapter.clone()

            }

            fn dyn_adapter_ref(&self) -> &dyn StoredWidgetObject
            {

                self.widget_adapter.as_ref()

            }

        }

    };
    */
    ($widget_type:ty, $widget_state_container_type:ty) =>
    {

        impl DynWidgetStateContainer for $widget_state_container_type
        {

            fn dyn_widget_adapter(&self) -> Rc<dyn StoredWidgetObject>
            {

                self.widget_adapter.clone()

            }

            fn dyn_widget_adapter_ref(&self) -> &dyn StoredWidgetObject
            {

                self.widget_adapter.as_ref()

            }

        }

        impl WidgetStateContainer<$widget_type, $widget_state_container_type> for $widget_state_container_type
        {

            fn widget_adapter(&self) -> Rc<WidgetAdapter<$widget_type, $widget_state_container_type>>
            {

                self.widget_adapter.clone()

            }

            fn widget_adapter_ref(&self) -> &WidgetAdapter<$widget_type, $widget_state_container_type>
            {

                self.widget_adapter.as_ref()

            }

        }

    };
    ($widget_type:ty, $widget_state_container_type:ty, $widget_adapter:ident) =>
    {

        impl DynWidgetStateContainer for $widget_state_container_type
        {

            fn dyn_widget_adapter(&self) -> Rc<dyn StoredWidgetObject>
            {

                self.$widget_adapter.clone()

            }

            fn dyn_widget_adapter_ref(&self) -> &dyn StoredWidgetObject
            {

                self.$widget_adapter.as_ref()

            }

        }

        impl WidgetStateContainer<$widget_type, $widget_state_container_type> for $widget_state_container_type
        {

            fn widget_adapter(&self) -> Rc<WidgetAdapter<$widget_type, $widget_state_container_type>>
            {

                self.$widget_adapter.clone()

            }

            fn widget_adapter_ref(&self) -> &WidgetAdapter<$widget_type, $widget_state_container_type>
            {

                self.$widget_adapter.as_ref()

            }

        }

    };

}

//The StateContainers sigleton static location.

//static mut STATE_CONTAINERS: NonOption<Rc<StateContainers>> = NonOption::invalid(); 

cfg_if!
{

    if #[cfg(feature = "thread_local_state")]
    {

        thread_local!
        {

            static STATE_CONTAINERS: UnsafeCell<Option<Rc<StateContainers>>> = UnsafeCell::new(None); 

        }

        ///
        /// Clone a copy of the StateContainers state.
        /// 
        fn get_state_containers() -> Rc<StateContainers>
        {

            //check is correct thread?

            STATE_CONTAINERS.with(|containers|
            {

                let mut opt_ref = unsafe { containers.get().as_ref() };

                if let Some(res) = get_some!(opt_ref)
                {

                    res.clone()

                }
                else
                {

                    StateContainers::init()

                }

            })


        }

        ///
        /// Try to clone a copy of the StateContainers state.
        /// 
        fn try_get_state_containers() -> Option<Rc<StateContainers>>
        {

            //check is correct thread?

            STATE_CONTAINERS.with(|containers|
            {

                //let opt = unsafe { containers.get().as_ref() }; //STATE_CONTAINERS.as_ref(); //.try_get_ref();

                let mut opt_ref = unsafe { containers.get().as_ref() };

                get_some!(opt_ref).clone()

                /*
                if let Some(sc) = get_some!(opt_ref)
                {

                    return Some(sc.clone());

                }
                else
                {

                    None
                    
                }
                */

            })

        }

        ///
        /// Set the StateContainers state if it is invalid.
        /// 
        fn set_state_containers(state_containers: &Rc<StateContainers>) -> bool
        {

            STATE_CONTAINERS.with(|containers|
            {

                let mut opt_mut = unsafe { containers.get().as_mut() };

                let mut_ref = get_some!(opt_mut);

                if mut_ref.is_none()
                {

                    *mut_ref = Some(state_containers.clone());

                    true

                }
                else
                {

                    false
                    
                }

            })

            //Check if the current thread is correct?

            /*
            unsafe
            {

                if !STATE_CONTAINERS.is_some() //.is_valid()
                {

                    STATE_CONTAINERS = Some(state_containers.clone()); //.set(state_containers.clone())

                }

            }
            */

        }

        fn state_containers_is_set() -> bool
        {

            STATE_CONTAINERS.with(|containers|
            {

                let mut opt_mut = unsafe { containers.get().as_ref() };

                let mut_ref = get_some!(opt_mut);

                mut_ref.is_some()

            })

        }

    }

}

//StateContainers internal, externally mutable state.

struct InternalNonCollectionStateContainers
{

    pub application_state: NonOption<Rc<dyn DynApplicationStateContainer>>,
    //widget_state: HashMap<TypeId, HashSet<RcByPtr<Rc<dyn WidgetStateContainer>>>>,
    pub weak_self: Weak<StateContainers>, //Self is a Reference Type!
    pub widget_states_to_remove: HashSet<RcByPtr<dyn DynWidgetStateContainer>> //Vec<Rc<dyn WidgetStateContainer>>

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

///
/// The struct within which all widget states are centrally located.
/// 
pub struct StateContainers
{

    nc_internals: RefCell<InternalNonCollectionStateContainers>,
    widget_state: RefCell<WidgetStateContainers>,
    widget_state_removal_timeout: RcSimpleTimeOut,

}

impl StateContainers
{

    ///
    /// Called once to Initialise the StateContainers.
    /// 
    /// This is done automatically when you call the "get" static method.
    /// 
    pub fn init() -> Rc<Self>
    {

        //Check if StateContainers  has already been intialised.

        #[cfg(feature = "thread_local_state")]
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

        });

        #[cfg(feature = "thread_local_state")]
        set_state_containers(&sc);

        sc

    }

    cfg_if!
    {

        if #[cfg(feature = "thread_local_state")]
        {

            ///
            /// Get the StateContainers singleton.
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

        }

    }

    ///
    /// Get the "weak self" of the StateContainers.
    /// 
    pub fn weak_self(&self) -> Weak<StateContainers>
    {

        self.nc_internals.borrow().weak_self.clone()

    }

    ///
    /// Set the application state. Returns false if a DynApplicationStateContainer is already present.
    /// 
    pub fn try_set_application_state<T>(&self, state: &Rc<T>) -> bool //&Rc<dyn ApplicationStateContainer>) -> bool
        where T: DynApplicationStateContainer + 'static
    {

        {

            let mut nc_internals_mut = self.nc_internals.borrow_mut();
            
            if nc_internals_mut.application_state.is_valid()
            {

                return false;

            }

            nc_internals_mut.application_state.set(state.clone());

        }

        true

    }

    ///
    /// Set the application state or panic.
    /// 
    pub fn set_application_state<T>(&self, state:&Rc<T>) //&Rc<dyn ApplicationStateContainer>)
        where T: DynApplicationStateContainer + 'static
    {

        if !self.try_set_application_state(state)
        {

            panic!("GTK Estate - Error: Cound not set applicaton state!")

        }

    }

    ///
    /// Get the application state or panic.
    /// 
    pub fn dyn_application_state(&self) -> Rc<dyn DynApplicationStateContainer>
    {

        self.nc_internals.borrow().application_state.get_ref().clone()

    }

    ///
    /// Try and get the application state or panic.
    /// 
    pub fn try_get_application_state(&self) -> Option<Rc<dyn DynApplicationStateContainer>>
    {

        match self.nc_internals.borrow().application_state.try_get_ref()
        {

            Some(val) =>
            {

                Some(val.clone())

            }
            None => None

        }

    }

    ///
    /// Check if the application state exists or not.
    /// 
    pub fn has_application_state(&self) -> bool
    {

        self.nc_internals.borrow().application_state.is_valid()

    }

    ///
    /// Add a Rc<dyn WidgetStateContainer> to the widgets states.
    /// 
    pub fn dyn_add(&self, sc: &Rc<dyn DynWidgetStateContainer>)
    {

        self.widget_state.borrow_mut().add(sc);
        
    }

    ///
    /// Add a Rc<WSC: WidgetStateContainer> to the widgets states.
    /// 
    pub fn add<WSC>(&self, sc: &Rc<WSC>)
        where WSC: DynWidgetStateContainer + 'static
    {

        //let any_sc: &dyn Any = sc;

        //let wsc = any_sc.downcast_ref::<Rc<dyn WidgetStateContainer>>().expect("Error: No Rc<dyn WidgetStateContainer>");

        let wsc = to_rc_dyn_wsc(sc.clone());

        self.widget_state.borrow_mut().add(&wsc);
        
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

    ///
    /// Remove a widget - delayed by a short period.
    /// 
    pub fn delayed_removal(&self, sc: &Rc<dyn DynWidgetStateContainer>) -> bool
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

    ///
    /// Remove a widget - via an RcByPtr.
    /// 
    pub fn remove_by_rc_by_ptr(&self, rbp_sc: &RcByPtr<dyn DynWidgetStateContainer>) -> bool
    {

        self.widget_state.borrow_mut().remove_by_rc_by_ptr(rbp_sc)

    }

    impl_rfc_borrow_and_mut_2!(widget_state, WidgetStateContainers);

    ///
    /// Does the widget state exist?
    /// 
    pub fn has_widget_state<T: WidgetExt + Eq + ObjectExt + Clone + IsA<T>>(&self, widget: &T) -> bool // + MayDowncastTo<Widget>
    {

        let lwa = LookUpWidgetAdapter::new(widget);

        {

            let ws = self.widget_state.borrow();

            ws.dyn_has_state(&lwa)

        }

    }

    ///
    /// Try find the widget state based on the widget instance.
    /// 
    pub fn find_widget_state<T: WidgetExt + Eq + ObjectExt + Clone + IsA<T>>(&self, widget: &T) -> Option<Rc<dyn DynWidgetStateContainer>> // + MayDowncastTo<Widget>
    {

        let lwa = LookUpWidgetAdapter::new(widget);

        {

            let ws = self.widget_state.borrow();

            ws.dyn_find_state(&lwa)

        }

    }

}

/*
macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
`#[deny(macro_expanded_macro_exports_accessed_by_absolute_paths)]` on by defaultrustcClick for full compiler diagnostic
state_containers.rs(917, 9): the macro is defined here
gtk_estate::state_containers
macro_rules! scs_add
This macro gets a StateContainers Rc instance and adds the "$this" widget state to it
 */

 /*
cfg_if!
{

    if #[cfg(feature = "thread_local_state")]
    {


        ///
        /// This macro gets a StateContainers Rc instance and adds the "$this" widget state to it  
        /// 
        #[macro_export]
        macro_rules! scs_add
        {

            ($this:ident) =>
            {

                let scs = StateContainers::get();

                scs.add(&$this);

            }

        }

    }

}
*/

///
/// This macro gets a StateContainers Rc instance and calls "set_application_state" on it, passing "$this", to set the application state.
/// 
#[cfg(feature = "thread_local_state")]
#[macro_export]
macro_rules! scs_set_application_state
{

    ($this:ident) =>
    {

        let scs = StateContainers::get();

        scs.set_application_state(&$this);

    }

}

///
/// This macro gets a StateContainers Rc instance and adds the "$this" widget state to it  
///
#[cfg(feature = "thread_local_state")]
#[macro_export]
macro_rules! scs_add
{

    ($this:ident) =>
    {

        let scs = StateContainers::get();

        scs.add(&$this);

    }

}
