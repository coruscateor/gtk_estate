//use gtk4 as gtk;

use gtk::glib::{object::ObjectExt, signal::SignalHandlerId};

use std::ops::Fn;

/*
struct ScopedSignalHandlerDetails<T>
    where T: ObjectExt + Clone
{

    id: SignalHandlerId,
    object: T

}

impl<T> ScopedSignalHandlerDetails<T>
{

    pub fn new(id: SignalHandlerId, object: &T) -> Self
    {

        Self
        {

            id,
            object: object.clone()

        }

    }

}
*/

///
/// Makes it easier to handle signals
/// 
pub struct ScopedSignalHandlerId<'a, T>
    where T: ObjectExt + Clone
{

    object: &'a T,
    id: Option<SignalHandlerId>

}

impl<'a, T> ScopedSignalHandlerId<'a, T>
    where T: ObjectExt + Clone
{

    pub fn new(object: &'a T) -> Self
    {

        Self
        {

            id: None,
            object //: object.clone()

        }

    }

    pub fn new_connected(object: &'a T, id: SignalHandlerId) -> Self
    {

        Self
        {

            id: Some(id),
            object //: object.clone()

        }

    }

    pub fn new_connect<F>(object: &'a T, f: F) -> Self
        where F: FnOnce(&T) -> SignalHandlerId
    {

        Self
        {

            id: Some(f(object)),
            object //: object.clone()

        }

    }

    pub fn get_object_ref(&self) -> &T
    {

        &self.object

    }

    pub fn get_id_ref(&self) -> &Option<SignalHandlerId>
    {

        &self.id

    }

    pub fn connect<F>(&mut self, f: F) //->
        where F: FnOnce(&T) -> SignalHandlerId
    {

        self.disconnect();

        self.id = Some(f(&self.object));

    }

    pub fn set_connected(&mut self, id: SignalHandlerId)
    {

        self.disconnect();

        self.id = Some(id);

    }

    pub fn disconnect(&mut self)
    {

        if let Some(id) = self.id.take()
        {

            self.object.disconnect(id) //.clone())

        }

        //self.id = None;

    }

    pub fn is_connected(&self) -> bool
    {

        self.id.is_some()

    }

}

impl<T> Drop for ScopedSignalHandlerId<'_, T>
    where T: ObjectExt + Clone
{

    fn drop(&mut self)
    {

        self.disconnect();

    }

}

