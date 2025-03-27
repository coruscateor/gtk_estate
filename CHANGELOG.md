# Changelog

## Version 0.4.0 (__/04/2025)

Changed

- Static StateContainers objects are now located in thread local storage.

-- Renamed LookupApplicationObject to LookUpApplicationObject. (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700)



Removed 

-- Removed the corlib::AsAny supertrait from the LookUpApplicationObject (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700), ApplicationStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynApplicationStateContainer) and WidgetStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynWidgetStateContainer) traits.



Added

-- Added dyn_adapter_ref as a required method to the WidgetStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynWidgetStateContainer) trait.

-- Added the impl_application_state_container (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700) and impl_widget_state_container macros.

-- Added a window_ref method to AdwApplcationWindowState (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - AdwApplicationWindowState).

-- Added widget_ref to LookupWidgetObject (disabled?).

-- Added application_ref, weak_parent_ref, weak_self  and weak_self_ref to ApplicationAdapter (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700).

- Added widget_ref methods, weak_parent_ref, weak_self and weak_self_ref to WidgetAdapter.



Changed

- The provided type parameters for the to_rc_dyn_wsc, to_rc_dyn_sao and to_rc_dyn_swo functions now must be 'static.

-- (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7) Renamed the adapter method to window, added a window_ref method and implemented the dyn_adapter_ref method of the WidgetStateContainer trait on both AdwWindowState and GtkWindowState.

- Updated dependencies and dependency related features.

-- In ApplicationAdapter the application method now returns a clone of the contained application object (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700).

- In WidgetAdapter the widget method now returns a clone of the contained widget object.

-- In LookUpWidgetAdapter (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700) the widget method now returns a clone of the contained widget object.

-- Added widget_ref methods to LookUpWidgetAdapter (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700).

-- Updated the method names of the WidgetStateContainer implementations of AdwApplcationWindowState (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - AdwApplicationWindowState), AdwWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7) and GtkWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7) - (Vague).

-- In the ApplicationStateContainer (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700) trait dyn_adapter has been renamed to dyn_application_adapter_ref and dyn_adapter_ref has been renamed to dyn_application_adapter_ref. These changes have been reflected elsewhere in the library.

- In the WidgetStateContainer trait dyn_adapter has been renamed to dyn_widget_adapter_ref and dyn_adapter_ref has been renamed to dyn_widget_adapter_ref. These changes have been reflected elsewhere in the library.

/*
-- AdwApplicationWindowState, AdwWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7) and GtkWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7) are now now only added to the thread local StateContainers object if the "thread_local_state" feature is enabled.
*/



Fixed 

- Fixed an error in the capacity method of WidgetStateContainers. It now calls the capacity method of the widget_state field instead of calling it recursively.



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







