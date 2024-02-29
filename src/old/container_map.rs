use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, TryReserveError};
use std::collections::hash_map::{Keys, IntoKeys, Values, ValuesMut, IntoValues, Entry};
use std::rc::Rc;
use std::result::Result;

use std::ops::{Index, IndexMut, RangeBounds};

use std::slice::{Iter, IterMut, SliceIndex};

use std::vec::Drain;

use delegate::delegate;
use gtk4::prelude::ObjectExt;

//use crate::storage_container::*;

use crate::object_container::*;

//use corlib::collections::List;

//#[derive(Default)]

///
/// The object for mapping widgets, applications and windows to aplication state.
/// 
pub struct ContainerMap<T>
    where T: ObjectExt
{

    contents: HashMap<T, Rc<dyn Any>> //dyn HasObject<T>

}

impl<T> ContainerMap<T>
    where T: ObjectExt
{

    //type dyn_object = dyn HasObject<T>;

    //const

    pub fn new() -> Self
    {

        Self
        {

            contents: HashMap::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: HashMap::with_capacity(capacity)

        }

    }

    pub fn add<C>(&mut self, value: &Rc<C>) -> bool //-> Option<
        where C: HasObject<T>
    {

        let key = value.get_object().clone();

        if !self.contents.contains_key(&key)
        {

            return self.contents.insert(key, value.clone()).is_none();

        }

        false

    }

    /*
    the trait `object_container::HasObject` cannot be made into an object
    `object_container::HasObject` cannot be made into an objectrustcE0038
    object_container.rs(10, 14): for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
    */

    pub fn add_refcell<C>(&mut self, value: &Rc<RefCell<C>>) -> bool //-> Option<Rc<RefCell<C>>>
        where C: HasObject<T>
    {

        let key = value.borrow().get_object().clone();

        if !self.contents.contains_key(&key)
        {

            return self.contents.insert(key, value.clone()).is_none();

        }

        false

    }

    /*
    pub fn add_new<C>(&mut self, ui_object: T) -> Option<Rc<RefCell<C>>>
        where T: Clone, C: InitContainer<T>
    {

        if !self.contains_object(&ui_object)
        {

            let container = C::new(ui_object.clone()); //, sc)

            self.contents.push(container.clone());

            return Some(container);

        }

        None

    }

    pub fn add_new_ref<C>(&mut self, ui_object: &T) -> Option<Rc<RefCell<C>>> //'a, value: &Rc<RefCell<dyn HasObject<T>>>) -> bool
        where T: Clone, C: InitContainer<T> //'a, , C>
    {

        if !self.contains_object(ui_object)
        {

            //let sc = StorageContainer::new(contents)

            let container = C::new(ui_object.clone()); //, sc)

            self.contents.push(container.clone());

            return Some(container);

        }

        None

    }

    pub fn add_new_param<C, P>(&mut self, ui_object: T, param: P) -> Option<Rc<RefCell<C>>>
        where T: Clone, C: InitParamContainer<T, P>
    {

        if !self.contains_object(&ui_object)
        {

            let container = C::new_param(ui_object, param);

            self.contents.push(container.clone());

            return Some(container);

        }

        None

    }
    
    pub fn add_new_ref_param<C, P>(&mut self, ui_object: &T, param: P) -> Option<Rc<RefCell<C>>>
        where T: Clone, C: InitParamContainer<T, P>
    {

        if !self.contains_object(ui_object)
        {

            let container = C::new_param(ui_object.clone(), param);

            self.contents.push(container.clone());

            return Some(container);

        }

        None

    }
    */

    //ensure uniqness

    delegate! {
        to self.contents {

            pub fn capacity(&self) -> usize;

            pub fn keys(&self) -> Keys<'_, T, Rc<dyn Any>>;

            pub fn into_keys(self) -> IntoKeys<T, Rc<dyn Any>>;

            pub fn values(&self) -> Values<'_, T, Rc<dyn Any>>;

            /*
                proc macro panicked
                message: called `Result::unwrap()` on an `Err` value: Error("expected `,`")rustc
                proc macro panicked
                message: called `Result::unwrap()` on an `Err` value: Error("expected `,`")
            */

            pub fn values_mut(&mut self) -> ValuesMut<'_, T, Rc<dyn Any>>;

            pub fn into_values(self) -> IntoValues<T, Rc<dyn Any>>;

            pub fn iter(&self) -> std::collections::hash_map::Iter<'_, T, Rc<dyn Any>>;

            pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, T, Rc<dyn Any>>;

            pub fn len(&self) -> usize;

            //pub fn drain(&mut self) -> Drain<'_, T> //, Rc<dyn Any>>;

            //pub fn drain_filter<F>(&mut self, pred: F) -> DrainFilter<'_, T, Rc<dyn Any>, F>;

            pub fn retain<F>(&mut self, f: F)
                where
                F: FnMut(&T, &mut Rc<dyn Any>) -> bool;

            pub fn clear(&mut self);

            pub fn hasher(&self) -> &std::collections::hash_map::RandomState;

            //

            pub fn reserve(&mut self, additional: usize);

            pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;

            pub fn shrink_to_fit(&mut self);

            pub fn shrink_to(&mut self, min_capacity: usize);

            pub fn entry(&mut self, key: T) -> Entry<'_, T, Rc<dyn Any>>;

            pub fn get(&self, k: &T) -> Option<&Rc<dyn Any>>;

            pub fn get_key_value(&self, k: &T) -> Option<(&T, &Rc<dyn Any>)>;

            //pub fn get_many_mut<Q: ?Sized, const N: usize>(&mut self, ks: [&Rc<dyn Any>; N]) -> Option<[&mut Rc<dyn Any>; N]>;

            //get_many_unchecked_mut

            pub fn contains_key(&self, k: &T) -> bool;

            pub fn get_mut(&mut self, k: &T) -> Option<&mut Rc<dyn Any>>;

            pub fn insert(&mut self, k: T, v: Rc<dyn Any>) -> Option<Rc<dyn Any>>;

            //try_insert

            pub fn remove(&mut self, k: &T) -> Option<Rc<dyn Any>>;

            pub fn remove_entry(&mut self, k: &T) -> Option<(T, Rc<dyn Any>)>;

            //raw_entry_mut

            //raw_entry

        }

    }

}

/*
impl<T> UniqueItemList<T> where
    T: Copy + PartialEq
{

    delegate! {
        to self.contents {

            pub fn add_or_repace_copy(&mut self, value: T) -> Option<T>;

        }
    }

}
*/

/*
impl<T> Index<usize> for ContainerMap<T>
    where T: ObjectExt
{

    type Output = Rc<RefCell<dyn HasObject<T>>>;

    delegate! {
        to self.contents {

            fn index(&self, index: usize) -> &Self::Output;

        }
        
    }

    /*
    fn index(&self, index: usize) -> &Self::Output
    {
        
        self.contents.index(index)

    }
    */

}
*/

/*
impl<T> IndexMut<usize> for ContainerMap<T>
    where T: ObjectExt
{
    
    delegate! {
        to self.contents {

            fn index_mut(&mut self, index: usize) -> &mut Self::Output;

        }
        
    }

}
*/

/*
impl<T> NodeVec<T>
    where T: ObjectExt
{


}
*/


