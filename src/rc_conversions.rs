//!
//! This module contains functions which upcast objects stored in std::rc::Rc objects and return the rc’d dynamic object.
//! 

use std::{any::Any, rc::Rc};

use crate::DynWidgetStateContainer; //StoredApplicationObject,

#[cfg(feature = "strong_widget_state")]
use crate::{DynStrongWidgetStateContainer, StrongWidgetObject};

///
/// Clone the provided DynWidgetStateContainer implementing reference as a dynamic DynWidgetStateContainer reference.
/// 
pub fn to_rc_dyn_wsc<T>(value: &Rc<T>) -> Rc<dyn DynWidgetStateContainer>
    where T: DynWidgetStateContainer + 'static
{

    value.clone()

}

/*
pub fn to_rc_dyn_wsc<T>(value: Rc<T>) -> Rc<dyn DynWidgetStateContainer>
    where T: DynWidgetStateContainer + 'static
{

    value

}
*/

///
/// Clone the provided DynStrongWidgetStateContainer implementing reference as a dynamic DynStrongWidgetStateContainer reference.
/// 
#[cfg(feature = "strong_widget_state")]
pub fn to_rc_dyn_strong_wsc<T>(value: &Rc<T>) -> Rc<dyn DynStrongWidgetStateContainer>
    where T: DynStrongWidgetStateContainer + 'static
{

    value.clone()

}

/*
#[cfg(feature = "strong_widget_state")]
pub fn to_rc_dyn_strong_wsc<T>(value: Rc<T>) -> Rc<dyn DynStrongWidgetStateContainer>
    where T: DynStrongWidgetStateContainer + 'static
{

    value

}
*/

//Disabled

/*
pub fn to_rc_dyn_sao<T>(value: Rc<T>) -> Rc<dyn StoredApplicationObject>
    where T: StoredApplicationObject + 'static
{

    value

}
*/

///
/// Clone the provided StrongWidgetObject implementing reference as a dynamic StrongWidgetObject reference.
/// 
#[cfg(feature = "strong_widget_state")]
pub fn to_rc_dyn_swo<T>(value: &Rc<T>) -> Rc<dyn StrongWidgetObject>
    where T: StrongWidgetObject + 'static
{

    value.clone()

}

///
/// Clone the provided concrite reference
/// 
pub fn to_rc_dyn_any<T>(value: &Rc<T>) -> Rc<dyn Any>
    where T: 'static
{

    value.clone()

}
