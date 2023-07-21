use gtk4 as gtk;

use gtk::glib::ObjectExt;

use std::{any::Any, cell::RefCell, rc::*};

//use crate::storage_container::StorageContainer;

///
/// Has a GTK/libadwaita object?
/// 
pub trait HasObject<T> : Any //+ Sized
    where T: ObjectExt //+ Sized
{

    fn get_object(&self) -> &T;

}

pub trait InitContainer<T> : HasObject<T> //'a, Any +  , S
    where T: ObjectExt //, S: HasObject<T>
{

    fn new(object: T) -> Rc<RefCell<Self>>; //, sc: &'a StorageContainer<T, S>

}

pub trait InitParamContainer<T, P> : HasObject<T> //'a, Any + S, , S: HasObject<T>
    where T: ObjectExt
{

    fn new_param(object: T, param: P) -> Rc<RefCell<Self>>; //, sc: &StorageContainer<T, S>

}

/*
pub trait Contents_Set_StorageContainer<T, S> : HasObject<T>
    where T: ObjectExt, S: HasObject<T>
//    where Self: Sized
{
    
    //fn new() -> Self;

    fn set_container_ref(&mut self, container: &StorageContainer<T, S>); //ApplicationNode<Self>>>); //container: &'a mut ApplicationNode<'a, Self>);

}
*/

///
/// Fot implementing HasObject on a state container object.
/// 
#[macro_export]
macro_rules! impl_has_object
{

    ($field:ident, $object_type:ty, $for_type:ty) =>
    {

        impl HasObject<$object_type> for $for_type
        {

            fn get_object(&self) -> &$object_type
            {

                &self.$field
                
            }

        }

    }

}

///
/// Fot implementing HasObject on an Application state container object.
/// 
#[macro_export]
macro_rules! impl_has_application
{

    ($field:ident, $for_type:ty) =>
    {

        impl_has_object!($field, Application, $for_type);

    }

}

///
/// Fot implementing HasObject on an ApplicationWindow state container object.
/// 
#[macro_export]
macro_rules! impl_has_application_window
{

    ($field:ident, $for_type:ty) =>
    {

        impl_has_object!($field, ApplicationWindow, $for_type);

    }

}

///
/// Fot implementing HasObject on an Box state container object.
/// 
#[macro_export]
macro_rules! impl_has_box
{

    //use gtk_estate::impl_has_object;

    ($field:ident, $for_type:ty) =>
    {

        impl_has_object!($field, Box, $for_type);

    }

}

///
/// Fot implementing HasObject on an TabPage state container object.
/// 
 #[macro_export]
 macro_rules! impl_has_tab_page
 {
 
     ($field:ident, $for_type:ty) =>
     {
 
         impl_has_object!($field, TabPage, $for_type);
 
     }
 
 }
