# Changelog

## Version 0.2.0 (19/04/2024)

- Updated dependencies (gtk4 to "0.8.*" adw to "0.6.*" and corlib to "0.2.0")
- Re-wrote GTK Estate to be focused around widget adapters and glib Type bucketing.

(Remove) - Implemented widget_as_any and connect_destroy in WidgetObject and now contains a reference to parent weak WidgetStateContainer.

(Remove) - Tried to implement a widget hashing container for use in WidgetStateContainers but corlib::RcByPtr will have to do instead.

(Remove) - updated state_containers to work with WidgetStateContainers.

(Remove) - Created widget_state_containers


(Remove) - Updated the gtk4 and libadwaita dependencies to versions 0.6 and 0.4 respectively.

- Added should_flow and should_continue functions to the diy module for conversions between the ControlFlow and bool types.

(Remove) - Started the state_containers rewrite.

(Remove) - Removed the object_container and container_map modules.

(Remove) - Added the adapters module for adapting objects that implement non-object safe gtk4 and libadwaita traits into objects that satisfy Rusts requirements for object safety.


- Made libadwaita (adw) an optional dependency.
(Remove) - Renamed “widget_as_any” to “dyn_widget” in WidgetObject and WidgetAdapter in adapters.rs

(Remove) - Added “dyn_application” for getting a dynamicly dispatched Application object to ApplicationObject and ApplicationAdapter in adapters.rs

(Remove) - Added delayed object removal to StateContainers

(Remove) - WidgetStateContainers now has a “remove_by_rc_by_ptr” which allows you to remove an object directly by RcByPtr.


(Remove) - Implemented the corlib::as_any trait on ApplicationObject and WidgetObject.

(Remove) - Added methods to find widget states in both StateContainer and WidgetStateContainer.

- Implemented feature flags for the gtk4 and adw dependences.
(Remove) - StateContainers is now oriented around glib::types::Type instead of std::any::TypeId

(Remove) - Added the gtk related adw feature flags.
(Remove) - In StateContainers set_application_state now returns bool and a set_application_state_or_panic method has been added.

- Added adw_application_window_state, adw_window_state and gtk_window_state

(Remove) - Implemented fixes etc

(Remove) Tried to get the generic trait bounds setup correctly so that generic container objects can be converted to Widget objects in widget adapter objects. This lead to getting an “error[E0277]: the trait bound `gtk_estate::gtk4::Widget: IsA<gtk_estate::libadwaita::ApplicationWindow>` is not satisfied” error in simple_unix_time_outputer so I’ll have to figure out another solution.

(Remove) Began implementing the ContainerWidgetRef enum to avoid the ugly glib::object::Cast situation.

(Remove) - Found that I should’ve been using upcast_ref instead of downcast_ref in WidgetAdapter and LookupWidgetAdapter which should mean that I no longer need to implement a work around (ContainerWidgetRef) to get the base widget type of each stored widget.

- Removed some old enums (gtk_enums.rs and adw_enums.rs).

(Remove) - Made parents of adapters generic.
(Remove) - Added to_rc_dyn_wsc which converts a Rc<T: WidgetStateContainer> to a Rc<dyn WidgetStateContainer>.

(Remove) - Removed erroneous &dyn Any conversions and downcast_ref calls.


(Remove) - Renamed some methods, de-genericised some structs and moved stuff around.

(Remove) - All methods that returned &dyn StoredWidgetObject now return Rc<dyn StoredWidgetObject>, consequently all adapters are now Rc’d.

(Remove) - Added more constructor variants to AdwApplcationWindowState.

(Remove) - It works!

- Fixed crate homepage URL.

- Added a FUNDING.yml file, specifying my GitHub account.
- Added scs_set_app and scs_add macros for quickly setting and adding Application and Widget state.

- Added a set_margin_sides_and_bottom helper function.

- Updated the README.md.
- Corrected the spelling of “delayed” in adapters.rs and state_containers.rs.

- Added “#![doc = include_str!("../README.md")]” to the top of lib.rs.

- Added documentation to the objects defined in state_containers.rs.

- set_state_containers will now only set the provided state containers object if the static STATE_CONTAINERS object is invalid.

(Remove) Updated the corlib dependency to 0.2.0.

- Added a CHAGELOG.md file.

## Version 0.1.0 (21/07/2023)

- Inital release







