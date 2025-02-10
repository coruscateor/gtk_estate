//use gtk4 as gtk;

use gtk::glib::{object::ObjectExt, signal::SignalHandlerId};

use std::collections::hash_map::{Keys, Iter};
use std::ops::Fn;

use std::collections::HashMap;

use delegate::delegate;

use std::cmp::Eq;

use std::hash::Hash;

///
/// For making it easier to handle multiple signals at once.
/// 
pub struct ScopedSignalHandlerIdHashMap<'a, T, K>
    where T: ObjectExt + Clone
        , K: Eq + Hash
{

    object: &'a T,
    map: HashMap<K, SignalHandlerId> //&'a str

}

impl<'a, T, K> ScopedSignalHandlerIdHashMap<'a, T, K>
    where T: ObjectExt + Clone
        , K: Eq + Hash //+ Clone
{

    pub fn new(object: &'a T) -> Self
    {

        Self
        {

            object, //: object.clone(),
            map: HashMap::new()

        }

    }

    pub fn with_capacity(object: &'a T, capacity: usize) -> Self
    {

        Self
        {

            object, //: object.clone(),
            map: HashMap::with_capacity(capacity)

        }

    }

    /*
    pub fn new_connected(object: &T, id: SignalHandlerId) -> Self
    {

        Self
        {

            object: object.clone(),
            map: HashMap::new()

        }

    }

    pub fn new_connect<F>(object: &T, f: F) -> Self
        where F: Fn(&T) -> SignalHandlerId
    {

        Self
        {

            object: object.clone(),
            map: HashMap::new()

        }

    }
    */

    pub fn get_object_ref(&self) -> &T
    {

        &self.object

    }

    /*
    pub fn get_id_ref(&self) -> &Option<SignalHandlerId>
    {

        &self.id

    }
    */

    pub fn connect<F>(&mut self, key: K, f: F) //&'a str //->
        where F: Fn(&T) -> SignalHandlerId
    {

        self.disconnect(&key);

        self.map.insert(key, f(&self.object));

    }

    pub fn set_connected(&mut self, key: K, id: SignalHandlerId) //&'a str
    {

        self.disconnect(&key);

        self.map.insert(key, id);

    }

    pub fn disconnect(&mut self, key: &K) //&'a str
    {

        let mut removed = self.map.remove(key);

        if let Some(id) = removed.take()
        {

            self.object.disconnect(id) //.clone())

        }

        //self.id = None;

    }

    pub fn disconnect_all(&mut self)
    {

        for (_, id) in self.map.drain() //name
        {

            self.object.disconnect(id)

        }

    }

    /*
    pub fn is_connected(&self, name: &'a str) -> bool
    {

        self.map.contains_key(name)

    }
    */

    delegate! {
        to self.map {

            #[call(contains_key)]
            pub fn is_connected(&self, key: &K) -> bool; //name: &'a str

            pub fn len(&self) -> usize;

            #[call(keys)]
            pub fn names(&self) -> Keys<'_, K, SignalHandlerId>; //&'a str,

            pub fn iter(&self) -> Iter<'_, K, SignalHandlerId>; //&'a str

            pub fn is_empty(&self) -> bool;

        }
    }


    /*
    pub fn len(&self) -> usize
    {

        self.map.len()

    }

    pub fn names(&self) -> Keys<'_, &'a str, SignalHandlerId>
    {

        self.map.keys()

    }

    pub fn iter(&self) -> Iter
    */

}

impl<'a, T, K> Drop for ScopedSignalHandlerIdHashMap<'a, T, K>
    where T: ObjectExt + Clone
        , K: Eq + Hash
{

    fn drop(&mut self)
    {

        self.disconnect_all();

    }

}