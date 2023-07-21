
use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::time::Duration;

use gtk_estate::corlib::events::SenderEventFunc;
use gtk_estate::corlib::rc_default::RcDefault;
use gtk_estate::gtk4::traits::{BoxExt, WidgetExt};
use gtk_estate::{HasObject, impl_has_box, impl_has_object, StateContainers}; //get_state_containers, 

use gtk_estate::gtk4::{self as gtk, Box, Orientation, Label, BaselinePosition, Align};

use gtk_estate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle, prelude::AdwApplicationWindowExt, gtk::prelude::ApplicationWindowExt, gtk::prelude::GtkWindowExt};

use gtk_estate::corlib::{NonOption, rc_self_setup}; //, rc_self_refcell_setup};

use gtk_estate::time_out::*;

//use gtk_estate::adw::{TabBar, TabPage, TabView};

use tokio::runtime::{Runtime, Handle, Builder};

use gtk_estate::gtk4::glib::object::Cast;

use crate::applicaion_state::ApplicattionState;

pub struct WindowContentsState
{

    weak_self: RefCell<NonOption<Weak<Self>>>,
    contents_box: Box,
    //app_window: ApplicationWindow,
    window_title: WindowTitle,
    hb: HeaderBar,
    tokio_rt_handle: Handle

}

impl WindowContentsState
{

    pub fn new(app_window: &ApplicationWindow) -> Rc<Self>
    {

        let contents_box = Box::new(Orientation::Vertical, 0);

        contents_box.set_vexpand(true);

        let window_title = WindowTitle::new("Escape It", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        contents_box.append(&hb);

        let scs = StateContainers::get();

        let tokio_rt_handle;
        
        {

            let application = app_window.application().unwrap();
    
            let adw_application = application.downcast_ref::<Application>().unwrap();
    
            let applications = scs.adw().borrow_applications();

            let app_state = applications.get(&adw_application).unwrap();
    
            let app_state_ref = app_state.downcast_ref::<ApplicattionState>().unwrap();
    
            tokio_rt_handle = app_state_ref.clone_tokio_rt_handle();

        }

        let this = Self
        {

            weak_self: NonOption::invalid_refcell(),
            contents_box,
            //app_window: app_window.clone(),
            window_title,
            hb,
            tokio_rt_handle

        };

        let rc_self = Rc::new(this);

        //setup weak self reference

        rc_self_setup!(rc_self, weak_self);

        //Add to StateContainers

        scs.gtk().borrow_mut_boxes().add(&rc_self);

        app_window.set_content(Some(&rc_self.contents_box));

        rc_self

    }

}

impl_has_box!(contents_box, WindowContentsState);

