use gtk4 as gtk;

use std::any::Any;

use std::ops::Deref;

use std::rc::Weak;

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use crate::{ApplicationStateContainer, StateContainers, WidgetStateContainer};

use std::hash::{Hash, Hasher};

use corlib::collections::DynHash;

use corlib::AsAny; //{AsAny, impl_as_any};

use gtk::glib::object::ObjectExt;

use gtk::glib::Type;

///
/// Implement on an object which stores an Application object for the purpose of dynmically comparing with other objects.
/// 
pub trait LookupApplicationObject : AsAny + Any //: Deref //Any + ApplicationExt +
{

    fn dyn_application(&self) -> &dyn Any;

    fn dyn_has_in_other(&self, other: &dyn LookupApplicationObject) -> bool;

    fn dyn_has(&self, application: &dyn Any) -> bool;

    fn glib_type(&self) -> Type;

}

///
/// Indicates that the LookupApplicationObject stored somewhere, perhaps in a Hashmap.
/// 
pub trait StoredApplicationObject : LookupApplicationObject + Any
{

    fn parent(&self) -> &Weak<dyn ApplicationStateContainer>;

}

///
/// Implement on an object which stores an Widget object for the purpose of dynmically comparing with other objects.
/// 
pub trait LookupWidgetObject : AsAny + Any //+ DynHash //+ Eq //Hash  //: WidgetExt + Deref
{

    fn dyn_widget(&self) -> &dyn Any; //_as_any

    fn dyn_has_in_other(&self, other: &dyn LookupWidgetObject) -> bool;

    fn dyn_has(&self, widget: &dyn Any) -> bool;

    //fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F);

    //&WidgetExt

    fn glib_type(&self) -> Type;

}

///
/// Indicates that the LookupWidgetObject stored somewhere, perhaps in a Hashmap.
/// 
pub trait StoredWidgetObject : LookupWidgetObject + Any
{

    fn parent(&self) -> &Weak<dyn WidgetStateContainer>;

    fn connect_destroy(&self, sc: Weak<StateContainers>);

}

//ApplicationAdapter

pub struct ApplicationAdapter<T: ApplicationExt + Eq + ObjectExt>
{

    object: T

}

impl<T: ApplicationExt + Eq + ObjectExt> ApplicationAdapter<T>
{

    pub fn new(object: T) -> Self
    {

        Self
        {

            object

        }

    }

    pub fn application(&self) -> &T
    {

        &self.object

    }

    pub fn has_in_other(&self, other: &ApplicationAdapter<T>) -> bool
    {
        
        self.object == other.object

    }

    pub fn has(&self, application: &T) -> bool
    {
        
        self.object == *application

    }

}

impl<T: ApplicationExt + Eq + ObjectExt> LookupApplicationObject for ApplicationAdapter<T>
{

    fn dyn_application(&self) -> &dyn Any
    {

        &self.object
        
    }
    
    fn dyn_has_in_other(&self, other: &dyn LookupApplicationObject) -> bool {

        self.dyn_has(other.dyn_application())

        //let dyn_other = other as &dyn Any;

        /*
        if let Some(other_dc_ref) = other.downcast_ref::<ApplicationAdapter<T>>()
        {

            self.object == other_dc_ref.object

        }
        else
        {
            
            false

        }
        */
        
    }
    
    fn dyn_has(&self, application: &dyn Any) -> bool
    {

        if let Some(application_dc_ref) = application.downcast_ref::<T>()
        {

            self.object == *application_dc_ref

        }
        else
        {
            
            false

        }
       
    }
    
    fn glib_type(&self) -> Type
    {

        self.object.type_()

    }

}

//impl_as_any!(ApplicationAdapter, T);

impl<T: ApplicationExt> AsAny for ApplicationAdapter<T>
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}

//WidgetAdapter

pub struct WidgetAdapter<T: WidgetExt + Eq + ObjectExt>
{

    object: T,
    parent: Weak<dyn WidgetStateContainer>

}

impl<T: WidgetExt + Eq + ObjectExt> WidgetAdapter<T>
{

    pub fn new(object: T, parent: &Weak<dyn WidgetStateContainer>) -> Self
    {

        Self
        {

            object,
            parent: parent.clone()

        }

    }

