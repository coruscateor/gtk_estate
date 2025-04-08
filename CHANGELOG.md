# Changelog

## Version 0.4.0 (08/04/2025)

Added

- Added the impl_widget_state_container_traits macro.

- Added weak_parent_ref, weak_self and weak_self_ref methods to WidgetAdapter.

- Added a thread_local_state default feature and made the inclusion of thread local StateContainers instances and related functionality dependant on this feature.

- Added the try_application_state_ref_func method to StateContainers.

- Added impl_weak_self_methods

- Added AsAnyRef meta-implementations to both impl_widget_state_container_traits rules.

- Added the WidgetContainer trait and the impl_widget_container macro.

- Added the impl_contents_box_ref macro.

- Added application_state_ref_func to StateContainers.

- Added TimeOutRunType

- Added the WidgetUpgradeError struct, the WidgetUpgradeResult type, the WidgetObject trait and a new WidgetAdapter struct.

- Added to_rc_dyn_strong_wsc to the rc_conversions module.

- Added the impl_strong_widget_state_container_traits macro.

- The scs_strong_add macro has been added.

- Added the strong_widget_state feature.

- Added the to_rc_dyn_any function to the rc_conversions module.

- Added the StrongWidgetStateContainers struct.

- Added on_widget_upgrade_error, on_widget_upgrade_error_with_param, widget_upgrade_error_display_println, widget_upgrade_error_debug_println, widget_upgrade_error_display_panic and widget_upgrade_error_debug_panic functions.

- Added widget_state_ref and strong_widget_state_ref methods to StateContainers.

- Added WidgetObject

- Added a new WidgetAdapter object replacing the old one.

- Added a new impl_widget_state_container_traits macro.

- Added a new WidgetStateContainers struct.

- Added and updated a bunch of documentation.

- Added the adw_v1_1 feature.

- Added the time crate as a development dependency.

- Added a new set_application_state method to StateContainers. 



Changed

- StateContainers objects are now initialised in thread-local storage by default.

- The provided type parameters for the to_rc_dyn_wsc and to_rc_dyn_swo functions now must have 'static lifetimes.

- Updated dependencies and dependency related features.

- set_state_containers now returns a bool.

- Renamed set_application_state to try_set_application_state in StateContainers.

- Renamed set_application_state_or_panic to set_application_state in StateContainers.

- Renamed the scs_set_app macro to scs_set_application_state.

- Renamed the original WidgetStateContainer to DynWidgetStateContainer. The relevant parts of the project have been updated to reflect these changes.

- Renamed SimpleTimeOut to TimeOut and rewrote it.

- Disabled delayed Widget removal functionality in StateContainers.

- The StateContainers application_state sub-field now requires that the provided Rc object contain a dyn Any object instead of a dyn DynApplicationStateContainer object, the relevant methods of StateContainers have been updated.

- Renamed the orignal adapters module to strong_adapters.

- Updated to_rc_dyn_swo to use the StrongWidgetObject trait instead of StoredWidgetObject in its definition.

- Disabled to_rc_dyn_sao

- Disabled ApplicationStateContainer

- Disabled ApplicationAdapter

- Renamed the old WidgetAdapter object to StrongWidgetAdapter and it now uses DynStrongWidgetStateContainer instead of DynWidgetStateContainer in its struct and impl definitions.

- Disabled StoredApplicationObject

- Renamed LookupWidgetObject to StrongWidgetObject and added a widget_ref method declaration to it.

- Disabled RcApplicationAdapter

- Made WidgetStateContainers internally mutable, a reference type, work within the new weak-widget paradigm and renamed the original add method to dyn_add and added find_widget_state and add methods.

- The name of the gtk4 dependency has been changed to gtk and the project has been updated to reflect this change.

- Renamed all the features that started with “gtk” to now start with “gtk4”.

- WidgetStateContainers no longer requires a weak_parent be provided on initialisation.

- Optimised how the dyn_add, dyn_find_state, dyn_has_state methods of WidgetStateContainers obtain widgets internally.

- The on_destroy method of WidgetStateContainers now takes a Widget (In addition to its other parameter) instead of trying to obtain one internally.

- The try_set_application_state method of StateContainers is now generic.

- Renamed the original WidgetStateContainer to DynStrongWidgetStateContainer and moved it into the strong_widget_state_containers module.

- Adjusted the fields of StateContainers.

- Disabled the dyn_add, add, remove_by_rc_by_ptr, WidgetStateContainers RefCell accessor, remove, delayed_removal, remove_by_widget_ref, has_widget_state and the find_widget_state methods of StateContainers. 

- Updated the scs_add macro.

- Renamed the original widget_state_containers module to strong_widget_state_containers.

- Renamed the original WidgetStateContainers struct to StrongWidgetStateContainers.

- Updated RcByPtr import statements throughout the project.

- Updated the gtk dependency version to 0.9.6.

- Updated the corlib dependency to version 0.4.1.

- In the package.metadata.docs.rs section of the Cargo.toml file the features configuration flag has been set to ["strong_widget_state"].

- In the rc_conversions module, the value parameters of the to_rc_dyn_wsc,
to_rc_dyn_swo functions now take references instead of values and now clone the referenced values internally as well. The project has been updated to reflect these changes.

- Updated the readme.

- Updated the libadwaita dependency to version 0.7.2.

- Updated to_rc_dyn_swo to use the StrongWidgetObject trait instead of StoredWidgetObject in its definition. It now also is dependant on the inclusion of the strong_widget_state feature.

- Replaced the old WidgetAdapter object with a new one.



Fixed

- Fixed an error in the capacity method of WidgetStateContainers. It now calls the capacity method of the widget_state field instead of calling it recursively.

- Fixed an issue where WidgetStateContainers wasn’t reacting to widget destruction correctly and dropping Rc'd DynWidgetStateContainer objects as expected.

- Fixed a bunch of adw out of place imports.



Removed 

- Removed AdwApplicationWindowState, AdwWindowState and GtkWindowState.

- Removed the template directory.

- Removed some cfg_if blocks in the project lib file.

- Removed the orignal TimeOut struct



Missed

- Due to the amount of re-naming and movement of objects and functions in the production of this version of GTK Estate, some changes may have been missed.



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







