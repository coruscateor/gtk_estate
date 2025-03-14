use gtk::Widget;

//use gtk4 as gtk;

use std::any::Any;

use std::ops::Deref;

use std::rc::{Rc, Weak};

use gtk::gio::prelude::ApplicationExt;

use gtk::prelude::WidgetExt;

use crate::rc_conversions::to_rc_dyn_wsc;

use crate::{DynStrongWidgetStateContainer, DynWidgetStateContainer, StateContainers}; //DynApplicationStateContainer, 

use std::hash::{Hash, Hasher};

use gtk::glib::Type;

use gtk::glib::object::{Cast, IsA, ObjectExt, ObjectType}; 

use corlib::convert::AsAnyRef;

use corlib::{impl_as_any_ref, impl_as_any_ref_method, rc::RcByPtr};

//Disabled

/*
///
/// Implement on an object which stores an Application object for the purpose of dynmically comparing with other objects.
/// 
pub trait LookUpApplicationObject : AsAnyRef //: AsAny + Any //: Deref //Any + ApplicationExt +
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
*/

///
/// Implement on an object which stores a Widget object for the purpose of dynmically comparing with other objects.
/// 
pub trait StrongWidgetObject : AsAnyRef
{

    fn dyn_widget(&self) -> &dyn Any; //_as_any

    fn dyn_has_in_other(&self, other: &dyn StrongWidgetObject) -> bool;

    fn dyn_has(&self, widget: &dyn Any) -> bool;

    //fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F);

    //&WidgetExt

    fn glib_type(&self) -> Type;

    //fn widget(&self) -> Widget;

    fn has(&self, widget: &Widget) -> bool;

    fn widget(&self) -> Widget; //&dyn IsA<Widget>;

    fn widget_ref(&self) -> &Widget;

}

//Disabled

/*
///
/// Indicates that the LookupWidgetObject stored somewhere, perhaps in a Hashmap.
/// 
pub trait StoredWidgetObject : LookupWidgetObject //+ Any
{

    //fn parent(&self) -> &Weak<dyn WidgetStateContainer>;

    fn connect_destroy(&self, sc: Weak<StateContainers>);

}


pub type RcApplicationAdapter<T, P> = Rc<ApplicationAdapter<T, P>>;

*/

//Disabled

//ApplicationAdapter

/*
#[derive(Debug)]
pub struct ApplicationAdapter<T, P>
    where T: ApplicationExt + Eq + ObjectExt + Clone,
          P: DynApplicationStateContainer
{

    object: T,
    //parent: Weak<dyn ApplicationStateContainer>
    weak_parent: Weak<P>,
    weak_self: Weak<Self>

}

impl<T, P> ApplicationAdapter<T, P>
    where T: ApplicationExt + Eq + ObjectExt + Clone,
          P: DynApplicationStateContainer
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
*/

//impl<T: ApplicationExt + Eq + ObjectExt, P: ApplicationStateContainer> StoredApplicationObject for ApplicationAdapter<T, P>
//{

//    fn parent(&self) -> Weak<dyn ApplicationStateContainer>
//    {

        //&self.parent //.clone()

//        let ws: &Weak<dyn ApplicationStateContainer> = &self.parent;

//        ws.clone()

//    }

//}

//impl_as_any_ref!(ApplicationAdapter, T, P);

//Disabled

/*

impl<T, P> AsAnyRef for ApplicationAdapter<T, P>
    where T: ApplicationExt + Eq + ObjectExt + Clone,
          P: DynApplicationStateContainer + 'static
{

    impl_as_any_ref_method!();

}

impl<T, P> LookUpApplicationObject for ApplicationAdapter<T, P>
    where T: ApplicationExt + Eq + ObjectExt,
          P: DynApplicationStateContainer + 'static
          
{

    fn dyn_application(&self) -> &dyn Any
    {

        &self.object
        
    }
    
    fn dyn_has_in_other(&self, other: &dyn LookUpApplicationObject) -> bool
    {

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
          P: DynApplicationStateContainer + 'static
{



}

pub type RcWidgetAdapter<T, P> = Rc<WidgetAdapter<T, P>>;

*/

///
/// Strongly-references implementers of WidgetExt and enables them to be used in scenarios requiring dyn compatibility.
/// 
#[derive(Clone, Debug)]
pub struct StrongWidgetAdapter<T, P>
    where T: WidgetExt + ObjectExt + Eq + Clone,
          P: DynStrongWidgetStateContainer

