use std::rc::Rc;

use crate::WidgetStateContainer;

pub fn to_rc_dyn_wsc<T: WidgetStateContainer>(value: Rc<T>) -> Rc<dyn WidgetStateContainer>
{

    value

}


