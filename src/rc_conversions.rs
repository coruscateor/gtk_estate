use std::rc::Rc;

use crate::{StoredApplicationObject, StoredWidgetObject, DynWidgetStateContainer};

pub fn to_rc_dyn_wsc<T>(value: Rc<T>) -> Rc<dyn DynWidgetStateContainer>
    where T: DynWidgetStateContainer + 'static
{

    value

}

pub fn to_rc_dyn_sao<T>(value: Rc<T>) -> Rc<dyn StoredApplicationObject>
    where T: StoredApplicationObject + 'static
{

    value

}

pub fn to_rc_dyn_swo<T>(value: Rc<T>) -> Rc<dyn StoredWidgetObject>
    where T: StoredWidgetObject + 'static
{

    value

}

