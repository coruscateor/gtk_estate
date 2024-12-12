use gtk::Widget;
use gtk4 as gtk;

use std::any::Any;

use std::ops::Deref;

use std::rc::{Rc, Weak};

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use crate::rc_conversions::to_rc_dyn_wsc;
use crate::{ApplicationStateContainer, StateContainers, WidgetStateContainer};

use std::hash::{Hash, Hasher};

use gtk::glib::Type;

use gtk::glib::object::{Cast, IsA, ObjectExt, ObjectType}; 

///
/// Implement on an object which stores an Application object for the purpose of dynmically comparing with other objects.
/// 
pub trait LookUpApplicationObject //: AsAny + Any //: Deref //Any + ApplicationExt +
{

    fn dyn_application(&self) -> &dyn Any;

    fn dyn_has_in_other(&self, other: &dyn LookUpApplicationObject) -> bool;

    fn dyn_has(&self, application: &dyn Any) -> bool;

    fn glib_type(&self) -> Type;

}

///
/// Indicates that the LookUpApplicationObject stored somewhere, perhaps in a Hashmap.
///

pub trait StoredApplicationObject : LookUpApplicationObject //+ Any
{

    //fn parent(&self) -> &Weak<dyn ApplicationStateContainer>;

}

///
/// Implement on an object which stores a Widget object for the purpose of dynmically comparing with other objects.
/// 
pub trait LookupWidgetObject
{

    fn dyn_widget(&self) -> &dyn Any; //_as_any

    fn dyn_has_in_other(&self, other: &dyn LookupWidgetObject) -> bool;

    fn dyn_has(&self, widget: &dyn Any) -> bool;

    //fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F);

    //&WidgetExt

    fn glib_type(&self) -> Type;

    //fn widget(&self) -> Widget;

    fn is(&self, widget: &Widget) -> bool;

    fn widget(&self) -> Widget; //&dyn IsA<Widget>;

    fn widget_ref(&self) -> &Widget;

}

///
/// Indicates that the LookupWidgetObject stored somewhere, perhaps in a Hashmap.
/// 
pub trait StoredWidgetObject : LookupWidgetObject //+ Any
{

    //fn parent(&self) -> &Weak<dyn WidgetStateContainer>;

    fn connect_destroy(&self, sc: Weak<StateContainers>);

}

//ApplicationAdapter

pub struct ApplicationAdapter<T, P>
    where T: ApplicationExt + Eq + ObjectExt + Clone,
          P: ApplicationStateContainer
{

    object: T,
    //parent: Weak<dyn ApplicationStateContainer>
    weak_parent: Weak<P>,
    weak_self: Weak<Self>

}

impl<T, P> ApplicationAdapter<T, P>
    where T: ApplicationExt + Eq + ObjectExt + Clone,
          P: ApplicationStateContainer
{

    pub fn new(object: &T, weak_parent: &Weak<P>) -> Rc<Self> //parent: &Weak<dyn ApplicationStateContainer>) -> Self
    {

        Rc::new_cyclic(|weak_self|
        {

            Self
            {
    
                object: object.clone(),
                weak_parent: weak_parent.clone(),
                weak_self: weak_self.clone()
    
            }

        })

    }

    pub fn application(&self) -> T
    {

        self.object.clone()

    }

    pub fn application_ref(&self) -> &T
    {

        &self.object

    }

    pub fn has_in_other(&self, other: &ApplicationAdapter<T, P>) -> bool
    {
        
        self.object == other.object

    }

    pub fn has(&self, application: &T) -> bool
    {
        
        self.object == *application

    }

    pub fn weak_parent(&self) -> Weak<P>
    {

        self.weak_parent.clone()

    }

    pub fn weak_parent_ref(&self) -> &Weak<P>
    {

        &self.weak_parent

    }

    pub fn weak_self(&self) -> Weak<Self>
    {

        self.weak_self.clone()

    }

    pub fn weak_self_ref(&self) -> &Weak<Self>
    {

        &self.weak_self

    }

}

//impl<T: ApplicationExt + Eq + ObjectExt, P: ApplicationStateContainer> StoredApplicationObject for ApplicationAdapter<T, P>
//{

//    fn parent(&self) -> Weak<dyn ApplicationStateContainer>
//    {

        //&self.parent //.clone()

//        let ws: &Weak<dyn ApplicationStateContainer> = &self.parent;

//        ws.clone()

//    }

//}

impl<T: ApplicationExt + Eq + ObjectExt, P: ApplicationStateContainer> LookUpApplicationObject for ApplicationAdapter<T, P>
{

    fn dyn_application(&self) -> &dyn Any
    {

        &self.object
        
    }
    
