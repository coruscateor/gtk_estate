# Changelog

## Version 0.4.0 (__/04/2025)

Added

-- Added dyn_adapter_ref as a required method to the WidgetStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynWidgetStateContainer) trait.

-- Added the impl_application_state_container (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700) and impl_widget_state_container macros.

-- Added a window_ref method to AdwApplcationWindowState (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - AdwApplicationWindowState - Removed).

-- Added widget_ref to LookupWidgetObject (disabled?).

-- Added application_ref, weak_parent_ref, weak_self  and weak_self_ref to ApplicationAdapter (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700).

- Added widget_ref methods, weak_parent_ref, weak_self and weak_self_ref to WidgetAdapter.

b832c12ec0041ddba75f9d251423b2c9615bb69d

- Added a thread_local_state default feature and made the inclusion of thread local StateContainers instances and related functionality dependant on this feature.

-- Added state_containers_is_set (disabled)

-- Added try_get_application_state (Renamed 7ee8915878dab781619193719c0a2d4d67476ec2)

24340bf5e837862f3079e56563b547a80bbba0bb

-- Added the RcApplicationAdapter and RcWidgetAdapter types. (Both disabled)

895ac21b36a07675b7211e03a16a5071677c7887

- Added impl_weak_self_methods

-- weak_self_ref has been added to AdwApplcationWindowState (0f7df212920b051cd8c48da18a1cd39ad1d7d6e7 - Removed)), AdwWindowState (ditto), GtkWindowState (ditto), -SimpleTimeOut and TimeOut.

/*
-- Added TimeOutWithParent (Removed - 8a3a5ec4b0adae1504be1182edeac4a4361132cd)
*/

6895c140f3df2f411f19a483d5761072762b3a98

-- Added AsAnyRef implementations to all impl_application_state_container_traits (Disabled) rules.

- Added AsAnyRef implementations to both impl_widget_state_container_traits rules.

-- Added dyn_application_state_ref (Renamed to try_application_state_ref_func 7ee8915878dab781619193719c -> 3adb0092c6c6cdfd12b31bb450b2395dbc4fb99c) to StateContainers.

55a0fe3d24dab3f694fba8bfbfd95c27ddfb06ca

- Added the WidgetContainer trait and the impl_widget_container macro.

431c709cfaefbdf93e127f663a7a594fe886b47d -

- Added the impl_contents_box_ref macro.

3adb0092c6c6cdfd12b31bb450b2395dbc4fb99c

- Added application_state_ref_func to StateContainers.

8a3a5ec4b0adae1504be1182edeac4a4361132cd

- Added TimeOutRunType

a6aff03e551abdaed58faaaf1097a6cd611b8f66

-- Added ClearStateContainersOnDrop (Then removed)

- Added remove_by_widget_ref and clear methods to StateContainers.


Changed


- Static StateContainers objects are now located in thread local storage.

-- Renamed LookupApplicationObject to LookUpApplicationObject. (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700)

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

- set_state_containers now returns a bool.

- Renamed set_application_state to try_set_application_state.

- Renamed set_application_state_or_panic to set_application_state.

- Renamed scs_set_app to scs_set_application_state.

895ac21b36a07675b7211e03a16a5071677c7887

-- In -SimpleTimeOut (Renamed - efd18f92ed92a8d2e89152d1218f4eb9ff3303cf) and TimeOut weak_self now returns a clone of Weak<Self>.

b20f216e08a82263b78495c5d4100935e9f49ec7

- Renamed the original ApplicationStateContainer to DynApplicationStateContainer and added a new trait called ApplicationStateContainer. The relevant parts elsewhere in the project have been updated to reflect these changes.

- Renamed the original WidgetStateContainer to DynWidgetStateContainer and added a new trait called WidgetStateContainer. The relevant parts elsewhere in the project have been updated to reflect these changes.

-- Renamed AdwApplcationWindowState to AdwApplicationWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7).

-- AdwApplicationWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7), AdwWindowState (ditto) and GtkWindowState (ditto) are now now only added to the thread local StateContainers object if the "thread_local_state" feature is enabled.

-- AdwApplicationWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7), AdwWindowState (ditto) and GtkWindowState (ditto) now implement the DynWidgetStateContainer and the new WidgetStateContainer traits.

6895c140f3df2f411f19a483d5761072762b3a98

-- Made corlib::convert::AsAnyRef a supertrait of LookUpApplicationObject (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700), LookupWidgetObject (Renamed to StrongWidgetObject - b8c308f034761c83b210a1b0a94bd6e06cc01700), DynApplicationStateContainer (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700) and DynWidgetStateContainer.

-- Implemented corlib::convert::AsAnyRef on ApplicationAdapter (Removed), WidgetAdapter and LookUpWidgetAdapter (Disabled).

7ee8915878dab781619193719c0a2d4d67476ec2

-- Renamed dyn_application_state_ref to application_state_ref_func (Renamed to try_application_state_ref_func - 3adb0092c6c6cdfd12b31bb450b2395dbc4fb99c).

- Renamed try_get_application_state to try_get_dyn_application_state.

3adb0092c6c6cdfd12b31bb450b2395dbc4fb99c

- Renamed application_state_ref_func to try_application_state_ref_func in StateContainers.

- Added a new application_state_ref_func method to StateContainers.

-- Discovered that the current method of handling automatic dropping of state-containers does not work.

efd18f92ed92a8d2e89152d1218f4eb9ff3303cf

-- Renamed SimpleTimeOut to TimeOut and mostly completed rewriting it.

-- Added TimeOutWithParent (Removed - 8a3a5ec4b0adae1504be1182edeac4a4361132cd)

8a3a5ec4b0adae1504be1182edeac4a4361132cd

-- Re-wote TimeOut (For Added) /*, added TimeOutRunType */.

b64752aa5c9fc8f51f1591d583379e786d680304

-- Rewrote TimeOut to not be Rc centric. (Redundant - 8a3a5ec4b0adae1504be1182edeac4a4361132cd)

a6aff03e551abdaed58faaaf1097a6cd611b8f66

- Disabled delayed Widget removal functionality.


Fixed 

- Fixed an error in the capacity method of WidgetStateContainers. It now calls the capacity method of the widget_state field instead of calling it recursively.

a6aff03e551abdaed58faaaf1097a6cd611b8f66

- Corrected a dependency error in the time_out module.



Removed 

-- Removed the corlib::AsAny supertrait from the LookUpApplicationObject (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700), ApplicationStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynApplicationStateContainer) and WidgetStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynWidgetStateContainer) traits.

--

-- Removed AdwApplcationWindowState (0f7df212920b051cd8c48da18a1cd39ad1d7d6e7)

--

8a3a5ec4b0adae1504be1182edeac4a4361132cd

-- Removed TimeOutWithParent (Added - efd18f92ed92a8d2e89152d1218f4eb9ff3303cf)



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







