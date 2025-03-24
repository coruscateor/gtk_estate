


bc3348627d7259431efbc283581321936d430e75



- Renamed LookupApplicationObject to LookUpApplicationObject.
- Removed the corlib::AsAny supertrait from the LookUpApplicationObject, ApplicationStateContainer and WidgetStateContainer traits.

- Added a window_ref method to AdwApplcationWindowState.

- Renamed the adapter method to window, added a window_ref method and implemented the dyn_adapter_ref method of the WidgetStateContainer trait on both AdwWindowState and GtkWindowState.

- The provided type parameters for the to_rc_dyn_wsc, to_rc_dyn_sao and to_rc_dyn_swo functions now must be 'static.

- Added the impl_application_state_container and impl_widget_state_container macros.

- Added dyn_adapter_ref as a required method to the WidgetStateContainer trait.

- Static StateContainers objects are now located in thread local storage.

- Fixed an error in the capacity method of WidgetStateContainers. It now calls the capacity method of the widget_state field instead of calling it recursively.



30a2f172dd49d5e7ed9f61964fbfc9972619c0db



- Updated dependencies and dependency related features.
- Added widget_ref to LookupWidgetObject.

- Added application_ref, weak_parent_ref, weak_self  and weak_self_ref to ApplicationAdapter.

- In ApplicationAdapter the application method now returns a clone of the contained application object.

- In WidgetAdapter the widget method now returns a clone of the contained widget object.

- Added widget_ref methods, weak_parent_ref, weak_self and weak_self_ref to WidgetAdapter.

- In LookUpWidgetAdapter the widget method now returns a clone of the contained widget object.

- Added widget_ref methods to LookUpWidgetAdapter.

- Updated the method names of the WidgetStateContainer implementations of AdwApplcationWindowState, AdwWindowState and GtkWindowState

- In the ApplicationStateContainer trait dyn_adapter has been renamed to dyn_application_adapter_ref and dyn_adapter_ref has been renamed to dyn_application_adapter_ref. These changes have been reflected elsewhere in the library.

- In the WidgetStateContainer trait dyn_adapter has been renamed to dyn_widget_adapter_ref and dyn_adapter_ref has been renamed to dyn_widget_adapter_ref. These changes have been reflected elsewhere in the library.



b832c12ec0041ddba75f9d251423b2c9615bb69d



- Added a thread_local_state default feature and made the inclusion of t…
…hread local StateContainers instances and related functionality dependant on this feature.

- Added state_containers_is_set

- set_state_containers now returns a bool.

- Renamed set_application_state to try_set_application_state.

- Renamed set_application_state_or_panic to set_application_state.

- Added try_get_application_state

- Renamed scs_set_app to scs_set_application_state.



24340bf5e837862f3079e56563b547a80bbba0bb



Added the RcApplicationAdapter and RcWidgetAdapter types.



895ac21b36a07675b7211e03a16a5071677c7887



- Added impl_weak_self_methods
- weak_self_ref has been added to AdwApplcationWindowState, AdwWindowState, GtkWindowState, SimpleTimeOut and TimeOut.

- In SimpleTimeOut and TimeOut weak_self now returns a clone of Weak<Self>.






