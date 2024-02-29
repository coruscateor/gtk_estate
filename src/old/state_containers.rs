use std::collections::HashMap;

use std::{rc::*, any::Any};

use std::cell::{RefCell, Ref, RefMut};

use corlib::*;

use paste::paste;

use corlib::
{
    
    collections::UniqueItemList,
    //macros::{refcell_get_ref, refcell_get_mut}
    impl_rfc_borrow,
    impl_rfc_borrow_mut,
    impl_rfc_borrow_and_mut,
    GetRcOrWeakRefCellSelf

};

use gtk4 as gtk;

use gtk::{glib::object::ObjectExt}; //, *};

use crate::container_map::*;

//https://gtk-rs.org/gtk-rs-core/stable/0.16/docs/glib/source/index.html

//use crate::{container_vec::*}; //, HasApplication}; //storage_container::*, 

//use adw::*;

static mut STATE_CONTAINERS: NonOption<Weak<StateContainers>> = NonOption::invalid(); //Option<Weak<StateContainers>> = None;

fn get_state_containers() -> Rc<StateContainers>
{

    //check is correct thread?

    unsafe
    {

        STATE_CONTAINERS.get_ref().upgrade().unwrap()

        /*
        if let Some(state_containers) = &STATE_CONTAINERS
        {

            if let Some(rc) = state_containers.upgrade()
            {

                return rc;

            }

        }
        */

    }

    //panic!("STATE_CONTAINERS not initalised")

}

fn try_get_state_containers() -> Option<Rc<StateContainers>>
{

    //check is correct thread?

    unsafe
    {

        let opt = STATE_CONTAINERS.try_get_ref();

        if let Some(weak) = opt
        {

            return Some(weak.upgrade().unwrap());

        }

        /*

        if let Some(state_containers) = &STATE_CONTAINERS
        {

            return state_containers.upgrade();

        }

        */

    }

    None

}

//fn set_ui_nodes(state_containers: Weak<StateContainers>)
fn set_state_containers(state_containers: &Rc<StateContainers>)
{

    //check is correct thread?

    unsafe
    {

        //STATE_CONTAINERS = Some(state_containers);

        STATE_CONTAINERS.set(Rc::downgrade(state_containers))

    }

}

///
/// The applcation state container maps
/// 
pub struct StateContainers
{

    //weak_self: RefCell<NonOption<Weak<Self>>>,
    //applications_any: UniqueItemList<Rc<RefCell<dyn Any>>>,
    //applications: RefCell<NodeVec<Application>>, //<Rc<RefCell<dyn HasObject<Application>>>> //dyn HasApplication>>> //UniqueItemList<Rc<RefCell<dyn HasApplication>>> //dyn HasApplication //dyn HasObject<Application>
    //application_windows: RefCell<NodeVec<ApplicationWindow>>
    gtk_state: GTKState,
    adw_state: AdwState //adwaita_state

}

impl StateContainers
{

    ///
    /// Call this onece: prefereably in the main function.
    /// 
    pub fn new() -> Rc<Self>
    {

        if let Some(state_containers) = try_get_state_containers()
        {

            return state_containers;

        }

        //

        let this = Self
        {

            //weak_self: RefCell::new(NonOption::invalid()),
            //applications: RefCell::new(NodeVec::new()),
            //application_windows: RefCell::new(NodeVec::new())
            gtk_state: GTKState::new(),
            adw_state: AdwState::new() //adwaita_state //AdwaitaState

        };

        let rc_self = Rc::new(this);

        set_state_containers(&rc_self);
        
        /*
        {

            //let mut rc_self_mut = rc_self.borrow_mut();

            let weak_self = Rc::downgrade(&rc_self);

            //rc_self_mut.weak_self = NonOption::new(weak_self.clone());

            rc_self.weak_self.borrow_mut().set(weak_self.clone());

            set_ui_nodes(weak_self);

        }
        */

        //this.contents.set_container(&mut this);

        rc_self

    }

