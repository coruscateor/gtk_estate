use std::{any::Any, rc::Rc};

use crate::DynWidgetStateContainer; //StoredApplicationObject,

#[cfg(feature = "strong_widget_state")]
use crate::{DynStrongWidgetStateContainer, StrongWidgetObject};

pub fn to_rc_dyn_wsc<T>(value: Rc<T>) -> Rc<dyn DynWidgetStateContainer>
    where T: DynWidgetStateContainer + 'static
{

    value

}

#[cfg(feature = "strong_widget_state")]
pub fn to_rc_dyn_strong_wsc<T>(value: Rc<T>) -> Rc<dyn DynStrongWidgetStateContainer>
    where T: DynStrongWidgetStateContainer + 'static
{

    value

}

//Disabled

/*
pub fn to_rc_dyn_sao<T>(value: Rc<T>) -> Rc<dyn StoredApplicationObject>
    where T: StoredApplicationObject + 'static
{

    value

}
*/

#[cfg(feature = "strong_widget_state")]
pub fn to_rc_dyn_swo<T>(value: Rc<T>) -> Rc<dyn StrongWidgetObject>
    where T: StrongWidgetObject + 'static
{

    value

}

pub fn to_rc_dyn_any<T>(value: Rc<T>) -> Rc<dyn Any>
    where T: 'static
{

    value

}
