use gtk4 as gtk;

use std::any::Any;

use std::ops::Deref;

use std::rc::Weak;

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use crate::StateContainers;

pub trait ApplicationObject : Any  //: Deref //Any + ApplicationExt +
{

}

pub trait WidgetObject : Any  //: WidgetExt + Deref
{

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



}

//WidgetAdapter

pub struct WidgetAdapter<T: WidgetExt>
{

    object: T

}

impl<T: WidgetExt> WidgetAdapter<T>
{

    pub fn new(object: T) -> Self
    {

        Self
        {

            object

        }

    }

    pub fn widget(&self) -> &T
    {

        &self.object

    }
    
}


impl<T: WidgetExt> WidgetObject for WidgetAdapter<T>
{

    fn connect_destroy(&self, sc: Weak<StateContainers>)
    {

        todo!()

    }

}
