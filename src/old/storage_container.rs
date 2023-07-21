use gtk4 as gtk;

use gtk::*;
use gtk::glib::SignalHandlerId;
//use gtk::{Application};
use corlib::events::ListEvent;

//use gtk4::prelude::*;

use crate::object_container::HasObject;
//use crate::has_contents::HasContents;
use crate::{object_container::*, container_node::*};

use std::any::Any;

use std::ops::Fn;

use std::rc::*;

use std::cell::RefCell;

use corlib::*;

use glib::ObjectExt;

//use crate::container_node::*;

/*
pub trait HasApplication : Any //PartialEq + 
{

    fn get_application(&self) -> &Application;

}
*/

//PartialEq +

pub struct StorageContainer<T, S>
    where T: ObjectExt, S: HasObject<T> //NodeContents<Node<T, S>>
{

    //object: T,
    contents: S,
    //query_end_event: Rc<RefCell<ListEvent<Fn(&mut ApplicationContaner<T>, &())>>>,
    //connect_window_added_id: Option<SignalHandlerId>,
    weak_self: NonOption<Weak<RefCell<Self>>>

}

impl<T, S> StorageContainer<T, S>
    where T: ObjectExt, S: HasObject<T> //NodeContents<Node<T, S>>
{

    pub fn new(contents: S) -> Rc<RefCell<Self>> //object: T, 
        where S: Contents_Set_StorageContainer<T, S>
    {

        let mut this = Self
        {

            //object,
            contents,
            //query_end_event: Rc::new(RefCell::new(ListEvent::new())),
            //connect_window_added_id: None
            weak_self: NonOption::invalid()

        };

        let rc_self = Rc::new(RefCell::new(this));

        {

            let mut rc_self_mut = rc_self.borrow_mut();

            let weak_self = Rc::downgrade(&rc_self);

            rc_self_mut.weak_self = NonOption::new(weak_self.clone());   

            rc_self_mut.contents.set_container_ref(weak_self);

        }

        //this.contents.set_container(&mut this);

        rc_self

    }

}

impl<T, S> HasContents<S> for StorageContainer<T, S>
    where T: ObjectExt, S: HasObject<T> //NodeContents<Node<T, S>>
{

    fn get_contents(&self) -> &S
    {

        &self.contents

    }

    fn get_contents_mut(&mut self) -> &mut S
    {

        &mut self.contents

    }

}

/*
impl<S, T> HasContents<T> for Node<S, T>
    where T: NodeContents<Node<S, T>>,
{

    fn get_contents(&self) -> &T
    {

        &self.contents

    }

    fn get_contents_mut(&mut self) -> &mut T
    {

        &mut self.contents

    }

}
*/

/*
impl<T, S> HasContents<T> for Node<T, S>
    where S: NodeContents<Node<T, S>>, T: ObjectExt
{

    fn get_contents(&self) -> &S
    {

        &self.contents

    }

    fn get_contents_mut(&mut self) -> &mut S
    {

        &mut self.contents

    }

}
*/

/*

change the output type to match the trait: `&T`rustcE0053
node.rs(90, 31): original diagnostic
change the output type to match the trait: `&T`rustcE0053
node.rs(90, 31): original diagnostic
method `get_contents` has an incompatible type for trait
expected fn pointer `fn(&Node<T, S>) -> &T`
   found fn pointer `fn(&Node<T, S>) -> &S`
a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parametersrustcE0053
node.rs(86, 6): expected type parameter
node.rs(86, 9): found type parameter
container_node.rs(64, 31): type in trait
node.rs(90, 31): change the output type to match the trait: `&T`
method `get_contents` has an incompatible type for trait
expected fn pointer `fn(&Node<T, S>) -> &T`
   found fn pointer `fn(&Node<T, S>) -> &S`
a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parametersrustcE0053
node.rs(86, 6): expected type parameter
node.rs(86, 9): found type parameter
container_node.rs(64, 31): type in trait
node.rs(90, 31): change the output type to match the trait: `&T`


 */

impl<T, S> GetRcOrWeakRefCellSelf for StorageContainer<T, S>
    where T: ObjectExt, S: HasObject<T> //NodeContents<Node<T, S>>,
{

    fn get_rc_self(&self) -> Rc<RefCell<Self>>
    {

        self.weak_self.get_ref().upgrade().unwrap()

    }

    fn get_weak_self(&self) -> Weak<RefCell<Self>>
    {

        self.weak_self.get_ref().clone()

    }

}

/*
impl<T> HasApplication for ApplicationNode<T>
    where T: NodeContents<ApplicationNode<T>>
{

    fn get_application(&self) -> &Application
    {
        
        &self.application
        
    }

}

impl<T> ContainerNode for ApplicationNode<T>
    where T: NodeContents<ApplicationNode<T>> + 'static
{

    fn get_object_type(&self) -> ObjectType
    {
    
        ObjectType::GTK(GTKObjectType::Application)

    }

}
*/

impl<T, S> HasObject<T> for StorageContainer<T, S>
    where S: HasObject<T> + 'static, T: ObjectExt //NodeContents<Node<T, S>>
{

    fn get_object(&self) -> &T
    {
        
        //&self.object

        self.contents.get_object()

    }

}

impl<T, S> PartialEq<StorageContainer<T, S>> for StorageContainer<T, S>
    where S: HasObject<T>, T: ObjectExt + PartialEq //NodeContents<Node<T, S>>,
{

    fn eq(&self, other: &StorageContainer<T, S>) -> bool
    {

        //self.object == other.object //&& self.contents == other.contents && self.weak_self == other.weak_self

        self.get_object() == other.get_object()

    }

}
