use std::collections::{HashMap, HashSet};

use std::any::{Any, TypeId};

use std::rc::{Rc, Weak};

use corlib::convert::AsAnyRef;
use gtk::glib::object::IsA;

use corlib::{rc::RcByPtr, cell::RefCellStore};

use gtk::glib::object::ObjectExt;

use gtk::prelude::WidgetExt;

use gtk::Widget;

use gtk::glib::Type;

use crate::rc_conversions::{to_rc_dyn_strong_wsc, to_rc_dyn_wsc};
use crate::{StateContainers, StrongWidgetAdapter, StrongWidgetObject}; //, DynStrongWidgetStateContainer}; //LookupWidgetObject,

use gtk::glib;

use gtk::glib::clone;

use gtk::glib::object::Cast;

//use gtk4::glib::glib_macros::clone;

///
/// Indicates that the implementing object contains a StrongWidgetAdapter object and makes it accessible.
/// 
pub trait StrongWidgetStateContainer<T, P>
    where T: WidgetExt + ObjectExt + Eq + Clone,
          P: DynStrongWidgetStateContainer
{

    fn widget_adapter(&self) -> Rc<StrongWidgetAdapter<T, P>>;

    fn widget_adapter_ref(&self) -> &StrongWidgetAdapter<T, P>;

}

///
/// Like the StrongWidgetStateContainer trait, but dynamic.
/// 
pub trait DynStrongWidgetStateContainer : AsAnyRef //+ Debug
{

    //fn adapted_widget(&self) -> &(dyn StoredWidgetObject); //'a  //Any + WidgetExt

    //fn dyn_adapted_widget(&self) -> &(dyn Any);

    //fn dyn_adapted_widget(&self) -> Rc<dyn StoredWidgetObject>;

    fn dyn_widget_adapter(&self) -> Rc<dyn StrongWidgetObject>;

    fn dyn_widget_adapter_ref(&self) -> &dyn StrongWidgetObject;

}

///
/// Use to setup a state container struct with a strongly referenced widget object.
/// 
/// Note: both rules require the implementing struct to have a widget_adapter field, however in the first rule this field is expected to be named "widget_adapter".
/// 
#[cfg(feature = "strong_widget_state")]
#[macro_export]
macro_rules! impl_strong_widget_state_container_traits
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

        impl AsAnyRef for $widget_state_container_type
        {

            fn as_any_ref(&self) -> &dyn Any
            {

                self
                
            }

        }

        impl DynStrongWidgetStateContainer for $widget_state_container_type
        {

            fn dyn_widget_adapter(&self) -> Rc<dyn StrongWidgetObject>
            {

                self.widget_adapter.clone()

            }

            fn dyn_widget_adapter_ref(&self) -> &dyn StrongWidgetObject
            {

                self.widget_adapter.as_ref()

            }

        }

        impl StrongWidgetStateContainer<$widget_type, $widget_state_container_type> for $widget_state_container_type
        {

            fn widget_adapter(&self) -> Rc<StrongWidgetAdapter<$widget_type, $widget_state_container_type>>
            {

                self.widget_adapter.clone()

            }

            fn widget_adapter_ref(&self) -> &StrongWidgetAdapter<$widget_type, $widget_state_container_type>
            {

                self.widget_adapter.as_ref()

            }

        }

        impl WeakSelf for $widget_state_container_type
        {

            fn weak_self(&self) -> Weak<Self>
            {

                self.widget_adapter.weak_parent()
                
            }

            fn weak_self_ref(&self) -> &Weak<Self>
            {

                self.widget_adapter.weak_parent_ref()
                
            }

        }

    };
    ($widget_type:ty, $widget_state_container_type:ty, $widget_adapter:ident) =>
    {

        impl AsAnyRef for $widget_state_container_type
        {

            fn as_any_ref(&self) -> &dyn Any
            {

                self
                
            }

        }

        impl DynStrongWidgetStateContainer for $widget_state_container_type
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

        impl StrongWidgetStateContainer<$widget_type, $widget_state_container_type> for $widget_state_container_type
        {

            fn widget_adapter(&self) -> Rc<StrongWidgetAdapter<$widget_type, $widget_state_container_type>>
            {

                self.$widget_adapter.clone()

            }

            fn widget_adapter_ref(&self) -> &StrongWidgetAdapter<$widget_type, $widget_state_container_type>
            {

                self.$widget_adapter.as_ref()

            }

        }

        impl WeakSelf for $widget_state_container_type
        {

            fn weak_self(&self) -> Weak<Self>
            {

                self.$widget_adapter.weak_parent()
                
            }

            fn weak_self_ref(&self) -> &Weak<Self>
            {

                self.$widget_adapter.weak_parent_ref()
                
            }

        }

    };

}

