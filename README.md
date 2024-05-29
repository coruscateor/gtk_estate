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

The StateContainers struct is a singleton and contains widget state association hashmaps, each type of widget gets its own map instance.

</br>

```rust

mod application_state;

use gtk_estate::{adw::{prelude::*, Application}, StateContainers};

use crate::application_state::ApplicationState;

fn main()
{

    let app = Application::builder().application_id("org.example_gui").build();

    //Explicit initialisation of the StateContainers object is no longer required as of v0.3.0.

    //StateContainers::init();

    //The ApplicationState (which you define) should add itself to the StateContainers instance from within its own constructor.

    ApplicationState::new(&app);

    //Run the application

    _ = app.run();

}

```

</br>

In the above example an adw Application is constructed then ApplicationState is instantiated and passed a reference to the Application (a clone of which will become part of its state). An ApplicationState object must implement ApplicationStateContainer, likewise widget state container objects (including windows) must implement WidgetStateContainer (see examples).

StateContainers is a single-threaded singleton which should only contain state which deals with UI and inter-thread-communication related tasks probably using a crate like [act_rs](https://crates.io/crates/act_rs) for the latter.

</br>

## Building Requirements

Requires the GTK4 library binaries on your system (See [The GTK Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html) for GTK installation instructions).

Search your software repositories to find the libadwaita libraries if you want to use any adw features.

</br>

## Features

### GTK4

| Feature | Enabled Feature |
|---------|-----
| gtk_v4_14 | gtk4/v4_14 |
| gtk_v4_12 | gtk4/v4_12 |
| gtk_v4_10 | gtk4/v4_10 |
| gtk_v4_8 |  gtk4/v4_8  |
| gtk_v4_6 |  gtk4/v4_6  |
| gtk_v4_4 |  gtk4/v4_4  |
| gtk_v4_2 |  gtk4/v4_2  |
| gtk_gnome_45 | gtk4/gnome_45 |
| gtk_gnome_44 | gtk4/gnome_44 |
| gtk_gnome_43 | gtk4/gnome_43 |
| gtk_gnome_42 | gtk4/gnome_42 |
| gtk_unsafe-assume-initialized | gtk4/unsafe-assume-initialized |
| gtk_xml_validation | gtk4/xml_validation |
| gtk_blueprint | gtk4/blueprint |

</br>

### libadwaita

| Feature | Enabled Feature |
|---------|-----
| adw | dep:adw |
| adw_gtk_v4_2 | adw/gtk_v4_2 |
| adw_gtk_v4_4 | adw/gtk_v4_4 |
| adw_gtk_v4_6 | adw/gtk_v4_6 |
| adw_gtk_v4_8 | adw/gtk_v4_8 |
| adw_gtk_v4_10 | adw/gtk_v4_10 |
| adw_gtk_v4_12 | adw/gtk_v4_12 |
| adw_v1_2 | adw/v1_2 |
| adw_v1_3 | adw/v1_3 |
| adw_v1_4 | adw/v1_4 |
| adw_v1_5 | adw/v1_5 |

</br>

## Additionally

GTK Estate Re-exposes:

- GTK4 (gtk)
- libadwaita (adw (if selected))
- Corlib (corlib)

</br>

## Todo

- Add more GTK/adw helper functions and helper objects.
- Add more documentation.

</br>

## Examples

- [Simple Unix Time Outputer](https://github.com/coruscateor/simple_unix_time_outputer)

- [Escape It](https://crates.io/crates/escape_it)

- [Req It](https://crates.io/crates/req_it)

</br>

## Coding Style

This project uses a coding style which emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust
fn foo()
{

    bar();

}

```

Not this:

```rust
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