    fn dyn_has_in_other(&self, other: &dyn LookUpApplicationObject) -> bool {

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

impl<T, P> StoredApplicationObject for ApplicationAdapter<T, P>
    where T: ApplicationExt + Eq + ObjectExt,
          P: ApplicationStateContainer + 'static
{



}

//WidgetAdapter

#[derive(Clone)]
pub struct WidgetAdapter<T, P>
    where T: Eq + ObjectExt + Clone,
          P: WidgetStateContainer

{

    object: T,
    //parent: Weak<dyn WidgetStateContainer>
    weak_parent: Weak<P>,
    weak_self: Weak<Self>

}

impl<T, P> WidgetAdapter<T, P>
    where T: Eq + ObjectExt + Clone,
          P: WidgetStateContainer
{

    pub fn new(object: &T, weak_parent: &Weak<P>) -> Rc<Self>
    {

        Rc::new_cyclic(|weak_self|
        {    

            Self
            {

                object: object.clone(),
                weak_parent: weak_parent.clone(),
                weak_self: weak_self.clone()

            }

        })

    }

    pub fn widget(&self) -> T
    {

        self.object.clone()

    }

    pub fn widget_ref(&self) -> &T
    {

        &self.object

    }

    fn has_in_other(&self, other: &WidgetAdapter<T, P>) -> bool
    {
        
        self.object == other.object

    }

    fn has(&self, widget: &T) -> bool
    {
        
        self.object == *widget

    }

    pub fn weak_parent(&self) -> Weak<P>
    {

        self.weak_parent.clone()

    }

    pub fn weak_parent_ref(&self) -> &Weak<P>
    {

        &self.weak_parent

    }

    pub fn weak_self(&self) -> Weak<Self>
    {

        self.weak_self.clone()

    }

    pub fn weak_self_ref(&self) -> &Weak<Self>
    {

        &self.weak_self

    }
    
}

impl<T, P> StoredWidgetObject for WidgetAdapter<T, P> //Cast + MayDowncastTo<Widget> + IsA<Widget> + /MayDowncastTo<Widget> + IsA<T> + //MayDowncastTo<Widget> //WidgetExt + 
    where T: Eq + ObjectExt + WidgetExt,
          P: WidgetStateContainer + 'static
{

    /*
    fn parent(&self) -> &Weak<dyn WidgetStateContainer>
    {

        &self.parent

    }
    */

    fn connect_destroy(&self, sc: Weak<StateContainers>)
    {

        let weak_parent = self.weak_parent.clone();

        //let wsc_parent: &Rc<dyn WidgetStateContainer> = &self.parent.clone();

        self.object.connect_destroy(move |_widget|
        {

            //Get the state continers.

            if let Some(rc_sc) = sc.upgrade()
            {

                //Upgrade the current state container.
                
                if let Some(parent) = weak_parent.upgrade()
                {

                    let wsc_parent: Rc<dyn WidgetStateContainer> = to_rc_dyn_wsc(parent); //to_wsc_super(rc_parent); //&rc_parent;

                    //Don't remove right now but soon.

                    rc_sc.delayed_removal(&wsc_parent); //&rc_parent); //remove(&rc_parent);

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

impl<T, P> LookupWidgetObject for WidgetAdapter<T, P>
    where T: Eq + ObjectExt + WidgetExt,
          P: WidgetStateContainer + 'static
{

    fn dyn_widget(&self) -> &dyn Any
    {

        &self.object    
    
    }
    
    fn dyn_has_in_other(&self, other: &dyn LookupWidgetObject) -> bool
    {
        
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
    
    fn is(&self, widget: &Widget) -> bool
    {

        //self.object == widget 

        *widget == self.object

    }

    /*
    fn widget(&self) -> Widget
    {

        self.object.downcast_ref::<Widget>().expect("GTK Estate - Error: Downcast to Widget failed.").clone()
        
    }
    */

    fn widget(&self) -> Widget
    {

        self.object.upcast_ref::<Widget>().clone()
        
    }

    fn widget_ref(&self) -> &Widget
    {

        self.object.upcast_ref::<Widget>()
        
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

/*
impl<T: IsA<Widget>, P: WidgetStateContainer> AsAny for WidgetAdapter<T, P> //WidgetExt
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}
*/

///
///A WidgetAdapter for checking on the existance of state objects.
///
pub struct LookUpWidgetAdapter<T>
    where T: Eq + ObjectExt + Clone
{

    object: T

}

impl<T> LookUpWidgetAdapter<T>
    where T: Eq + ObjectExt + Clone
{

    pub fn new(object: &T) -> Self
    {

        Self
        {

            object: object.clone()

        }

    }
    
    pub fn widget(&self) -> T
    {

        self.object.clone()

    }

    pub fn widget_ref(&self) -> &T
    {

        &self.object

    }

    fn has_in_other<P>(&self, other: &WidgetAdapter<T, P>) -> bool
        where P: WidgetStateContainer
    {
        
        self.object == other.object

    }

    fn has(&self, widget: &T) -> bool
    {
        
        self.object == *widget

    }

}

impl<T> LookupWidgetObject for LookUpWidgetAdapter<T>
    where T: Eq + ObjectExt + WidgetExt
{

    fn dyn_widget(&self) -> &dyn Any
    {

        &self.object    
    
    }
    
    fn dyn_has_in_other(&self, other: &dyn LookupWidgetObject) -> bool
    {
        
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
    
    /*
    fn widget(&self) -> Widget
    {

        self.object.downcast_ref::<Widget>().expect("GTK Estate - Error: Downcast to Widget failed.").clone()
        
    }
    */

    fn widget(&self) -> Widget
    {

        self.object.upcast_ref::<Widget>().clone()
        
    }

    fn widget_ref(&self) -> &Widget
    {

        self.object.upcast_ref::<Widget>()
        
    }

    fn is(&self, widget: &Widget) -> bool
    {

        //self.object == *widget 

        *widget == self.object

    }

}

/*
impl<T: IsA<Widget>> AsAny for LookupWidgetAdapter<T> //WidgetExt
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}
*/

