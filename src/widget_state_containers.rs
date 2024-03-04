use std::collections::{HashMap, HashSet};

use std::any::{Any, TypeId};

use std::rc::{Rc, Weak};

use corlib::RcByPtr;

use crate::{StateContainers, WidgetObject, WidgetStateContainer};

pub struct WidgetStateContainers
{

    widget_state: HashMap<TypeId, HashSet<RcByPtr<dyn WidgetStateContainer>>>,
    weak_self: Weak<StateContainers>

}

impl WidgetStateContainers
{

    pub fn new(weak_self: &Weak<StateContainers>) -> Self
    {

        Self
        {

            widget_state: HashMap::new(),
            weak_self: weak_self.clone()

        }

    }

    pub fn with_capacity(weak_self: &Weak<StateContainers>, capacity: usize) -> Self
    {

        Self
        {

            widget_state: HashMap::with_capacity(capacity),
            weak_self: weak_self.clone()

        }

    }

    pub fn capacity(&self) -> usize
    {

        self.capacity()

    }

    pub fn add(&mut self, sc: &Rc<dyn WidgetStateContainer>) -> bool
    {

        let wt_id = sc.widget().type_id();

        let rbp_sc = RcByPtr::new(sc); //: RcByPtr<dyn WidgetStateContainer>

        /*
            the method `clone` exists for struct `RcByPtr<dyn WidgetStateContainer>`, but its trait bounds were not satisfied
            the following trait bounds were not satisfied:
            `dyn state_containers::WidgetStateContainer: Clone`
            which is required by `RcByPtr<dyn state_containers::WidgetStateContainer>: Clone`rustcClick for full compiler diagnostic
            state_containers.rs(47, 1): doesn't satisfy `dyn state_containers::WidgetStateContainer: Clone`
            rc_by_ptr.rs(9, 1): doesn't satisfy `_: Clone`

         */

        if let Some(wsc_set) = self.widget_state.get_mut(&wt_id) // !self.widget_state.contains_key(&wt_id)
        {

            let rbp_sc_2: RcByPtr<dyn WidgetStateContainer> = rbp_sc.clone();

            if wsc_set.insert(rbp_sc)
            {

                //rbp_sc_2.contents().widget().connect_destroy(self.weak_self.clone());

                self.on_destroy(&rbp_sc_2);

                return true;
                
            }

        }
        else
        {
            
            let mut hs = HashSet::new();

            //rbp_sc.contents().widget().connect_destroy(self.weak_self.clone());

            self.on_destroy(&rbp_sc);

            hs.insert(rbp_sc);

            self.widget_state.insert(wt_id, hs);

            return true;

        }

        false

    }

    fn on_destroy(&self, rbp_sc: &RcByPtr<dyn WidgetStateContainer>)
    {

        //Make sure the added state gets removed when its widget gets destroyed.

        rbp_sc.contents().widget().connect_destroy(self.weak_self.clone());

    }

    pub fn remove(&mut self, sc: &Rc<dyn WidgetStateContainer>) -> bool
    {

        let rbp_sc = RcByPtr::new(sc);

        let wt_id = rbp_sc.contents().widget().type_id();

        if let Some(wsc_set) = self.widget_state.get_mut(&wt_id)
        {

            return wsc_set.remove(&rbp_sc);

        }

        false

    }

    pub fn remove_by_rc_by_ptr(&mut self, rbp_sc: &RcByPtr<dyn WidgetStateContainer>) -> bool
    {

        let wt_id = rbp_sc.contents().widget().type_id();

        if let Some(wsc_set) = self.widget_state.get_mut(&wt_id)
        {

            return wsc_set.remove(rbp_sc);

        }

        false

    }

    pub fn contains(&self, sc: &Rc<dyn WidgetStateContainer>) -> bool
    {

        let rbp_sc = RcByPtr::new(sc);

        let wt_id = rbp_sc.contents().widget().type_id();

        if let Some(wsc_set) = self.widget_state.get(&wt_id)
        {

            return wsc_set.contains(&rbp_sc);

        }

        false

    }

    pub fn contains_widget_type(&self, wo: &(dyn WidgetObject)) -> bool
    {

        let wt_id = wo.type_id();

        self.widget_state.contains_key(&wt_id)

    }

    pub fn contains_widget_type_in(&self, sc: &Rc<dyn WidgetStateContainer>) -> bool
    {

        let wt_id = sc.widget().type_id();

        self.widget_state.contains_key(&wt_id)

    }

    pub fn count_of_types(&self) -> usize
    {

        self.widget_state.len()

    }

    pub fn total_count_of_bucket_lens(&self) -> u128
    {

        let mut total: u128 = 0;

        for bucket in self.widget_state.iter()
        {

            total += bucket.1.len() as u128;

        }

        total

    }

    pub fn individual_counts_of_bucket_lens(&self) -> HashMap<TypeId, usize>
    {

        let mut lens = HashMap::with_capacity(self.widget_state.len());

        for bucket in self.widget_state.iter()
        {

            lens.insert(bucket.0.clone(), bucket.1.len());

        }

        lens

    }

}