    ///
    /// Get the StateContainers singleton.
    /// 
    /// WARNING: will panic if it hasn't been initalised.
    /// 
    pub fn get() -> Rc<StateContainers>
    {

        get_state_containers()

    }

    ///
    /// Try to get the StateContainers singleton.
    /// 
    pub fn try_get() -> Option<Rc<StateContainers>>
    {

        try_get_state_containers()

    }


    //impl_get_ref!(gtk_state, GTKState);

    //impl_get_ref!(adw_state, AdwState); //adwaita_state //AdwaitaState

    //impl_alias_get_ref!(gtk_state, gtk, GTKState);

    pub fn gtk(&self) -> &GTKState
    {

        &self.gtk_state

    }

    //impl_alias_get_ref!(adw_state, adw, AdwState);

    pub fn adw(&self) -> &AdwState
    {

        &self.adw_state
        
    }

    /*
    pub fn get_applications(&self) -> Ref<'_, NodeVec<Application>>
    {

        self.applications.borrow()

    }

    pub fn get_applications_mut(&self) -> RefMut<'_, NodeVec<Application>>
    {

        self.applications.borrow_mut()

    }

    pub fn get_application_windows(&self) -> Ref<'_, NodeVec<ApplicationWindow>>
    {

        self.application_windows.borrow()

    }

    pub fn get_application_windows_mut(&self) -> RefMut<'_, NodeVec<ApplicationWindow>>
    {

        self.application_windows.borrow_mut()

    }
    */


}

type Map<T> = ContainerMap<T>; //HashMap<T, Rc<dyn Any>>;

///
/// GTK state container maps
/// 
pub struct GTKState
{

    applications: RefCell<Map<gtk::Application>>,
    application_windows: RefCell<Map<gtk::ApplicationWindow>>,
    aspect_frames: RefCell<Map<gtk::AspectFrame>>,
    boxes: RefCell<Map<gtk::Box>>,
    center_boxes: RefCell<Map<gtk::CenterBox>>,
    expanders: RefCell<Map<gtk::Expander>>,
    flow_boxes: RefCell<Map<gtk::FlowBox>>,
    flow_box_children: RefCell<Map<gtk::FlowBoxChild>>,
    grids: RefCell<Map<gtk::Grid>>,
    grid_layouts: RefCell<Map<gtk::GridLayout>>,
    grid_layout_children: RefCell<Map<gtk::GridLayoutChild>>,
    grid_views: RefCell<Map<gtk::GridView>>,
    notebooks: RefCell<Map<gtk::Notebook>>,
    overlays: RefCell<Map<gtk::Overlay>>,
    revealers: RefCell<Map<gtk::Revealer>>,
    scrolled_windows: RefCell<Map<gtk::ScrolledWindow>>,
    search_bars: RefCell<Map<gtk::SearchBar>>,
    stacks: RefCell<Map<gtk::Stack>>,
    viewports: RefCell<Map<gtk::Viewport>>,
    windows: RefCell<Map<gtk::Window>>

}

impl GTKState
{

    pub fn new() -> Self
    {

        Self
        {

            applications: RefCell::new(Map::new()),
            application_windows: RefCell::new(Map::new()),
            aspect_frames: RefCell::new(Map::new()),
            boxes: RefCell::new(Map::new()),
            center_boxes: RefCell::new(Map::new()),
            expanders: RefCell::new(Map::new()),
            flow_boxes: RefCell::new(Map::new()),
            flow_box_children: RefCell::new(Map::new()),
            grids: RefCell::new(Map::new()),
            grid_layouts: RefCell::new(Map::new()),
            grid_layout_children: RefCell::new(Map::new()),
            grid_views: RefCell::new(Map::new()),
            notebooks: RefCell::new(Map::new()),
            overlays: RefCell::new(Map::new()),
            revealers: RefCell::new(Map::new()),
            scrolled_windows: RefCell::new(Map::new()),
            search_bars: RefCell::new(Map::new()),
            stacks: RefCell::new(Map::new()),
            viewports: RefCell::new(Map::new()),
            windows: RefCell::new(Map::new())

        }

    }

