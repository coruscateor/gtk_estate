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

//use corlib::collections::DynHash;

use corlib::AsAny; //{AsAny, impl_as_any};

//use gtk::glib::object::{IsA, MayDowncastTo, ObjectExt};

use gtk::glib::Type;

use gtk::glib::object::{Cast, IsA, ObjectExt, ObjectType}; //MayDowncastTo, 

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

    //fn parent(&self) -> &Weak<dyn ApplicationStateContainer>;

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

    //fn widget(&self) -> Widget;

    fn is(&self, widget: &Widget) -> bool;

    fn widget(&self) -> Widget; //&dyn IsA<Widget>;

}

//Oops - I tred to downcast when I should've been upcasting.

/*

Turns out trying to cast a generic widget to Widget via Glib/GTK was a bas idea:

error[E0277]: the trait bound `gtk_estate::gtk4::Widget: IsA<gtk_estate::libadwaita::ApplicationWindow>` is not satisfied
   --> src/applicaion_state.rs:63:13
    |
63  |             AdwApplcationWindowState::builder(|builder| {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `IsA<gtk_estate::libadwaita::ApplicationWindow>` is not implemented for `gtk_estate::gtk4::Widget`
    |
    = help: the following other types implement trait `IsA<T>`:
              <gtk_estate::gtk4::Widget as IsA<gtk_estate::gtk4::glib::Object>>
              <gtk_estate::gtk4::Widget as IsA<Accessible>>
              <gtk_estate::gtk4::Widget as IsA<Buildable>>
              <gtk_estate::gtk4::Widget as IsA<ConstraintTarget>>
              <gtk_estate::gtk4::Widget as IsA<gtk_estate::gtk4::Widget>>
    = note: required for `gtk_estate::libadwaita::ApplicationWindow` to implement `MayDowncastTo<gtk_estate::gtk4::Widget>`
note: required by a bound in `AdwApplcationWindowState::<T>::builder`
   --> /run/media/paul/Main Stuff/SoftwareProjects/Rust/gtk_estate/src/adw_application_window_state.rs:100:55
    |
100 |     where T: GtkWindowExt + AdwApplicationWindowExt + MayDowncastTo<Widget> + IsA<Widget> //IsA<T> +  //WidgetExt + 
    |                                                       ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `AdwApplcationWindowState::<T>::builder`
...
145 |     pub fn builder<F>(window_fn: F) -> Rc<Self>
    |            ------- required by a bound in this associated function

For more information about this error, try `rustc --explain E0277`.
warning: `simple_unix_time_outputer` (bin "simple_unix_time_outputer") generated 8 warnings
error: could not compile `simple_unix_time_outputer` (bin "simple_unix_time_outputer") due to 1 previous error; 8 warnings emitted

*/


///
/// Indicates that the LookupWidgetObject stored somewhere, perhaps in a Hashmap.
/// 
pub trait StoredWidgetObject : LookupWidgetObject + Any
{

    //fn parent(&self) -> &Weak<dyn WidgetStateContainer>;

    fn connect_destroy(&self, sc: Weak<StateContainers>);

}

/*
pub fn to_wsc_super<T: WidgetStateContainer>(value: Rc<T>) -> Rc<dyn WidgetStateContainer>
{

    value

}
*/

//ApplicationAdapter

pub struct ApplicationAdapter<T: ApplicationExt + Eq + ObjectExt + Clone, P: ApplicationStateContainer>
{

    object: T,
    //parent: Weak<dyn ApplicationStateContainer>
    weak_parent: Weak<P>,
    weak_self: Weak<Self>

}

impl<T: ApplicationExt + Eq + ObjectExt + Clone, P: ApplicationStateContainer> ApplicationAdapter<T, P>
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

    /*
    pub fn new_mv(object: &T, parent: Weak<P>)
    {

        Self
        {

            object: object.clone(),
            parent: parent

        }

    }
    */

    pub fn application(&self) -> &T
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

impl<T: ApplicationExt + Eq + ObjectExt, P: ApplicationStateContainer> LookupApplicationObject for ApplicationAdapter<T, P>
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

impl<T: ApplicationExt + Eq + ObjectExt, P: ApplicationStateContainer> StoredApplicationObject for ApplicationAdapter<T, P>
{



}

