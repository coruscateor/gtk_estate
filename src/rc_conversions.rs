use std::rc::Rc;

use crate::{StoredApplicationObject, StoredWidgetObject, WidgetStateContainer};

pub fn to_rc_dyn_wsc<T: WidgetStateContainer>(value: Rc<T>) -> Rc<dyn WidgetStateContainer>
{

    value

}

pub fn to_rc_dyn_sao<T: StoredApplicationObject>(value: Rc<T>) -> Rc<dyn StoredApplicationObject>
{

    value

}

pub fn to_rc_dyn_swo<T: StoredWidgetObject>(value: Rc<T>) -> Rc<dyn StoredWidgetObject>
{

    value

}

