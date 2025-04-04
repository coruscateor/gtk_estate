


bc3348627d7259431efbc283581321936d430e75 -



- Renamed LookupApplicationObject to LookUpApplicationObject.

- Removed the corlib::AsAny supertrait from the LookUpApplicationObject, ApplicationStateContainer and WidgetStateContainer traits.

- Added a window_ref method to AdwApplcationWindowState.

- Renamed the adapter method to window, added a window_ref method and implemented the dyn_adapter_ref method of the WidgetStateContainer trait on both AdwWindowState and GtkWindowState.

- The provided type parameters for the to_rc_dyn_wsc, to_rc_dyn_sao and to_rc_dyn_swo functions now must be 'static.

- Added the impl_application_state_container and impl_widget_state_container macros.

- Added dyn_adapter_ref as a required method to the WidgetStateContainer trait.

- Static StateContainers objects are now located in thread local storage.

- Fixed an error in the capacity method of WidgetStateContainers. It now calls the capacity method of the widget_state field instead of calling it recursively.



30a2f172dd49d5e7ed9f61964fbfc9972619c0db -



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



b832c12ec0041ddba75f9d251423b2c9615bb69d -



- Added a thread_local_state default feature and made the inclusion of t…
…hread local StateContainers instances and related functionality dependant on this feature.

- Added state_containers_is_set

- set_state_containers now returns a bool.

- Renamed set_application_state to try_set_application_state.

- Renamed set_application_state_or_panic to set_application_state.

- Added try_get_application_state

- Renamed scs_set_app to scs_set_application_state.



24340bf5e837862f3079e56563b547a80bbba0bb -



Added the RcApplicationAdapter and RcWidgetAdapter types.



895ac21b36a07675b7211e03a16a5071677c7887 -



- Added impl_weak_self_methods
- weak_self_ref has been added to AdwApplcationWindowState, AdwWindowState, GtkWindowState, SimpleTimeOut and TimeOut.

- In SimpleTimeOut and TimeOut weak_self now returns a clone of Weak<Self>.



b20f216e08a82263b78495c5d4100935e9f49ec7 -



- Renamed ApplicationStateContainer to DynApplicationStateContainer and …
…added a new trait called ApplicationStateContainer. The relevant parts elsewhere in the project have been updated to reflect these changes.

- Renamed WidgetStateContainer to DynWidgetStateContainer and added a new trait called WidgetStateContainer. The relevant parts elsewhere in the project have been updated to reflect these changes.

- Renamed AdwApplcationWindowState to AdwApplicationWindowState.

- AdwApplicationWindowState, AdwWindowState and GtkWindowState are now now only added to the thread local StateContainers object if the "thread_local_state" feature is enabled.

- AdwApplicationWindowState, AdwWindowState and GtkWindowState now implement the DynWidgetStateContainer and the new WidgetStateContainer traits.



6895c140f3df2f411f19a483d5761072762b3a98 -



- Made corlib::convert::AsAnyRef a supertrait of LookUpApplicationObject…
…, LookupWidgetObject, DynApplicationStateContainer and DynWidgetStateContainer.

- Implemented corlib::convert::AsAnyRef on ApplicationAdapter, WidgetAdapter and LookUpWidgetAdapter.

- Added AsAnyRef implementations to all impl_application_state_container_traits rules.

- Added AsAnyRef implementations to both impl_widget_state_container_traits rules.

- Added dyn_application_state_ref to StateContainers.




55a0fe3d24dab3f694fba8bfbfd95c27ddfb06ca -



- Added the WidgetContainer trait and the impl_widget_container macro.



431c709cfaefbdf93e127f663a7a594fe886b47d -



- Added the impl_contents_box_ref macro.



7ee8915878dab781619193719c0a2d4d67476ec2 -



- Renamed dyn_application_state_ref to application_state_ref_func.

- Renamed try_get_application_state to try_get_dyn_application_state.



3adb0092c6c6cdfd12b31bb450b2395dbc4fb99c -



- Renamed application_state_ref_func to try_application_state_ref_func i…
…n StateContainers.

- Added application_state_ref_func to StateContainers.

- Discovered that the current method of handling automatic dropping of state-containers does not work.



efd18f92ed92a8d2e89152d1218f4eb9ff3303cf -



- Renamed SimpleTimeOut to TimeOut and mostly completed rewriting it.
- Added TimeOutWithParent



8a3a5ec4b0adae1504be1182edeac4a4361132cd -



- Re-wote TimeOut, added TimeOutRunType.
- Removed TimeOutWithParent



b64752aa5c9fc8f51f1591d583379e786d680304 -



- Rewrote TimeOut to not be Rc centric.



a6aff03e551abdaed58faaaf1097a6cd611b8f66 -



- Added ClearStateContainersOnDrop
- Disabled delayed Widget removal functionality.