/*
cfg_if!
{

    if #[cfg(feature = "strong_widget_state")]
    {


    }

}
*/

///
/// Keeps track of Rc'd DynStrongWidgetStateContainer implementers.
/// 
pub struct StrongWidgetStateContainers
{

    widget_state: RefCellStore<HashMap<Type, HashSet<RcByPtr<dyn DynStrongWidgetStateContainer>>>>,
    //weak_parent: Weak<StateContainers>

}

impl StrongWidgetStateContainers
{

    pub fn new() -> Self //weak_parent: &Weak<StateContainers>) -> Self
    {

        Self
        {

            widget_state: RefCellStore::new(HashMap::new()),
            //weak_parent: weak_parent.clone()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self //weak_parent: &Weak<StateContainers>, capacity: usize) -> Self
    {

        Self
        {

            widget_state: RefCellStore::new(HashMap::with_capacity(capacity)),
            //weak_parent: weak_parent.clone()

        }

    }

    pub fn capacity(&self) -> usize
    {

        self.widget_state.refcell_ref().borrow().capacity()

    }

    //Renamed  from add

    pub fn dyn_add(&self, sc: &Rc<dyn DynStrongWidgetStateContainer>) -> bool
    {

        //let wt_id = sc.widget().type_id();

        let glt = sc.dyn_widget_adapter_ref().glib_type();

        let rbp_sc = RcByPtr::new(sc); //: RcByPtr<dyn WidgetStateContainer>

        /*
            the method `clone` exists for struct `RcByPtr<dyn WidgetStateContainer>`, but its trait bounds were not satisfied
            the following trait bounds were not satisfied:
            `dyn state_containers::WidgetStateContainer: Clone`
            which is required by `RcByPtr<dyn state_containers::WidgetStateContainer>: Clone`rustcClick for full compiler diagnostic
            state_containers.rs(47, 1): doesn't satisfy `dyn state_containers::WidgetStateContainer: Clone`
            rc_by_ptr.rs(9, 1): doesn't satisfy `_: Clone`

         */

        self.widget_state.borrow_mut_with_param( rbp_sc, |mut state, rbp_sc|
        {

            if let Some(wsc_set) = state.get_mut(&glt)
            {
    
                //let rbp_sc_2: RcByPtr<dyn DynStrongWidgetStateContainer> = rbp_sc.clone();
    
                if wsc_set.insert(rbp_sc)
                {
    
                    return true;
                    
                }
    
            }
            else
            {
                
                let mut hs = HashSet::new();
    
                hs.insert(rbp_sc);
    
                state.insert(glt, hs);
    
                return true;
    
            }
    
            false

        })

    }

    pub fn add<WSC>(&self, sc: &Rc<WSC>)
        where WSC: DynStrongWidgetStateContainer + 'static
    {

        let wsc = to_rc_dyn_strong_wsc(sc); //sc.clone());

        self.dyn_add(&wsc);
        
    }
    
    //Disabled

    /*
    fn on_destroy(&self, rbp_sc: &RcByPtr<dyn DynWidgetStateContainer>)
    {

        let wbp_sc = rbp_sc.downgrade();

        //let rc_sc = rbp_sc.contents().clone();

        //let weak_sc = Rc::downgrade(rbp_sc.contents());

        let widget = rbp_sc.contents().dyn_widget_adapter_ref().widget();

        //let weak_rc_sc = Rc::downgrade(&rbp_sc.contents());

        let weak_parent = self.weak_parent.clone();

        //clone!( #[weak] rc_sc,

        widget.connect_destroy(move |_widget|
        {

            //Upgrade the current state container.

            if let Some(parent) = weak_parent.upgrade()
            {

                if let Some(rbp_sc) = wbp_sc.upgrade()
                {

                    //let wsc: Rc<dyn DynWidgetStateContainer> = to_rc_dyn_wsc(rc_sc); //to_wsc_super(rc_parent); //&rc_parent;

                    parent.remove_by_rc_by_ptr(&rbp_sc); //&RcByPtr::new(&rc_sc)); //&wsc));

                }

            }

        });

        //Make sure the added state gets removed when its widget gets destroyed.

        //rbp_sc.contents().dyn_widget_adapter_ref().connect_destroy(self.weak_parent.clone());

    }
    */

    pub fn remove(&self, sc: &Rc<dyn DynStrongWidgetStateContainer>) -> bool
    {

        let rbp_sc = RcByPtr::new(sc);

        let glt = rbp_sc.contents_ref().dyn_widget_adapter_ref().glib_type();

        self.widget_state.borrow_mut_with_param(glt, |mut state, glt|
        {

            if let Some(wsc_set) = state.get_mut(&glt)
            {
    
                return wsc_set.remove(&rbp_sc);
    
            }

            false

        })

    }

    pub fn remove_by_rc_by_ptr(&self, rbp_sc: &RcByPtr<dyn DynStrongWidgetStateContainer>) -> bool
    {

        let glt = rbp_sc.contents_ref().dyn_widget_adapter_ref().glib_type(); //.type_id();

        self.widget_state.borrow_mut_with_param(glt, |mut state, glt|
        {

            if let Some(wsc_set) = state.get_mut(&glt)
            {
    
                return wsc_set.remove(rbp_sc);
    
            }
    
            false

        })

    }

    pub fn remove_by_widget_ref<W>(&self, widget: &W) -> bool
        where W: WidgetExt + ObjectExt + Eq
    {

        self.widget_state.borrow_mut_with_param(widget, |mut state, widget|
        {

            let glib_type = widget.type_();

            if let Some(wsc_set) = state.get_mut(&glib_type)
            {
    
                let mut found_wsc = None;
    
                for item in wsc_set.iter()
                {
    
                    if item.contents_ref().dyn_widget_adapter_ref().widget_ref() == widget
                    {
    
                        found_wsc = Some(item.clone());
    
                        break;
    
                    }
    
                }
    
                if let Some(wsc) = found_wsc
                {
    
                    return wsc_set.remove(&wsc);
    
                }
    
            }
    
            false

        })

    }

    pub fn contains(&self, sc: &Rc<dyn DynStrongWidgetStateContainer>) -> bool
    {

        self.widget_state.borrow_with_param(sc, |state, sc|
        {

            let rbp_sc = RcByPtr::new(sc);

            let glt = rbp_sc.contents_ref().dyn_widget_adapter_ref().glib_type();

            if let Some(wsc_set) = state.get(&glt)
            {

                return wsc_set.contains(&rbp_sc);

            }

            false

        })

    }

    pub fn contains_widget_type(&self, wo: &(dyn StrongWidgetObject)) -> bool
    {

        let glt = wo.glib_type(); //.type_id();

        self.widget_state.refcell_ref().borrow().contains_key(&glt) //wt_id)

    }

    pub fn contains_widget_type_in(&self, sc: &Rc<dyn DynStrongWidgetStateContainer>) -> bool
    {

        let glt = sc.dyn_widget_adapter_ref().glib_type(); //.type_id();

        self.widget_state.refcell_ref().borrow().contains_key(&glt) //wt_id)

    }

    pub fn count_of_types(&self) -> usize
    {

        self.widget_state.refcell_ref().borrow().len()

    }

    pub fn total_count_of_bucket_lens(&self) -> u128
    {

        let mut total: u128 = 0;

        for bucket in self.widget_state.refcell_ref().borrow().iter()
        {

            total += bucket.1.len() as u128;

        }

        total

    }

    pub fn individual_counts_of_bucket_lens(&self) -> HashMap<Type, usize>
    {

        self.widget_state.borrow(|state|
        {

            let mut lens = HashMap::with_capacity(state.len());

            for bucket in state.iter()
            {

                lens.insert(bucket.0.clone(), bucket.1.len());

            }

            lens

        })

    }

    pub fn dyn_find_state(&self, widget: &dyn StrongWidgetObject) -> Option<Rc<dyn DynStrongWidgetStateContainer>>
    {

        self.widget_state.borrow_with_param(widget, |state, widget|
        {

            let glt = widget.glib_type(); //.type_(); //.type_id();

            //Lookup ther bucket

            if let Some(wsc_set) = state.get(&glt) //wt_id)
            {

                //Iterate through the values trying find the state with the widget

                for ws in wsc_set.iter()
                {

                    let contents = ws.contents_ref();

                    if contents.dyn_widget_adapter_ref().dyn_has(widget.dyn_widget())
                    {

                        return Some(contents.clone());

                    }

                }

            }

            None

        })

    }

        ///
    /// Try find the widget state based on the widget instance.
    ///
    pub fn find_widget_state<T: WidgetExt + Eq + ObjectExt + Clone + IsA<T>>(&self, widget: &T) -> Option<Rc<dyn DynStrongWidgetStateContainer>> // + MayDowncastTo<Widget>
    {

        self.widget_state.borrow_with_param( widget, |state, widget|
        {

            let glt = widget.type_();

            let widget_widget = widget.upcast_ref::<Widget>();

            //Lookup ther bucket

            if let Some(wsc_set) = state.get(&glt)
            {

                //Iterate through the values trying find the state with the widget

                for ws in wsc_set.iter()
                {

                    let contents = ws.contents_ref();

                    if contents.dyn_widget_adapter_ref().has(widget_widget)
                    {

                        return Some(contents.clone());

                    }

                }

            }

            None
    
        })

    }

    pub fn dyn_has_state(&self, widget: &dyn StrongWidgetObject) -> bool
    {

        self.widget_state.borrow_with_param(widget, |state, widget|
        {
        
            let glt = widget.glib_type(); //type_(); //.type_id();

            //Lookup ther bucket

            if let Some(wsc_set) = state.get(&glt)
            {

                //Iterate through the values trying find the state with the widget

                for ws in wsc_set.iter()
                {

                    let contents = ws.contents_ref();

                    if contents.dyn_widget_adapter_ref().dyn_has(widget.dyn_widget())
                    {

                        return true;

                    }

                }

            }

            false

        })

    }

    pub fn buckets_len(&self) -> usize
    {

        self.widget_state.refcell_ref().borrow().len()

    }

    pub fn buckets_capacity(&self) -> usize
    {

        self.widget_state.refcell_ref().borrow().capacity()

    }

    pub fn bucket_len(&self, type_of_bucket: &Type) -> Option<usize>
    {

        if let Some(bucket) = self.widget_state.refcell_ref().borrow().get(type_of_bucket)
        {

            Some(bucket.len())

        }
        else
        {

            None
            
        }

    }

    pub fn bucket_capacity(&self, type_of_bucket: &Type) -> Option<usize>
    {

        if let Some(bucket) = self.widget_state.refcell_ref().borrow().get(type_of_bucket)
        {

            Some(bucket.capacity())

        }
        else
        {

            None
            
        }

    }

    pub fn clear(&self)
    {

        self.widget_state.refcell_ref().borrow_mut().clear();

    }

}