    impl_rfc_borrow_and_mut!(applications, Map<gtk::Application>);

    impl_rfc_borrow_and_mut!(application_windows, Map<gtk::ApplicationWindow>);

    impl_rfc_borrow_and_mut!(aspect_frames, Map<gtk::AspectFrame>);

    impl_rfc_borrow_and_mut!(boxes, Map<gtk::Box>);

    impl_rfc_borrow_and_mut!(center_boxes, Map<gtk::CenterBox>);

    impl_rfc_borrow_and_mut!(expanders, Map<gtk::Expander>);

    impl_rfc_borrow_and_mut!(flow_boxes, Map<gtk::FlowBox>);

    impl_rfc_borrow_and_mut!(flow_box_children, Map<gtk::FlowBoxChild>);

    impl_rfc_borrow_and_mut!(grids, Map<gtk::Grid>);

    impl_rfc_borrow_and_mut!(grid_layouts, Map<gtk::GridLayout>);

    impl_rfc_borrow_and_mut!(grid_layout_children, Map<gtk::GridLayoutChild>);

    impl_rfc_borrow_and_mut!(grid_views, Map<gtk::GridView>);

    impl_rfc_borrow_and_mut!(notebooks, Map<gtk::Notebook>);

    impl_rfc_borrow_and_mut!(overlays, Map<gtk::Overlay>);

    impl_rfc_borrow_and_mut!(revealers, Map<gtk::Revealer>);

    impl_rfc_borrow_and_mut!(scrolled_windows, Map<gtk::ScrolledWindow>);

    impl_rfc_borrow_and_mut!(search_bars, Map<gtk::SearchBar>);

    impl_rfc_borrow_and_mut!(stacks, Map<gtk::Stack>);

    impl_rfc_borrow_and_mut!(viewports, Map<gtk::Viewport>);

    impl_rfc_borrow_and_mut!(windows, Map<gtk::Window>);

    //refcell_get_mut_method

}

//https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/

//AboutWindow - https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.AboutWindow.html
//ActionRow - https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.ActionRow.html
//Application - https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.Application.html
//ApplicationWindow - https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.ApplicationWindow.html
//Bin - https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.Bin.html
//Carousel
//Clamp
//ClampScrollable
//- ComboRow
//- EntryRow
//- ExpanderRow
//Flap
//Leaflet
//LeafletPage
//PreferencesGroup
//PreferencesWindow
//Squeezer
//SqueezerPage
//StatusPage
//- TabBar
//TabPage
//TabView
//ViewStack
//ViewStackPage
//ViewSwitcher
//- ViewSwitcherBar
//- ViewSwitcherTitle
//Window

///
/// libadwaita state container maps
/// 
pub struct AdwState //AdwaitaState
{

    //about_windows: RefCell<Map<adw::AboutWindow>>,
    action_rows: RefCell<Map<adw::ActionRow>>,
    applications: RefCell<Map<adw::Application>>,
    application_windows: RefCell<Map<adw::ApplicationWindow>>,
    bins: RefCell<Map<adw::Bin>>,
    carousels: RefCell<Map<adw::Carousel>>,
    clamps: RefCell<Map<adw::Clamp>>,
    clamp_scrollables: RefCell<Map<adw::ClampScrollable>>,
    flaps: RefCell<Map<adw::Flap>>,
    leaflets: RefCell<Map<adw::Leaflet>>,
    leaflet_pages: RefCell<Map<adw::LeafletPage>>,
    preferences_group: RefCell<Map<adw::PreferencesGroup>>,
    preferences_windows: RefCell<Map<adw::PreferencesWindow>>,
    squeezers: RefCell<Map<adw::Squeezer>>,
    squeezer_pages: RefCell<Map<adw::SqueezerPage>>,
    status_pages: RefCell<Map<adw::StatusPage>>,
    tab_pages: RefCell<Map<adw::TabPage>>,
    tab_views: RefCell<Map<adw::TabView>>,
    view_stacks: RefCell<Map<adw::ViewStack>>,
    view_stack_pages: RefCell<Map<adw::ViewStackPage>>,
    view_switchers: RefCell<Map<adw::ViewSwitcher>>,
    windows: RefCell<Map<adw::Window>>

}

