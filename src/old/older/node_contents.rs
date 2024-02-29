use gtk4 as gtk;

use std::rc::Weak;

//use crate::application_node::*;

use std::cell::RefCell;

pub trait NodeContents<T> //<'a>
    where Self: Sized
{
    
    //fn new() -> Self;

    fn set_container(&mut self, container: Weak<RefCell<T>>); //ApplicationNode<Self>>>); //container: &'a mut ApplicationNode<'a, Self>);

}