- Added remove_by_widget_ref and clear methods to StateContainers.

- Corrected a dependency error in the time_out module.



1a6276de2e70b0dacd598c16954b2586841352f8 -



- Derived Debug on ApplicationAdapter, WidgetAdapter, AdwApplicationWind…
…owState, AdwWindowState, GtkWindowState, TimeOutRunType and WidgetStateContainers.

- Made the DynApplicationStateContainer and DynWidgetStateContainer traits require that std::fmt::Debug be implemented.

- Added buckets_len, buckets_capacity, bucket_len and bucket_capacity on both StateContainers and WidgetStateContainers.

- Conditionally implemented std::fmt::Debug on TimeOut.



5449238975ef1f356323a9e263abe1ca82a90479 -



- DynWidgetStateContainer no longer requires Debug.

- The StateContainers application_state sub-field now requires the provided Rc object contain a dyn Any object instead of a dyn DynApplicationStateContainer object, the relevant methods of StateContainers have been updated.

- Added WidgetUpgradeError, WidgetUpgradeResult, WeakWidgetObject, WeakWidgetAdapter, DynWeakWidgetStateContainer and WeakWidgetStateContainers.

- WidgetStateContainers no longer derives Debug.



50abccc9f53d23c8c51805956f97423438e85a6c -



- Renamed the weak_state_containers module to weak_widget_state_containe…
…rs.



7ece4d5add9a6fe18e4cb44256425fb57bd81937 -



- Commented the clear_state_containers_on_drop module declaration.

- Renamed the old DynWidgetStateContainer trait to DynStrongWidgetStateContainer and updated the project accordingly.

- Disabled WidgetStateContainer

- Adjusted the fields of StateContainers.

- Added widget_state_ref and strong_widget_state_ref methods to StateContainers.

- Disabled the dyn_add,  add, remove_by_rc_by_ptr, WidgetStateContainers RefCell accessor, remove, delayed_removal, remove_by_rc_by_ptr, remove_by_widget_ref, has_widget_state, has_widget_state, find_widget_state, buckets_len, buckets_capacity, bucket_len, bucket_capacity and clear methods.

- Updated the scs_add macro.

- Renamed the widget_state_containers module to strong_widget_state_containers.

- Renamed the old WidgetStateContainers struct to StrongWidgetStateContainers.

- Renamed the WeakWidgetObject trait to WidgetObject, the WeakWidgetAdapter struct to WidgetAdapter, WeakWidgetObject to WidgetObject and updated the relevant parts of the project with these changes.

- Added a new impl_widget_state_container_traits macro.

- Renamed WeakWidgetStateContainers struct to WidgetStateContainers.



7d164b83300b7322a65a9d60e53145e578f278d4 -



- Renamed the weak_widget_state_containers module to widget_state_contai…
…ners.



886be84b1cd10b54c1986aa4fc8aa1cc956fea7d -



- Renamed the adapters module to strong_adapters.

- Renamed the weak_adapters module to adapters.

- Adjusted a field of StateContainers.

- Added StrongGtkWindowState

WIP



b8c308f034761c83b210a1b0a94bd6e06cc01700 -



- Updated AdwApplicationWindowState, AdwWindowState and GtkWindowState t…
…o work with the new WidgetAdapter and WidgetStateContainers objects.

- Added to_rc_dyn_strong_wsc and updated to_rc_dyn_swo to use the StrongWidgetObject trait instead of StoredWidgetObject in its definition.

- Disabled to_rc_dyn_sao

- Disabled the DynApplicationStateContainer and ApplicationStateContainer
 traits and the impl_application_state_container_traits macro.

- Updated the DynStrongWidgetStateContainer trait, the impl_strong_widget_state_container_traits macro to use StrongWidgetObject instead of StoredWidgetObject in its definition.

- impl_strong_widget_state_container_traits now no longer includes WidgetStateContainer trait implementations in its definition.

- Disabled StrongWidgetStateContainers and WidgetStateContainer trait implementations in the impl_strong_widget_state_container_traits macro implementaion.

- In the StateContainers object widget_state now has the type RcWidgetStateContainers and strong_widget_state; StrongWidgetStateContainers.

- The scs_strong_add macro has been added.

- Disabled StoredApplicationObject

- Renamed LookupWidgetObject to StrongWidgetObject.

- Disabled LookUpApplicationObject

- Disabled StoredApplicationObject

- Disabled RcApplicationAdapter

- Disabled ApplicationAdapter

- Disabled ApplicationAdapter

- Disabled LookUpWidgetAdapter

- Renamed the old WidgetAdapter object to StrongWidgetAdapter and it now uses DynStrongWidgetStateContainer instead of DynWidgetStateContainer in its struct and impl definitions.

- Added StrongAdwApplicationWindowState