impl AdwState
{

    pub fn new() -> Self
    {

        Self
        {

            //about_windows: RefCell::new(Map::new()),
            action_rows: RefCell::new(Map::new()),
            applications: RefCell::new(Map::new()),
            application_windows: RefCell::new(Map::new()),
            bins: RefCell::new(Map::new()),
            carousels: RefCell::new(Map::new()),
            clamps: RefCell::new(Map::new()),
            clamp_scrollables: RefCell::new(Map::new()),
            flaps: RefCell::new(Map::new()),
            leaflets: RefCell::new(Map::new()),
            leaflet_pages: RefCell::new(Map::new()),
            preferences_group: RefCell::new(Map::new()),
            preferences_windows: RefCell::new(Map::new()),
            squeezers: RefCell::new(Map::new()),
            squeezer_pages: RefCell::new(Map::new()),
            status_pages: RefCell::new(Map::new()),
            tab_pages: RefCell::new(Map::new()),
            tab_views: RefCell::new(Map::new()),
            view_stacks: RefCell::new(Map::new()),
            view_stack_pages: RefCell::new(Map::new()),
            view_switchers: RefCell::new(Map::new()),
            windows: RefCell::new(Map::new())

        }

    }

    impl_rfc_borrow_and_mut!(action_rows, Map<adw::ActionRow>);

    impl_rfc_borrow_and_mut!(applications, Map<adw::Application>);

    impl_rfc_borrow_and_mut!(application_windows, Map<adw::ApplicationWindow>);

    impl_rfc_borrow_and_mut!(bins, Map<adw::Bin>);

    impl_rfc_borrow_and_mut!(carousels, Map<adw::Carousel>);

    impl_rfc_borrow_and_mut!(clamps, Map<adw::Clamp>);

    impl_rfc_borrow_and_mut!(clamp_scrollables, Map<adw::ClampScrollable>);

    impl_rfc_borrow_and_mut!(flaps, Map<adw::Flap>);

    impl_rfc_borrow_and_mut!(leaflets, Map<adw::Leaflet>);

    impl_rfc_borrow_and_mut!(leaflet_pages, Map<adw::LeafletPage>);

    impl_rfc_borrow_and_mut!(preferences_group, Map<adw::PreferencesGroup>);

    impl_rfc_borrow_and_mut!(preferences_windows, Map<adw::PreferencesWindow>);

    impl_rfc_borrow_and_mut!(squeezers, Map<adw::Squeezer>);

    impl_rfc_borrow_and_mut!(squeezer_pages, Map<adw::SqueezerPage>);

    impl_rfc_borrow_and_mut!(status_pages, Map<adw::StatusPage>);

    impl_rfc_borrow_and_mut!(tab_pages, Map<adw::TabPage>);

    impl_rfc_borrow_and_mut!(tab_views, Map<adw::TabView>);

    impl_rfc_borrow_and_mut!(view_stacks, Map<adw::ViewStack>);

    impl_rfc_borrow_and_mut!(view_stack_pages, Map<adw::ViewStackPage>);

    impl_rfc_borrow_and_mut!(view_switchers, Map<adw::ViewSwitcher>);

    impl_rfc_borrow_and_mut!(windows, Map<adw::Window>);

}

/*
impl GetRcOrWeakSelf for StateContainers
{

    fn get_rc_self(&self) -> Rc<Self>
    {

        self.weak_self.borrow().get_ref().upgrade().unwrap()
        
    }

    fn get_weak_self(&self) -> Weak<Self>
    {

        self.weak_self.borrow().get_ref().clone()

    }

}
*/

