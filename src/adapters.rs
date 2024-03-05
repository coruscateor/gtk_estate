use gtk4 as gtk;

use std::any::Any;

use std::ops::Deref;

use std::rc::Weak;

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use crate::{StateContainers, WidgetStateContainer};

use std::hash::{Hash, Hasher};

use corlib::collections::DynHash;

use corlib::AsAny; //{AsAny, impl_as_any};

pub trait ApplicationObject : Any + AsAny  //: Deref //Any + ApplicationExt +
{

    fn dyn_application(&self) -> &dyn Any;

    fn dyn_has_in_other(&self, other: &dyn ApplicationObject) -> bool;

    fn dyn_has(&self, application: &dyn Any) -> bool;

}

pub trait WidgetObject : Any + AsAny //+ DynHash //+ Eq //Hash  //: WidgetExt + Deref
{

    fn dyn_widget(&self) -> &dyn Any; //_as_any

    fn dyn_has_in_other(&self, other: &dyn WidgetObject) -> bool;

    fn dyn_has(&self, widget: &dyn Any) -> bool;

    //fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F);

    //&WidgetExt

    fn connect_destroy(&self, sc: Weak<StateContainers>);

}

//ApplicationAdapter

pub struct ApplicationAdapter<T: ApplicationExt + Eq>
{

    object: T

}

impl<T: ApplicationExt + Eq> ApplicationAdapter<T>
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

impl<T: ApplicationExt + Eq> ApplicationObject for ApplicationAdapter<T>
{

    fn dyn_application(&self) -> &dyn Any
    {

        &self.object
        
    }
    
    fn dyn_has_in_other(&self, other: &dyn ApplicationObject) -> bool {

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

pub struct WidgetAdapter<T: WidgetExt + Eq>
{

    object: T,
    parent: Weak<dyn WidgetStateContainer>

}

impl<T: WidgetExt + Eq> WidgetAdapter<T>
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


impl<T: WidgetExt + Hash + Eq> WidgetObject for WidgetAdapter<T>
{

    fn dyn_widget(&self) -> &dyn Any
    {

        &self.object    
    
    }
    
    fn dyn_has_in_other(&self, other: &dyn WidgetObject) -> bool {
        
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

    fn connect_destroy(&self, sc: Weak<StateContainers>)
    {

        let parent = self.parent.clone();

        self.object.connect_destroy(move |_widget|
        {

            if let Some(rc_sc) = &sc.upgrade()
            {
                
                if let Some(rc_parent) = &parent.upgrade()
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

impl<T: WidgetExt + Hash> DynHash for WidgetAdapter<T>
{

    //+ ?Sized

    fn dyn_hash(&self, mut state: &mut dyn Hasher)
    { 
        
        self.object.hash(&mut state) //Not completely sure how this works
    
        //Seems like the &mut dyn is cast into a &mut

    }

}

impl<T: WidgetExt> AsAny for WidgetAdapter<T>
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}
