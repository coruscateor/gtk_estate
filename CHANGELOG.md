# Changelog

## Version 0.3.0 (03/06/2024)

- Cleaned up lib.rs
- Removed a bunch of old code.
- Updated the Corlib RefCell borrowing macros in the state_containers module.
- Cleaned up TimeOut, updated its Corlib macro imports and added a generic state field.
- Added a state object and related methods to SimpleTimeOut.
- Added a “state” method to TimeOut which returns a reference to its state object.
- Removed the gtk_estate.code-workspace file and added it to the .gitignore.
- Cleaned up SimpleTimeOut and renamed the field “function” to “on_time_out_fn” as well as “set_function” to “set_on_time_out_fn”, “has_function” to “has_on_time_out_fn”, “remove_function” to “remove_on_time_out_fn” and “remove_function_only” to “remove_on_time_out_fn_only”.
- Added a getter for “interval”, so you can get the set interval for the SimpleTimeOut.
- StateContainers is now lazily loaded.
- Updated the documentation of TimeOut and added an interval getter method to it.
- Updated the readme
- Added try_find_parent and find_parent functions to the widget_ext helpers module.
- Updated the Corlib dependency to v0.3.0.
- Added docs.rs documentation build configurations to the cargo.toml and lib.rs files.
- Updated the StateContainers documentation.
- Directly exposed the time_out module contents in lib.rs.

## Version 0.2.0 (22/04/2024)

- Updated dependencies (gtk4 to "0.8.*" adw to "0.6.*" and corlib to "0.2.0")
- Re-wrote GTK Estate to be focused around widget adapters and glib Type bucketing.
- Added should_flow and should_continue functions to the diy module for conversions between the ControlFlow and bool types.
- Made libadwaita (adw) an optional dependency.
- Implemented feature flags for the gtk4 and adw dependences.
- Added AdwApplcationWindowState, AdwWindowState and GtkWindowState.
- Removed some old enums (gtk_enums.rs and adw_enums.rs).
- Fixed crate homepage URL.
- Added a FUNDING.yml file, specifying my GitHub account.
- Added scs_set_app and scs_add macros for quickly setting and adding Application and Widget state.
- Added a set_margin_sides_and_bottom helper function.
- Updated the README.md.
- Added “#![doc = include_str!("../README.md")]” to the top of lib.rs.
- Added documentation to the objects defined in state_containers.rs.
- set_state_containers will now only set the provided state containers object if the static STATE_CONTAINERS object is invalid.
- Added a CHAGELOG.md file.

## Version 0.1.0 (21/07/2023)

- Inital release