- Added StrongAdwWindowState

- Finished updating StrongGtkWindowState.

- Made StrongWidgetStateContainers internally mutable, renamed the original add method to dyn_add and added find_widget_state and add methods.

- Made WidgetStateContainers internally mutable, a reference type, work within the new weak-widget paradigm and renamed the original add method to dyn_add and added find_widget_state and add methods.

- The library now complies again.



b1534f5ebbf2aeaeb84b08551b4156bb895f59ce -



- The gtk4 dependency is now referred to as gtk and the project has been…
… updated to reflect this change.

- Renamed all features the started with “gtk” so that they now start with “gtk4”.

- Fixed a bunch of adw out of place imports.

- Added a bunch of code only to find I didn’t actually need it.

- Made the inclusion of StrongAdwApplicationWindowState and StrongAdwWindowState dependant on the adw feature.

- StrongWidgetStateContainers and WidgetStateContainers each no longer require a weak_parent be provided on initialisation.

- Optimised how the dyn_add, dyn_find_state, dyn_has_state methods of WidgetStateContainers obtain widgets internally.

- The on_destroy method of WidgetStateContainers now takes a Widget instead of trying to obtain one internally.

- Other minor changes.



4c9f816c070157e0fb68a1ed541328cfc366a3b7 -



- Added the strong_widget_state feature.

- Commented impl_weak_self_methods usage instances thought the library.

- Everything that begins with “Strong” or includes an object that does is now only included when the strong_widget_state freature is active.



568c798d517dd53cbdc3e8c48490c9552d1f97a6 -



- Updated AdwApplicationWindowState and AdwWindowState to work with the …
…latest version of WidgetAdapter.

- Added the to_rc_dyn_any function.

- Moved DynStrongWidgetStateContainer into the strong_widget_state_containers module.

- Re-enabled WidgetStateContainer and its usage instances.

- Moved impl_strong_widget_state_container_traits into the strong_widget_state_containers module.

- Added StrongWidgetStateContainers and added meta-implementations to the rules of the  impl_strong_widget_state_container_traits macro.

- The try_set_application_state and set_application_state methods of StateContainers are now generic.

- Updated StrongAdwApplicationWindowState and StrongAdwWindowState.



0f7df212920b051cd8c48da18a1cd39ad1d7d6e7 -



- WidgetUpgradeError::new now requires a glib::Type. This change has bee…
…n reflected elsewhere in the library.

- Added a static_type_ref method to WidgetUpgradeError.

- Removed AdwApplicationWindowState, AdwWindowState, ClearStateContainersOnDrop, GtkWindowState, StrongAdwApplicationWindowState, StrongAdwWindowState and StrongGtkWindowState.

- Removed some old code.

- Added corlib::weak_self::WeakSelf meta-implementations to all the rules of both impl_strong_widget_state_container_traits and impl_widget_state_container_traits.

- Fixed an issue where WidgetStateContainers wasn’t reacting to widget destruction correctly.

- Added on_widget_upgrade_error, on_widget_upgrade_error_with_param, widget_upgrade_error_display_println, widget_upgrade_error_debug_println, widget_upgrade_error_display_panic and widget_upgrade_error_display_panic functions.



2adc2a9f2cf507e2cf7c6b981c097e5eb8fee201 -



- Updated RcByPtr import statements throughout the project.



787261790a3f2fdb963263887d492d4327b30f96 -



- Changed the library version to be "0.4.0-beta".

- Updated the Corlib dependency to "0.4.0”.

- Updated the gtk dependency version to "0.9.6".

- Wrote some documentation.

- Removed the template directory.



4daf6e40712ce3397cfac5a250cc1f301af88639 -



- Updated the corlib dependency to version 0.4.1.

- In the package.metadata.docs.rs section of the Cargo.toml the features configuration flag has been set to ["strong_widget_state"].

- Started removing cfg_if blocks.



482317bd55c6feb4d71e0ab55d51f7ea282fcf3e -



Added and updated a bunch of documentation.



cad8a7c7846ae5852bfbe07b33ff339a92f85845 -



- Added more documentation

- In the rc_conversions module, the value parameters of the to_rc_dyn_wsc, to_rc_dyn_strong_wsc,
to_rc_dyn_swo and the to_rc_dyn_any functions now take references instead of values and now clone the referenced values internally as well. The project has been updated to reflect these changes.

- In TimeOut, the time_out_fn parameters are now passed by reference and cloned.



8a784dfb45a465b963ece0b6efbfe7ac0b7ea77d -



- Updated the readme.

- Added the adw_v1_1 feature.



faa23bb314315563e59cf4da8b588736390716b5 -



- Updated the libadwaita dependency version to 0.7.2.

- Added the time crate as a development dependency.

- Started work on the changelog notes.

- Further edited the readme and updated other documentation.