//impl_as_any!(ApplicationAdapter, T);

impl<T: ApplicationExt, P: ApplicationStateContainer> AsAny for ApplicationAdapter<T, P>
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}

//WidgetAdapter

#[derive(Clone)]
pub struct WidgetAdapter<T: Eq + ObjectExt + Clone, P: WidgetStateContainer> // + IsA<Widget> //WidgetExt + 
{

    object: T,
    //parent: Weak<dyn WidgetStateContainer>
    parent: Weak<P>,
    weak_self: Weak<Self>

}

impl<T: Eq + ObjectExt + Clone, P: WidgetStateContainer> WidgetAdapter<T, P> // + IsA<Widget> //WidgetExt + 
{

    /*
    pub fn dyn_new(object: T, parent: &Weak<dyn WidgetStateContainer>) -> Self
    {

        Self
        {

            object,
            parent: parent.clone()

        }

    }
    */

    pub fn new(object: &T, parent: &Weak<P>) -> Rc<Self> //<WSC> //Weak<dyn WidgetStateContainer>
        //where WSC: WidgetStateContainer
    {

        //let any_wsc: &dyn Any = parent;

        //let casted_wsc = any_wsc.downcast_ref::<Weak<dyn WidgetStateContainer>>().expect("GTK Estate - Error: Weak<dyn WidgetStateContainer> cast failed.");

        Rc::new_cyclic(|weak_self|
        {    

            Self
            {

                object: object.clone(),
                parent: parent.clone(), //casted_wsc.clone()
                weak_self: weak_self.clone()

            }

        })

    }

    pub fn widget(&self) -> &T
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

        self.parent.clone()

    }

}

impl<T: Eq + ObjectExt + WidgetExt, P: WidgetStateContainer> StoredWidgetObject for WidgetAdapter<T, P> //Cast + MayDowncastTo<Widget> + IsA<Widget> + /MayDowncastTo<Widget> + IsA<T> + //MayDowncastTo<Widget> //WidgetExt + 
{

    /*
    fn parent(&self) -> &Weak<dyn WidgetStateContainer>
    {

        &self.parent

    }
    */

    fn connect_destroy(&self, sc: Weak<StateContainers>)
    {

        let parent = self.parent.clone();

        //let wsc_parent: &Rc<dyn WidgetStateContainer> = &self.parent.clone();

        self.object.connect_destroy(move |_widget|
        {

            //Get the state continers.

            if let Some(rc_sc) = sc.upgrade()
            {

                //Upgrade the current state container.
                
                if let Some(rc_parent) = parent.upgrade()
                {

                    let wsc_parent: Rc<dyn WidgetStateContainer> = to_rc_dyn_wsc(rc_parent); //to_wsc_super(rc_parent); //&rc_parent;

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

impl<T: Eq + ObjectExt + Clone + WidgetExt, P: WidgetStateContainer> LookupWidgetObject for WidgetAdapter<T, P> //+ Cast + MayDowncastTo<Widget> + IsA<Widget> //MayDowncastTo<Widget> + //IsA<T> + //MayDowncastTo<Widget> //WidgetExt + 
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

impl<T: IsA<Widget>, P: WidgetStateContainer> AsAny for WidgetAdapter<T, P> //WidgetExt
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}

///
///A WidgetAdapter for checking on the existance of state objects.
///
pub struct LookupWidgetAdapter<T: Eq + ObjectExt + Clone> // + IsA<Widget> //WidgetExt + 
{

    object: T

}

impl<T: Eq + ObjectExt + Clone> LookupWidgetAdapter<T> // + IsA<Widget> //WidgetExt + 
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

impl<T: Eq + ObjectExt + WidgetExt> LookupWidgetObject for LookupWidgetAdapter<T> // + PartialEq<T> // + Cast + MayDowncastTo<Widget> + IsA<Widget> //IsA<T> + //WidgetExt + //MayDowncastTo<Widget> 
{

    //IsA<Widget> + 

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

    fn is(&self, widget: &Widget) -> bool
    {

        //self.object == *widget 

        *widget == self.object

    }

}

impl<T: IsA<Widget>> AsAny for LookupWidgetAdapter<T> //WidgetExt
{

    fn as_any(&self) -> &dyn Any
    {

        self
        
    }

}

