use gtk::{glib::{object::{IsA, ObjectExt}, Type}, subclass::widget};

use corlib::{convert::AsAnyRef, rc::RcByPtr, cell::RefCellStore};

use gtk::{prelude::WidgetExt, Widget};

use std::{collections::{HashMap, HashSet}, fmt::Debug, rc::{Rc, Weak}};

use crate::{rc_conversions::to_rc_dyn_wsc, StateContainers, WidgetAdapter, WidgetObject, WidgetUpgradeResult}; //WidgetUpgradeErrorSide,  WidgetUpgradeLeftRightError, WidgetUpgradeLeftRightResult,

use gtk::glib::object::Cast;

pub trait WidgetStateContainer<T, P>
    where T: WidgetExt + ObjectExt + Eq + Clone,
          P: DynWidgetStateContainer
{

    fn widget_adapter(&self) -> Rc<WidgetAdapter<T, P>>;

    fn widget_adapter_ref(&self) -> &WidgetAdapter<T, P>;

}

pub trait DynWidgetStateContainer : AsAnyRef + Debug
{

    fn dyn_widget_adapter(&self) -> Rc<dyn WidgetObject>;

    fn dyn_widget_adapter_ref(&self) -> &dyn WidgetObject;

}

#[macro_export]
macro_rules! impl_widget_state_container_traits
{

    ($widget_type:ty, $widget_state_container_type:ty) =>
    {

        impl AsAnyRef for $widget_state_container_type
        {

            fn as_any_ref(&self) -> &dyn Any
            {

                self
                
            }

        }

        impl DynWidgetStateContainer for $widget_state_container_type
        {

            fn dyn_widget_adapter(&self) -> Rc<dyn WidgetObject>
            {

                self.widget_adapter.clone()

            }

            fn dyn_widget_adapter_ref(&self) -> &dyn WidgetObject
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

}

pub type RcWidgetStateContainers = Rc<WidgetStateContainers>;

pub struct WidgetStateContainers
{

    widget_state: RefCellStore<HashMap<Type, HashSet<RcByPtr<dyn DynWidgetStateContainer>>>>,
    //weak_parent: Weak<StateContainers>,
    weak_self: Weak<WidgetStateContainers>

}

impl WidgetStateContainers
{

    pub fn new() -> Rc<Self> //weak_parent: &Weak<StateContainers>) -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {

                widget_state: RefCellStore::new(HashMap::new()),
                //weak_parent: weak_parent.clone(),
                weak_self: weak_self.clone()

            }

        })

    }

    pub fn with_capacity(capacity: usize) -> Rc<Self> //weak_parent: &Weak<StateContainers>, capacity: usize) -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {
            
            Self
            {

                widget_state: RefCellStore::new(HashMap::with_capacity(capacity)),
                //weak_parent: weak_parent.clone(),
                weak_self: weak_self.clone()

            }

        })

    }

    pub fn capacity(&self) -> usize
    {

        self.widget_state.refcell_ref().borrow().capacity()

    }

    pub fn dyn_add(&self, sc: &Rc<dyn DynWidgetStateContainer>) -> WidgetUpgradeResult<bool>
    {

        let widget = sc.dyn_widget_adapter_ref().widget()?;

        //let glt = sc.dyn_widget_adapter_ref().glib_type()?;

        let rbp_sc = RcByPtr::new(sc);

        self.widget_state.borrow_mut_with_param( (widget, rbp_sc),|mut state, (widget, rbp_sc)|
        {

            let glt = widget.type_();

            if let Some(wsc_set) = state.get_mut(&glt)
            {

                let rbp_sc_2: RcByPtr<dyn DynWidgetStateContainer> = rbp_sc.clone();

                if wsc_set.insert(rbp_sc)
                {

                    self.on_destroy(widget, &rbp_sc_2);

                    return Ok(true);
                    
                }

            }
            else
            {
                
                let mut hs = HashSet::new();

                self.on_destroy(widget, &rbp_sc);

                hs.insert(rbp_sc);

                state.insert(glt, hs);

                return Ok(true);

            }

            Ok(false)

        })

    }

    pub fn add<WSC>(&self, sc: &Rc<WSC>) -> WidgetUpgradeResult<bool>
        where WSC: DynWidgetStateContainer + 'static
    {

        let wsc = to_rc_dyn_wsc(sc.clone());

        self.dyn_add(&wsc)
        
    }

    fn on_destroy(&self, widget: Widget, rbp_sc: &RcByPtr<dyn DynWidgetStateContainer>)
    {

        let wbp_sc = rbp_sc.downgrade();

        //let widget = rbp_sc.contents().dyn_widget_adapter_ref().widget()?;

        let weak_self = self.weak_self.clone();

        //let weak_parent = self.weak_parent.clone();

        widget.connect_destroy(move |widget|
        {

            //Upgrade the current state container.

            if let Some(this) = weak_self.upgrade()
            {

                if let Some(rbp_sc) = wbp_sc.upgrade()
                {

                    this.widget_state.borrow_mut_with_param(rbp_sc, |mut state, rbp_sc|
                    {

                        if let Some(wsc_set) = state.get_mut(&widget.type_())
                        {

                            wsc_set.remove(&rbp_sc);

                        }
                        
                    });

                    //let _ = this.remove_by_rc_by_ptr(&rbp_sc);

                }

            }

        });

        //Ok(())

    }

    pub fn remove(&self, sc: &Rc<dyn DynWidgetStateContainer>) -> WidgetUpgradeResult<bool>
    {

        let rbp_sc = RcByPtr::new(sc);

        let glt = rbp_sc.contents_ref().dyn_widget_adapter_ref().glib_type()?;

        self.widget_state.borrow_mut_with_param( rbp_sc, |mut state, rbp_sc|
        {
        
            if let Some(wsc_set) = state.get_mut(&glt)
            {

                return Ok(wsc_set.remove(&rbp_sc));

            }

            Ok(false)

        })

    }

    pub fn remove_by_rc_by_ptr(&self, rbp_sc: &RcByPtr<dyn DynWidgetStateContainer>) -> WidgetUpgradeResult<bool>
    {

        let glt = rbp_sc.contents_ref().dyn_widget_adapter_ref().glib_type()?;

        self.widget_state.borrow_mut_with_param( rbp_sc, |mut state, rbp_sc|
        {
                
            if let Some(wsc_set) = state.get_mut(&glt)
            {

                return Ok(wsc_set.remove(rbp_sc));

            }

            Ok(false)

        })

    }

    pub fn remove_by_widget_ref<W>(&self, widget: &W) -> WidgetUpgradeResult<bool>
        where W: WidgetExt + ObjectExt + Eq
    {

        self.widget_state.borrow_mut_with_param( widget, |mut state, widget|
        {

            let glib_type = widget.type_();

            if let Some(wsc_set) = state.get_mut(&glib_type)
            {

                let mut found_wsc = None;

                for item in wsc_set.iter()
                {

                    if item.contents_ref().dyn_widget_adapter_ref().widget()? == *widget
                    {

                        found_wsc = Some(item.clone());

                        break;

                    }

                }

                if let Some(wsc) = found_wsc
                {

                    return Ok(wsc_set.remove(&wsc));

                }

            }

            Ok(false)

        })

    }

    pub fn contains(&self, sc: &Rc<dyn DynWidgetStateContainer>) -> WidgetUpgradeResult<bool>
    {

        let rbp_sc = RcByPtr::new(sc);

        let glt = rbp_sc.contents_ref().dyn_widget_adapter_ref().glib_type()?;

        if let Some(wsc_set) = self.widget_state.refcell_ref().borrow().get(&glt)
        {

            return Ok(wsc_set.contains(&rbp_sc));

        }

        Ok(false)

    }

    pub fn contains_widget_type(&self, wo: &(dyn WidgetObject)) -> WidgetUpgradeResult<bool>
    {

        let glt = wo.glib_type()?;

        Ok(self.widget_state.refcell_ref().borrow().contains_key(&glt))

    }

    pub fn contains_widget_type_in(&self, sc: &Rc<dyn DynWidgetStateContainer>) -> WidgetUpgradeResult<bool>
    {

        let glt = sc.dyn_widget_adapter_ref().glib_type()?;

        Ok(self.widget_state.refcell_ref().borrow().contains_key(&glt))

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

        self.widget_state.borrow( |state|
        {
            
            let mut lens = HashMap::with_capacity(state.len());

            for bucket in state.iter()
            {

                lens.insert(bucket.0.clone(), bucket.1.len());

            }

            lens

        })

    }

    //L/R

    pub fn dyn_find_state(&self, widget_object: &dyn WidgetObject) -> WidgetUpgradeResult<Option<Rc<dyn DynWidgetStateContainer>>> //WidgetUpgradeLeftRightResult<Option<Rc<dyn DynWidgetStateContainer>>>
    {

        let widget = widget_object.widget()?;

        /*
        let widget;

        match widget_object.widget()
        {

            Ok(val) =>
            {

                widget = val;

            }
            Err(_err) =>
            {

                return Err(WidgetUpgradeLeftRightError::new(WidgetUpgradeErrorSide::Right));

            }

        }
        */

        self.widget_state.borrow_with_param( widget, |state, widget|
        {

            let glt = widget.type_(); //.glib_type()?;

            //Lookup ther bucket

            if let Some(wsc_set) = state.get(&glt)
            {

                //Iterate through the values trying find the state with the widget

                for ws in wsc_set.iter()
                {

                    let contents = ws.contents_ref();

                    if let Ok(res) = contents.dyn_widget_adapter_ref().has(&widget)
                    {

                        if res
                        {

                            return Ok(Some(contents.clone()));

                        }

                    }

                    /*
                    match contents.dyn_widget_adapter_ref().has(&widget) //.widget()?)? //contents.dyn_widget_adapter_ref().dyn_has(widget.dyn_widget())
                    {

                        Ok(res) =>
                        {

                            if res
                            {

                                return Ok(Some(contents.clone()));

                            }

                        }
                        Err(_err) =>
                        {

                            //return Err(WidgetUpgradeLeftRightError::new(WidgetUpgradeErrorSide::Left));

                        }

                    }
                    */

                }

            }

            Ok(None)

        })

    }

    ///
    /// Try find the widget state based on the widget instance.
    ///
    pub fn find_widget_state<T>(&self, widget: &T) -> Option<Rc<dyn DynWidgetStateContainer>> // + MayDowncastTo<Widget>
        where T: WidgetExt + Eq + ObjectExt + Clone + IsA<T>
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

                    if let Ok(wi) = contents.dyn_widget_adapter_ref().has(widget_widget)
                    {

                        if wi
                        {

                            return Some(contents.clone());

                        }

                    }

                }

            }

            None
    
        })

    }

    //L/R

    pub fn dyn_has_state(&self, widget_object: &dyn WidgetObject) -> WidgetUpgradeResult<bool>
    {

        let widget = widget_object.widget()?;

        /*
        let widget;

        match widget_object.widget()
        {

            Ok(val) =>
            {

                widget = val;

            }
            Err(_err) =>
            {

                return Err(WidgetUpgradeLeftRightError::new(WidgetUpgradeErrorSide::Right));

            }

        }
        */

        self.widget_state.borrow_with_param( widget, |state, widget|
        {
        
            let glt = widget.type_();

            //Lookup ther bucket

            if let Some(wsc_set) = state.get(&glt)
            {

                //Iterate through the values trying find the state with the widget

                for ws in wsc_set.iter()
                {

                    let contents = ws.contents_ref();

                    if let Ok(res) = contents.dyn_widget_adapter_ref().has(&widget) //contents.dyn_widget_adapter_ref().dyn_has(widget.dyn_widget())
                    {

                        if res
                        {

                            return Ok(true);

                        }

                    }

                }

            }

            Ok(false)

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