use gtk4 as gtk;

use std::any::Any;

use std::ops::Deref;

use std::rc::Weak;

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use crate::{StateContainers, WidgetStateContainer};

use std::hash::{Hash, Hasher};

use corlib::collections::DynHash;

pub trait ApplicationObject : Any  //: Deref //Any + ApplicationExt +
{

    fn dyn_application(&self) -> &dyn Any;

}

pub trait WidgetObject : Any //+ DynHash //+ Eq //Hash  //: WidgetExt + Deref
{

    fn dyn_widget(&self) -> &dyn Any; //_as_any

    //fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F);

    //&WidgetExt

    fn connect_destroy(&self, sc: Weak<StateContainers>);

}

//ApplicationAdapter

pub struct ApplicationAdapter<T: ApplicationExt>
{

    object: T

}

impl<T: ApplicationExt> ApplicationAdapter<T>
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
    
}

impl<T: ApplicationExt> ApplicationObject for ApplicationAdapter<T>
{

    fn dyn_application(&self) -> &dyn Any
    {

        &self.object
        
    }

}

//WidgetAdapter

pub struct WidgetAdapter<T: WidgetExt>
{

    object: T,
    parent: Weak<dyn WidgetStateContainer>

}

impl<T: WidgetExt> WidgetAdapter<T>
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
    
}


impl<T: WidgetExt + Hash> WidgetObject for WidgetAdapter<T>
{

    fn dyn_widget(&self) -> &dyn Any //_as_any
    {

        &self.object    
    
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
