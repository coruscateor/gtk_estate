# Changelog

## Version 0.3.0 (_/05/2024)



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







