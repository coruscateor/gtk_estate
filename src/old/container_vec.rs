use std::cell::RefCell;
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
pub struct ContainerVec<T>
    where T: ObjectExt
{

    contents: Vec<Rc<RefCell<dyn HasObject<T>>>>

}

impl<T> ContainerVec<T>
    where T: ObjectExt
{

    //type dyn_object = dyn HasObject<T>;

    //const

    pub fn new() -> Self
    {

        Self
        {

            contents: Vec::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: Vec::with_capacity(capacity)

        }

    }

    pub fn contains_object(&self, object: &T) -> bool
    {

        for item in &self.contents
        {

            if item.borrow().get_object() == object //.eq(object)
            {

                return true;

            }

        }

        false

    }

    pub fn index_of_object(&self, object: &T) -> Option<usize>
    {

        let mut index: usize = 0;

        for item in &self.contents
        {

            if item.borrow().get_object() == object
            {

                return Some(index);

            }

            index += 1;

        }

        None

    }

    pub fn add(&mut self, value: &Rc<RefCell<dyn HasObject<T>>>) -> bool
    {

        if !self.contains_object(value.borrow().get_object())
        {

            self.contents.push(value.clone());

            return true;

        }

        false

    }

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

    //count refs

    pub fn remove_object(&mut self, object: &T) -> bool
    {

        let index_option = self.index_of_object(object);

        if let Some(index) = index_option
        {

            self.contents.remove(index);

            return true;

        }

        false

    }

    pub fn remove_and_get(&mut self, object: &T) -> Option<Rc<RefCell<dyn HasObject<T>>>>
    {

        let index_option = self.index_of_object(object);

        if let Some(index) = index_option
        {

            return Some(self.contents.remove(index));

        }

        None

    }

    //ensure uniqness

    delegate! {
        to self.contents {

            //pub const fn new(&self) -> Self;

            //pub fn add_or_repace(&mut self, value: T);

            //pub fn contains(&self, x: &T) -> bool;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            //pub fn to_vec(&self) -> Vec<T> where T: Clone;

            pub fn pop(&mut self) -> Option<Rc<RefCell<dyn HasObject<T>>>>;

            //pub fn insert(&mut self, index: usize, element: T);

            //#[call(remove)]
            //pub fn remove_at(&mut self, index: usize) -> Rc<RefCell<dyn HasObject<T>>>;

            pub fn first(&self) -> Option<&Rc<RefCell<dyn HasObject<T>>>>;

            pub fn first_mut(&mut self) -> Option<&mut Rc<RefCell<dyn HasObject<T>>>>;

            pub fn last(&self) -> Option<&Rc<RefCell<dyn HasObject<T>>>>;

            pub fn last_mut(&mut self) -> Option<&mut Rc<RefCell<dyn HasObject<T>>>>;

            pub fn iter(&self) -> Iter<'_, Rc<RefCell<dyn HasObject<T>>>>;

            pub fn iter_mut(&mut self) -> IterMut<'_, Rc<RefCell<dyn HasObject<T>>>>;

            //#[call(push)]
            //pub fn add(&mut self, value: T);

            //pub fn get_last_index(&self) -> Option<usize>;

            //pub fn index_of(&self, value: &T) -> Option<usize>;

            pub fn reverse(&mut self);

            //pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output> where I: SliceIndex<[T]>;

            //pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output> where I: SliceIndex<[T]>;

            pub fn drain<R>(&mut self, range: R) -> Drain<'_, Rc<RefCell<dyn HasObject<T>>>> where R: RangeBounds<usize>;

            //pub fn drain_all(&mut self) -> Drain<'_, T>;

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

impl<T> Index<usize> for ContainerVec<T>
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

impl<T> IndexMut<usize> for ContainerVec<T>
    where T: ObjectExt
{
    
    delegate! {
        to self.contents {

            fn index_mut(&mut self, index: usize) -> &mut Self::Output;

        }
        
    }

}

/*
impl<T> NodeVec<T>
    where T: ObjectExt
{


}
*/


