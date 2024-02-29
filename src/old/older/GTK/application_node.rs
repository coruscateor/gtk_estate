use gtk4 as gtk;

use gtk::*;
use gtk::glib::SignalHandlerId;
use gtk::{Application};
use corlib::events::ListEvent;

//use gtk4::prelude::*;

//use crate::has_contents::HasContents;
use crate::{node_contents::*, container_node::*};

use std::ops::Fn;

use std::rc::*;

use std::cell::RefCell;

use corlib::*;

//use crate::container_node::*;

pub trait HasApplication
{

    fn get_application(&self) -> &Application;

}

pub struct ApplicationNode<T>
    where T: NodeContents<ApplicationNode<T>>
{

    application: Application,
    contents: T,
    //query_end_event: Rc<RefCell<ListEvent<Fn(&mut ApplicationContaner<T>, &())>>>,
    //connect_window_added_id: Option<SignalHandlerId>,
    weak_self: NonOption<Weak<RefCell<Self>>>

}

impl<T> ApplicationNode<T>
    where T: NodeContents<ApplicationNode<T>>
{

    pub fn new(application: Application, contents: T) -> Rc<RefCell<Self>>
    {

        let mut this = Self
        {

            application,
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

            rc_self_mut.contents.set_container(weak_self);

        }

        //this.contents.set_container(&mut this);

        rc_self

    }

}

impl<T> HasContents<T> for ApplicationNode<T>
    where T: NodeContents<ApplicationNode<T>>
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

impl<T> GetRcOrWeakRefCellSelf for ApplicationNode<T>
    where T: NodeContents<ApplicationNode<T>>
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
