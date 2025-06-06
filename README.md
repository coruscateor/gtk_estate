<div align="center">

# GTK Estate

[![Crates.io](https://img.shields.io/crates/v/gtk_estate)](https://crates.io/crates/gtk_estate)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/gtk_estate)](https://crates.io/crates/gtk_estate)
[![Docs](https://docs.rs/gtk_estate/badge.svg)](https://docs.rs/gtk_estate/latest/gtk_estate/)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor)

GTK Estate is a state association library for GTK widgets using the excellent [GTK 4](https://crates.io/crates/gtk4) and [libadwaita](https://crates.io/crates/libadwaita) libraries.

</div>

</br>

The main purpose of GTK Estate is to provide a convenient way to associate user-defined state objects with GTK and libadwaita container widgets. Using user-defined objects you control the user-centric state of their associated widgets and react to their signals. GTK Estate also contains objects such as TimeOut and helper functions that can assist you in building dynamic GTK applications programmatically.

The StateContainers struct is a thread-local singleton and contains widget state association hashmaps, each type of widget gets its own map instance (bucket).

You should read the [gtk4-rs tutorial](https://gtk-rs.org/gtk4-rs/stable/latest/book/), if you haven't already, before proceeding.

</br>

## An Example:

To give you a clear idea of how a GTK Estate application is put together, here is a version of the [Simple Unix Time Outputer](https://github.com/coruscateor/simple_unix_time_outputer) example application code (with some amendments to its imports, so that it will run as a test) (see below for more [example applications](#example-applications)).

<br/>

You can run the following as a test using:

*cargo test --features adw*

<br/>

```rust

//main.rs



//mod applicaion_state;

use gtk_estate::adw::{prelude::*, Application};

//use crate::applicaion_state::ApplicationState;

//mod window_state;

fn main()
{

    //Create an Application object like usual (for GTK/Adwaita programmes oriented around Applications).

    let app = Application::builder().application_id("org.unit_time_gui").build();  

    //Setup the application state.
    
    ApplicationState::new(&app);

    //Run the application

    let run_res = app.run();

    println!("Application ran exiting with code: {}", run_res.value());

}



//applicaion_state.rs



use gtk_estate::corlib::impl_weak_self_trait;

use gtk_estate::gtk::prelude::ApplicationExt;

//use gtk_estate::adw::Application;

use gtk_estate::scs_set_application_state;

use std::rc::{Rc, Weak};

use gtk_estate::StateContainers;

//use crate::window_state::WindowState;

use gtk_estate::corlib::WeakSelf;

pub struct ApplicationState
{

    app: Application,
    weak_self: Weak<ApplicationState>

}

impl ApplicationState
{

    pub fn new(app: &Application) -> Rc<ApplicationState>
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                app: app.clone(),
                weak_self: weak_self.clone()

            }

        });

        app.connect_activate(move |app|
        {

            //new window

            WindowState::new(app);
            
        });

        //Set the application state

        scs_set_application_state!(this);

        this

    }

    pub fn app_ref(&self) -> &Application
    {

        &self.app

    }

}

impl_weak_self_trait!(ApplicationState);



//window_state.rs



//use std::rc::{Weak, Rc};

use gtk_estate::{impl_widget_state_container_traits, scs_add, /*StateContainers,*/ WidgetAdapter, WidgetStateContainer};

use gtk_estate::gtk::prelude::{BoxExt, WidgetExt};

use gtk_estate::gtk::{Box, Orientation, Label, Align};

use gtk_estate::adw::{/*Application,*/ ApplicationWindow, HeaderBar, WindowTitle};

use gtk_estate::corlib::convert::AsAnyRef;

use gtk_estate::{TimeOut, TimeOutRunType};

use time::OffsetDateTime;

use std::any::Any;

use gtk_estate::{DynWidgetStateContainer, WidgetObject};

//use gtk_estate::corlib::WeakSelf;

#[derive(Debug)]
pub struct WindowState
{

    unix_time_label: Label,
    time_out: TimeOut<WindowState>,
    widget_adapter: Rc<WidgetAdapter<ApplicationWindow, Self>>,

}

impl WindowState
{

    pub fn new(application: &Application) -> Rc<Self>
    {

        //Initialise The window content box.

        let cbox = Box::new(Orientation::Vertical, 0);

        cbox.set_vexpand(true);
        
        //HeaderBar

        let window_title = WindowTitle::new("Simple Unix Time Outputer", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        cbox.append(&hb);

        //Internal Content

        let internal_content = Box::new(Orientation::Vertical, 0);

        //The Unix time display Label.

        let unix_time_label = Label::new(Some(""));
        
        internal_content.append(&unix_time_label);

        internal_content.set_vexpand(true);

        internal_content.set_valign(Align::Center);

        cbox.append(&internal_content);

        //Initialise ApplicationWindow

        let builder = ApplicationWindow::builder();

        let window = builder.application(application)
            .default_width(1000)
            .default_height(1000)

            //Set the content of the ApplicationWindow.

            .content(&cbox)
            .visible(true)
            .build();

        //Initialise WindowState

        let this = Rc::new_cyclic( move |weak_self|
        {

            Self
            {

                unix_time_label,
                time_out: TimeOut::new(TimeOutRunType::Seconds(1), weak_self),
                widget_adapter: WidgetAdapter::new(&window, weak_self)

            }

        });

        //Add WindowState to the StateContainers object.

        scs_add!(this);

        //Setup the on_timeout closure.

        let on_timeout = Rc::new(move |this: Rc<Self>|
        {

            let utc_now = OffsetDateTime::now_utc();

            let uts = utc_now.unix_timestamp();

            this.unix_time_label.set_label(&uts.to_string());

            true

        });

        //Set the closure and start the TimeOut.

        this.time_out.set_time_out_fn(&on_timeout);

        this.time_out.start();

        this

    }

}

impl_widget_state_container_traits!(ApplicationWindow, WindowState);


```

</br>

Important details to note about the above example are:

The thread-local application state is set using the scs_set_application_state macro (Calls *StateContainers::set_application_state* basically) and the thread-local window state is set using the *scs_add* macro (This calls *StateContainers::widget_state_ref*).

Setting widget state using these macros (and methods) make these objects thread-locally accessible and allows you to keep *Rc* references around when they would otherwise be dropped.   

The *impl_weak_self_trait* and *impl_widget_state_container_traits* macros implement the various traits (*AsAnyRef*, *DynWidgetStateContainer*, *WidgetStateContainer* and *WeakSelf*) on the constituent objects for convience and so that they can work with the *StateContainers* object (Accessed via *scs_set_application_state* and *scs_add* in this case).

By default *StateContainers* is a thread-local singleton which should only contain state which deals with user-interface and inter-thread-communication related tasks possibly using a crate like [LibSync](https://crates.io/crates/libsync) for the latter.

</br>

## Building Requirements

Requires the GTK4 library binaries on your system (See [The GTK Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html) for GTK installation instructions).

Search your software repositories to find the relevant libadwaita libraries if you want to use any adw features.

</br>

## Building The Documentation

To build the documentation use:

*cargo doc --features strong_widget_state*

or (If applicable)

*cargo +nightly doc --features strong_widget_state*

</br>

## Features

### GTK4

| Feature | Enabled Feature |
|---------|-----
| gtk4_v4_18 | gtk/v4_18 |
| gtk4_v4_16 | gtk/v4_16 |
| gtk4_v4_14 | gtk4/v4_14 |
| gtk4_v4_12 | gtk4/v4_12 |
| gtk4_v4_10 | gtk4/v4_10 |
| gtk4_v4_8 |  gtk4/v4_8  |
| gtk4_v4_6 |  gtk4/v4_6  |
| gtk4_v4_4 |  gtk4/v4_4  |
| gtk4_v4_2 |  gtk4/v4_2  |
| gtk4_gnome_45 | gtk4/gnome_45 |
| gtk4_gnome_44 | gtk4/gnome_44 |
| gtk4_gnome_43 | gtk4/gnome_43 |
| gtk4_gnome_42 | gtk4/gnome_42 |
| gtk4_unsafe-assume-initialized | gtk4/unsafe-assume-initialized |
| gtk4_xml_validation | gtk4/xml_validation |
| gtk4_blueprint | gtk4/blueprint |

</br>

### libadwaita

| Feature | Enabled Feature |
|---------|-----
| adw | dep:adw |
| adw_gtk_v4_2 | adw/gtk_v4_2 |
| adw_gtk_v4_6 | adw/gtk_v4_6 |
| adw_gio_v2_80 | adw/gio_v2_80 |
| adw_v1_1 | adw/v1_1 |
| adw_v1_2 | adw/v1_2 |
| adw_v1_3 | adw/v1_3 |
| adw_v1_4 | adw/v1_4 |
| adw_v1_5 | adw/v1_5 |
| adw_v1_6 | adw/v1_6 |
| adw_v1_7 | adw/v1_7 |

</br>

## Additionally

GTK Estate Re-exposes [gtk4](https://crates.io/crates/gtk4) (gtk), [libadwaita](https://crates.io/crates/libadwaita) (adw (if selected)) and [Corlib](https://crates.io/crates/corlib) (corlib).

</br>

## Example Applications

- [Simple Unix Time Outputer](https://github.com/coruscateor/simple_unix_time_outputer)

- [Mapage Types Viewer](https://github.com/coruscateor/mapage_types_viewer)

- [Req It](https://crates.io/crates/req_it)

- [Escape It](https://crates.io/crates/escape_it)

<br/>

Note: Some of these examples may be out of date.

</br>

## Compiler:

Use the latest stable compiler.

</br>

## Todo

- Add more GTK/adw helper functions and helper objects.
- Add more documentation
- Add procedural macros

</br>

## Coding Style

This project has a coding style which emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust

fn bar() {} 

fn foo()
{

    bar();

}

```

Not this:

```rust

fn bar() {} 

fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
