
use std::rc::{Rc, Weak};

use std::hash::{Hash, Hasher};

use crate::WidgetObject;

//This may be revisited

pub struct HashByWidget<T: WidgetObject + ?Sized>
{

    contents: Rc<T>

}

impl<T: WidgetObject + ?Sized> HashByWidget<T>
{

    pub fn new(contents: Rc<T>) -> Self
    {

        Self
        {

            contents

        }

    }

}

impl<T: WidgetObject + ?Sized> Hash for HashByWidget<T>
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        self.contents.dyn_hash(state);

    }

}

/*
impl<T: WidgetObject +?Sized> PartialEq for HashByWidget<T>
{

    fn eq(&self, other: &Self) -> bool
    {

        Rc::ptr_eq(&self.contents, other.contents())

    }

}

impl<T: WidgetObject +?Sized> Eq for HashByWidget<T> {}
*/