    pub fn widget(&self) -> &T
    {

        &self.object

    }

    fn has_in_other(&self, other: &WidgetAdapter<T>) -> bool
    {
        
        self.object == other.object

    }

    fn has(&self, widget: &T) -> bool
    {
        
        self.object == *widget

    }

}

impl<T: WidgetExt + Eq + ObjectExt> StoredWidgetObject for WidgetAdapter<T>
{

    fn parent(&self) -> &Weak<dyn WidgetStateContainer>
    {

        &self.parent

    }

    fn connect_destroy(&self, sc: Weak<StateContainers>)
    {

        let parent = self.parent.clone();

        self.object.connect_destroy(move |_widget|
        {

            //Get the state continers.

            if let Some(rc_sc) = sc.upgrade()
            {

                //Upgrade the current state container.
                
                if let Some(rc_parent) = parent.upgrade()
                {

                    //Don't remove right now but soon.

                    rc_sc.delyed_removal(&rc_parent); //remove(&rc_parent);

                }

            }

        });

        /*
        if let Some(rc_sc) = sc.upgrade()
        {

            if let Some(parent) = self.parent.upgrade()
            {

                self.object.connect_destroy(move |_widget|
                {
    
                    rc_sc.remove(&parent);
        
                });

            }

        }
        */

    }

}

impl<T: WidgetExt + Eq + ObjectExt> LookupWidgetObject for WidgetAdapter<T>
{

    fn dyn_widget(&self) -> &dyn Any
    {

        &self.object    
    
    }
    
    fn dyn_has_in_other(&self, other: &dyn LookupWidgetObject) -> bool {
        
        self.dyn_has(other.dyn_widget())

        /*
        let dyn_other: &dyn Any = &other;

        if let Some(other_dc_ref) = dyn_other.downcast_ref::<WidgetAdapter<T>>()
        {

            self.object == other_dc_ref.object

        }
        else
        {
            
            false

        }
        */
        
    }
    
    fn dyn_has(&self, widget: &dyn Any) -> bool
    {

        if let Some(widget_dc_ref) = widget.downcast_ref::<T>()
        {

            //self.object == *widget_dc_ref

            self.has(widget_dc_ref)

        }
        else
        {
            
            false

        }
       
    }

    fn glib_type(&self) -> Type
    {

        self.object.type_()

    }

}

/*
impl<T: WidgetExt + Hash> DynHash for WidgetAdapter<T>
{

    //+ ?Sized

    fn dyn_hash(&self, mut state: &mut dyn Hasher)
    { 
        
        self.object.hash(&mut state) //Not completely sure how this works
    
        //Seems like the &mut dyn is cast into a &mut

    }

}
*/

impl<T: WidgetExt> AsAny for WidgetAdapter<T>
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}

///
///A WidgetAdapter for checking on the existance of state objects.
///
pub struct LookupWidgetAdapter<T: WidgetExt + Eq + ObjectExt + Clone>
{

    object: T

}

impl<T: WidgetExt + Eq + ObjectExt + Clone> LookupWidgetAdapter<T>
{

    pub fn new(object: &T) -> Self
    {

        Self
        {

            object: object.clone()

        }

    }

    pub fn widget(&self) -> &T
    {

        &self.object

    }

    fn has_in_other(&self, other: &WidgetAdapter<T>) -> bool
    {
        
        self.object == other.object

    }

    fn has(&self, widget: &T) -> bool
    {
        
        self.object == *widget

    }

}

impl<T: WidgetExt + Eq + ObjectExt> LookupWidgetObject for LookupWidgetAdapter<T>
{

    fn dyn_widget(&self) -> &dyn Any
    {

        &self.object    
    
    }
    
    fn dyn_has_in_other(&self, other: &dyn LookupWidgetObject) -> bool {
        
        self.dyn_has(other.dyn_widget())
        
    }
    
    fn dyn_has(&self, widget: &dyn Any) -> bool
    {

        if let Some(widget_dc_ref) = widget.downcast_ref::<T>()
        {

            self.has(widget_dc_ref)

        }
        else
        {
            
            false

        }
       
    }

    fn glib_type(&self) -> Type
    {

        self.object.type_()

    }

}

impl<T: WidgetExt> AsAny for LookupWidgetAdapter<T>
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}

