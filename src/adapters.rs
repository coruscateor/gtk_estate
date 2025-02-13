use std::error::Error;

use std::fmt::Display;
use std::rc::{Rc, Weak};

use gtk::glib::object::{ObjectExt, ObjectType};
use gtk::glib::types::StaticType;
use gtk::glib::{Type, WeakRef};
use corlib::convert::AsAnyRef;
use corlib::impl_as_any_ref_method;
use gtk::prelude::WidgetExt;
use gtk::Widget;

use crate::DynWidgetStateContainer;

use std::any::Any;

use gtk::glib::object::Cast;

#[derive(Debug, Clone)]
pub struct WidgetUpgradeError
{

    static_type: Type

}

impl WidgetUpgradeError
{

    pub fn new(static_type: Type) -> Self
    {

        Self
        {

            static_type

        }

    }

    pub fn static_type_ref(&self) -> &Type
    {

        &self.static_type

    }

}

impl Display for WidgetUpgradeError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        writeln!(f, "Error: failed to upgrade weak Widget.")

    }

}

impl Error for WidgetUpgradeError
{

    /*
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
    */

    //fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}

}

pub type WidgetUpgradeResult<T = ()> = std::result::Result<T, WidgetUpgradeError>;

//Disabled

/*
#[derive(Debug)]
pub enum WidgetUpgradeErrorSide
{

    Left,
    Right

}

/* */
impl Display for WidgetUpgradeErrorSide
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {
            WidgetUpgradeErrorSide::Left =>
            {

                write!(f, "Left")

            }
            WidgetUpgradeErrorSide::Right =>
            {

                write!(f, "Right")

            }

        }

    }

}

#[derive(Debug)]
pub struct WidgetUpgradeLeftRightError
{

    error_side: WidgetUpgradeErrorSide

}

impl WidgetUpgradeLeftRightError
{

    pub fn new(error_side: WidgetUpgradeErrorSide) -> Self
    {

        Self
        {

            error_side

        }

    }

}

impl Display for WidgetUpgradeLeftRightError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self.error_side
        {

            WidgetUpgradeErrorSide::Left =>
            {

                writeln!(f, "Error: failed to upgrade the weak Widget on the left side.")

            }
            WidgetUpgradeErrorSide::Right =>
            {

                writeln!(f, "Error: failed to upgrade the weak Widget on the right side.")

            }

        }

    }

}

impl Error for WidgetUpgradeLeftRightError
{
}

impl Into<WidgetUpgradeError> for WidgetUpgradeLeftRightError
{

    fn into(self) -> WidgetUpgradeError
    {

        WidgetUpgradeError::new()

    }

}

pub type WidgetUpgradeLeftRightResult<T = ()> = std::result::Result<T, WidgetUpgradeLeftRightError>;
*/

pub trait WidgetObject : AsAnyRef
{

    fn glib_type(&self) -> WidgetUpgradeResult<Type>;

    fn has(&self, widget: &Widget) -> WidgetUpgradeResult<bool>;

    fn widget(&self) -> WidgetUpgradeResult<Widget>;

}

#[derive(Clone, Debug)]
pub struct WidgetAdapter<T, P>
    where T: WidgetExt + ObjectExt + Eq + Clone,
          P: DynWidgetStateContainer
{

    weak_widget: WeakRef<T>,
    weak_parent: Weak<P>,
    weak_self: Weak<Self>

}

impl<T, P> WidgetAdapter<T, P>
    where T: WidgetExt + ObjectExt + Eq + Clone,
          P: DynWidgetStateContainer 
{

    pub fn new(widget: &T, weak_parent: &Weak<P>) -> Rc<Self>
    {

        let weak_widget = WeakRef::new();

        weak_widget.set(Some(widget));

        Rc::new_cyclic(|weak_self|
        {    

            Self
            {

                weak_widget,
                weak_parent: weak_parent.clone(),
                weak_self: weak_self.clone()

            }

        })

    }

    pub fn widget(&self) -> WidgetUpgradeResult<T>
    {

        match self.weak_widget.upgrade()
        {

            Some(widget) =>
            {

                Ok(widget)

            }
            None =>
            {

                let stype = T::static_type();  //WeakRef::<T>::GlibType; //WeakRef::<T>::static_type(); //WeakRef::<T>::static_type(); //T::StaticType;

                Err(WidgetUpgradeError::new(stype))

            }

        }

    }

    fn has(&self, widget: &T) -> WidgetUpgradeResult<bool>
    {
        
        let self_widget = self.widget()?;

        Ok(self_widget == *widget)

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

impl<T, P> AsAnyRef for WidgetAdapter<T, P>
    where T: WidgetExt + Eq + ObjectExt + Clone,
          P: DynWidgetStateContainer + 'static
{

    impl_as_any_ref_method!();

}

impl<T, P> WidgetObject for WidgetAdapter<T, P>
    where T: WidgetExt + ObjectExt + Eq + Clone,
          P: DynWidgetStateContainer + 'static
{

    fn glib_type(&self) -> WidgetUpgradeResult<Type>
    {

        let self_widget = self.widget()?;
       
        Ok(self_widget.type_())

    }

    fn has(&self, widget: &Widget) -> WidgetUpgradeResult<bool>
    {
        
        let self_widget = self.widget()?;

        Ok(*widget == self_widget)

    }

    fn widget(&self) -> WidgetUpgradeResult<Widget>
    {

        let self_widget = self.widget()?;

        Ok(self_widget.upcast_ref::<Widget>().clone())

    }

}