{

    object: T,
    //parent: Weak<dyn DynWidgetStateContainer>
    weak_parent: Weak<P>,
    weak_self: Weak<Self>

}

impl<T, P> StrongWidgetAdapter<T, P>
    where T: WidgetExt + ObjectExt + Eq + Clone,
          P: DynStrongWidgetStateContainer
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

    fn has_in_other(&self, other: &StrongWidgetAdapter<T, P>) -> bool
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

//Disabled

/*
impl<T, P> StoredWidgetObject for WidgetAdapter<T, P> //Cast + MayDowncastTo<Widget> + IsA<Widget> + /MayDowncastTo<Widget> + IsA<T> + //MayDowncastTo<Widget> //WidgetExt + 
    where T: Eq + ObjectExt + WidgetExt,
          P: DynWidgetStateContainer + 'static
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

                    let wsc_parent: Rc<dyn DynWidgetStateContainer> = to_rc_dyn_wsc(parent); //to_wsc_super(rc_parent); //&rc_parent;

                    //Remove now

                    rc_sc.remove_by_rc_by_ptr(&RcByPtr::new(&wsc_parent));

                    //Don't remove right now but soon.

                    //rc_sc.delayed_removal(&wsc_parent); //&rc_parent); //remove(&rc_parent);

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
*/

//impl_as_any_ref!(WidgetAdapter, T, P);

impl<T, P> AsAnyRef for StrongWidgetAdapter<T, P>
    where T: WidgetExt + Eq + ObjectExt + Clone,
          P: DynStrongWidgetStateContainer + 'static
{

    impl_as_any_ref_method!();

}

impl<T, P> StrongWidgetObject for StrongWidgetAdapter<T, P>
    where T: WidgetExt + Eq + ObjectExt,
          P: DynStrongWidgetStateContainer + 'static
{

    fn dyn_widget(&self) -> &dyn Any
    {

        &self.object    
    
    }
    
    fn dyn_has_in_other(&self, other: &dyn StrongWidgetObject) -> bool
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
    
    fn has(&self, widget: &Widget) -> bool
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

//Disabled

/*
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
        where P: DynWidgetStateContainer
    {
        
        self.object == other.object

    }

    fn has(&self, widget: &T) -> bool
    {
        
        self.object == *widget

    }

}

impl<T> AsAnyRef for LookUpWidgetAdapter<T>
    where T: WidgetExt + Eq + ObjectExt + Clone
{

    impl_as_any_ref_method!();

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
*/

///
/// Implements weak_self methods on your state container object.
/// 
/// Requires std::rc::Weak or similar to be in scope.
/// 
#[macro_export]
macro_rules! impl_weak_self_methods
{

    () =>
    {

        pub fn weak_self(&self) -> Weak<Self>
        {
    
            self.weak_self.clone()
    
        }
    
        pub fn weak_self_ref(&self) -> &Weak<Self>
        {
    
            &self.weak_self
    
        }

    };
    ($adapter_field:ident) =>
    {

        pub fn weak_self(&self) -> Weak<Self>
        {
    
            self.$adapter_field.weak_parent()
    
        }
    
        pub fn weak_self_ref(&self) -> &Weak<Self>
        {
    
            self.$adapter_field.weak_parent_ref()
    
        }

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

/*
#[macro_export]
macro_rules! impl_adapter_accessors
{

    ($field:ident, $adapter_type:ty) => //$adapter_lc:ident, $state_container_type:ty, 
    {

        paste!
        {

            //pub fn [<$adapter_lc _adapter>](&self) -> Rc<WidgetAdapter<$adapter_type, $state_container_type>>
            pub fn $field(&self) -> Rc<WidgetAdapter<$adapter_type, Self>>
            {

                //self. [<$adapter_lc _adapter>] .clone()

                self.$field.clone()

            }

            //pub fn [<$adapter_lc _adapter_ref>](&self) -> &WidgetAdapter<$adapter_type, $state_container_type>
            pub fn [<$field _ref>] (&self) -> &WidgetAdapter<$adapter_type, Self>
            {

                //self. [<$adapter_lc _adapter>] .as_ref()

                &self.$field

            }

        }

    };

}
*/
