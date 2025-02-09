use std::rc::Rc;

use crate::{DynStrongWidgetStateContainer, DynWidgetStateContainer, StrongWidgetObject}; //StoredApplicationObject,

pub fn to_rc_dyn_wsc<T>(value: Rc<T>) -> Rc<dyn DynWidgetStateContainer>
    where T: DynWidgetStateContainer + 'static
{

    value

}

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

pub fn to_rc_dyn_swo<T>(value: Rc<T>) -> Rc<dyn StrongWidgetObject>
    where T: StrongWidgetObject + 'static
{

    value

}

