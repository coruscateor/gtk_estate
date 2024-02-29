use std::any::Any;

use gtk4 as gtk;

use gtk::{glib::object::ObjectExt, prelude::IsA};

pub enum GTKObjectType
{

    Application,
    ApplicationWindow,
    AspectFrame,
    Box,
    CenterBox,
    Expander,
    FlowBox,
    FlowBoxChild,
    Grid,
    GridLayout,
    GridLayoutChild,
    GridView,
    Notebook,
    Overlay,
    Revealer,
    ScrolledWindow,
    SearchBar,
    Stack,
    Viewport,
    Window

}

pub enum AdwaitaObjectType
{

    Application,
    ApplicationWindow,

}

pub enum ObjectType
{

    GTK(GTKObjectType),
    Adwaita(AdwaitaObjectType)


}

pub trait ContainerNode : Any //<'a>
{

    fn get_object_type(&self) -> ObjectType;

    //fn get_object(&self) -> &'a dyn ObjectExt; // + Sized;

    //fn is(&self, object: &impl IsA<dyn ObjectExt>);

}

pub trait HasContents<T> //<S>
{

    fn get_contents(&self) -> &T; // &S;

    fn get_contents_mut(&mut self) -> &mut T; //&mut S;

}

