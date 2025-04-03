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

1a6276de2e70b0dacd598c16954b2586841352f8

-- Added buckets_len, buckets_capacity, bucket_len and bucket_capacity on both StateContainers (Disabled) and WidgetStateContainers. (Disabled for StateContainers)

5449238975ef1f356323a9e263abe1ca82a90479

-- Added WidgetUpgradeError, WidgetUpgradeResult, WeakWidgetObject (Renamed to WidgetObject - 7ece4d5add9a6fe18e4cb44256425fb57bd81937), WeakWidgetAdapter (Renamed to WidgetAdapter - 7ece4d5add9a6fe18e4cb44256425fb57bd81937), DynWeakWidgetStateContainer (Can't find) and WeakWidgetStateContainers (Renamed to WidgetStateContainers - 7ece4d5add9a6fe18e4cb44256425fb57bd81937).

886be84b1cd10b54c1986aa4fc8aa1cc956fea7d

-- Added StrongGtkWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7)

b8c308f034761c83b210a1b0a94bd6e06cc01700

-- Added to_rc_dyn_strong_wsc and updated to_rc_dyn_swo to use the StrongWidgetObject trait instead of StoredWidgetObject in its definition. (2 categories)

- Added to_rc_dyn_strong_wsc to the rc_conversions module.

(New)

- Added the impl_strong_widget_state_container_traits macro.

- Added the DynStrongWidgetStateContainer trait.



- The scs_strong_add macro has been added.

-- Added StrongAdwApplicationWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7)

-- Added StrongAdwWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7)

b1534f5ebbf2aeaeb84b08551b4156bb895f59ce

-- Added a bunch of code only to find I didn’t actually need it. (Yeah ok)

4c9f816c070157e0fb68a1ed541328cfc366a3b7

- Added the strong_widget_state feature.

- Added the to_rc_dyn_any function.

568c798d517dd53cbdc3e8c48490c9552d1f97a6

- Added StrongWidgetStateContainers and added meta-implementations to the rules of the impl_strong_widget_state_container_traits macro.

0f7df212920b051cd8c48da18a1cd39ad1d7d6e7

- Added a static_type_ref method to WidgetUpgradeError. (Irrelevant - Added in this version)

0f7df212920b051cd8c48da18a1cd39ad1d7d6e7

-- Added corlib::weak_self::WeakSelf meta-implementations to all the rules of both impl_strong_widget_state_container_traits (Irrelevant - Added in this version) and impl_widget_state_container_traits. (Irrelevant - Added in this version)

(fixed)

- Added on_widget_upgrade_error, on_widget_upgrade_error_with_param, widget_upgrade_error_display_println, widget_upgrade_error_debug_println, widget_upgrade_error_display_panic and widget_upgrade_error_debug_panic functions.



Was missed:

7ece4d5add9a6fe18e4cb44256425fb57bd81937

- Added widget_state_ref and strong_widget_state_ref methods to StateContainers

(From point 10)

- Added WidgetObject

- Replaced the old WidgetAdapter object with a new one.



- Added a new impl_widget_state_container_traits macro.



(From point 12)

- Added a new WidgetStateContainers struct.



787261790a3f2fdb963263887d492d4327b30f96

(From point 4)

-- Added Documentation (Remove)



482317bd55c6feb4d71e0ab55d51f7ea282fcf3e

- Added and updated a bunch of documentation.


cad8a7c7846ae5852bfbe07b33ff339a92f85845

-- Added more documentation (Remove)

8a784dfb45a465b963ece0b6efbfe7ac0b7ea77d

- Added the adw_v1_1 feature.

faa23bb314315563e59cf4da8b588736390716b5

- Added the time crate as a development dependency.



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

-- Implemented corlib::convert::AsAnyRef on ApplicationAdapter (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700), WidgetAdapter and LookUpWidgetAdapter (ditto).

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

1a6276de2e70b0dacd598c16954b2586841352f8

-- Derived Debug on ApplicationAdapter (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700), WidgetAdapter, AdwApplicationWindowState (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7), AdwWindowState (ditto), GtkWindowState (ditto), TimeOutRunType and WidgetStateContainers (Irrelevant - 5449238975ef1f356323a9e263abe1ca82a90479).

-- Made the DynApplicationStateContainer (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700) and DynWidgetStateContainer traits require that std::fmt::Debug be implemented.

-- Conditionally implemented std::fmt::Debug on TimeOut. (Added in this version.)

5449238975ef1f356323a9e263abe1ca82a90479

-- DynWidgetStateContainer no longer requires Debug. (Incorrect)

- The StateContainers application_state sub-field now requires the provided Rc object contain a dyn Any object instead of a dyn DynApplicationStateContainer object, the relevant methods of StateContainers have been updated.

- WidgetStateContainers no longer derives Debug. (Irrelevant - 1a6276de2e70b0dacd598c16954b2586841352f8)

50abccc9f53d23c8c51805956f97423438e85a6c

-- Renamed the weak_state_containers module to weak_widget_state_containers. (Irrelevant/incorrect)

- Renamed the weak_state_containers module to widget_state_containers. 

886be84b1cd10b54c1986aa4fc8aa1cc956fea7d

-- Renamed the adapters module to strong_adapters.

- Renamed the orignal adapters module to strong_adapters.

-- Renamed the weak_adapters module to adapters. (Irrelevant - added in this version)

-- Adjusted a field of StateContainers. (Not important)

b8c308f034761c83b210a1b0a94bd6e06cc01700

-- Updated AdwApplicationWindowState (Removed), AdwWindowState (Removed) and GtkWindowState (Removed) to work with the new WidgetAdapter and WidgetStateContainers objects.

-- Added to_rc_dyn_strong_wsc and updated to_rc_dyn_swo to use the StrongWidgetObject trait instead of StoredWidgetObject in its definition. (2 categories)

- Updated to_rc_dyn_swo to use the StrongWidgetObject trait instead of StoredWidgetObject in its definition.

- Disabled to_rc_dyn_sao

-- Disabled the DynApplicationStateContainer and ApplicationStateContainer
 traits and the impl_application_state_container_traits macro. (Disabled)

-- Updated the DynStrongWidgetStateContainer trait, the impl_strong_widget_state_container_traits macro to use StrongWidgetObject instead of StoredWidgetObject in its definition.

-- Updated the DynStrongWidgetStateContainer trait the to use StrongWidgetObject instead of StoredWidgetObject in their definitions. (Added)

-- impl_strong_widget_state_container_traits macro (Added)

- Disabled to_rc_dyn_sao

-- Disabled the DynApplicationStateContainer and ApplicationStateContainer
 traits and the impl_application_state_container_traits macro.

- Disabled ApplicationStateContainer

-- Updated the DynStrongWidgetStateContainer trait, the impl_strong_widget_state_container_traits macro to use StrongWidgetObject instead of StoredWidgetObject in its definition. (Irrelevant - new)

-- impl_strong_widget_state_container_traits now no longer includes WidgetStateContainer trait implementations in its definition. (Irrelevant - new)

-- impl_strong_widget_state_container_traits now no longer includes WidgetStateContainer trait implementations in its definition. (Irrelevant - new)

-- Disabled StrongWidgetStateContainers and WidgetStateContainer trait implementations in the impl_strong_widget_state_container_traits macro implementaion. (Irrelevant - new)

- In the StateContainers object widget_state now has the type RcWidgetStateContainers and strong_widget_state; StrongWidgetStateContainers.

-- Disabled RcApplicationAdapter (Irrelevant)

- Disabled ApplicationAdapter

-- Disabled LookUpWidgetAdapter (Irrelevant)

- Renamed the old WidgetAdapter object to StrongWidgetAdapter and it now uses DynStrongWidgetStateContainer instead of DynWidgetStateContainer in its struct and impl definitions.

- Disabled StoredApplicationObject

- Renamed LookupWidgetObject to StrongWidgetObject.

-- Disabled StoredApplicationObject (Duplicate)

- Disabled RcApplicationAdapter

-- Finished updating StrongGtkWindowState. (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7)

-- Made StrongWidgetStateContainers internally mutable, renamed the original add method to dyn_add and added find_widget_state and add methods. (Irrelevant - added this version)

- Made WidgetStateContainers internally mutable, a reference type, work within the new weak-widget paradigm and renamed the original add method to dyn_add and added find_widget_state and add methods.

-- The library now complies again. (Yay!)

b1534f5ebbf2aeaeb84b08551b4156bb895f59ce

- The gtk4 dependency is now referred to as gtk and the project has been updated to reflect this change.

- Renamed all features the started with “gtk” so that they now start with “gtk4”.

- Fixed a bunch of adw out of place imports.

- Made the inclusion of StrongAdwApplicationWindowState and StrongAdwWindowState dependant on the adw feature. (Removed - 0f7df212920b051cd8c48da18a1cd39ad1d7d6e7
)

-- StrongWidgetStateContainers (New in this version) and WidgetStateContainers each no longer require a weak_parent be provided on initialisation.

- Optimised how the dyn_add, dyn_find_state, dyn_has_state methods of WidgetStateContainers obtain widgets internally.

- The on_destroy method of WidgetStateContainers now takes a Widget instead of trying to obtain one internally.

-- Other minor changes. (Etc...)

4c9f816c070157e0fb68a1ed541328cfc366a3b7

-- Commented impl_weak_self_methods usage instances thought the library. (Irrelevant)

- Everything that begins with “Strong” or includes an object that does is now only included when the strong_widget_state freature is active.

568c798d517dd53cbdc3e8c48490c9552d1f97a6

-- Updated AdwApplicationWindowState and AdwWindowState to work with the latest version of WidgetAdapter. (Irrelevant)

-- Moved DynStrongWidgetStateContainer into the strong_widget_state_containers module. (Somewhat relevant - Renamed - 7ece4d5add9a6fe18e4cb44256425fb57bd81937)

- Re-enabled WidgetStateContainer and its usage instances. (Irrelevant)

-- Moved impl_strong_widget_state_container_traits into the strong_widget_state_containers module.  (Irrelevant - new)

- The try_set_application_state and set_application_state methods of StateContainers are now generic.

- Updated StrongAdwApplicationWindowState and StrongAdwWindowState. (Irrelevant - Removed)

0f7df212920b051cd8c48da18a1cd39ad1d7d6e7

-- WidgetUpgradeError::new now requires a glib::Type. This change has been reflected elsewhere in the library. (Irrelevant - Added in this version)

Was missed:

7ece4d5add9a6fe18e4cb44256425fb57bd81937

-- Commented the clear_state_containers_on_drop module declaration. (Irrelevant - Removed)

-- Renamed the old DynWidgetStateContainer trait to DynStrongWidgetStateContainer and updated the project accordingly. (Combine with below)

-- Moved DynStrongWidgetStateContainer into the strong_widget_state_containers module. (From 568c798d517dd53cbdc3e8c48490c9552d1f97a6)

-- Disabled WidgetStateContainer (Reenabled - 568c798d517dd53cbdc3e8c48490c9552d1f97a6)

- Adjusted the fields of StateContainers.

-- Disabled the dyn_add,  add, remove_by_rc_by_ptr, WidgetStateContainers RefCell accessor, remove, delayed_removal, remove_by_rc_by_ptr, remove_by_widget_ref, has_widget_state, has_widget_state, find_widget_state, buckets_len, buckets_capacity, bucket_len, bucket_capacity and clear methods. (In the StateContainers implementation.) (The clear method as added at some point) (Rewitten below)

- Disabled the dyn_add,  add, remove_by_rc_by_ptr, WidgetStateContainers RefCell accessor, remove, delayed_removal, remove_by_widget_ref, has_widget_state and the find_widget_state methods of StateContainers. 

- Updated the scs_add macro.

- Renamed the widget_state_containers module to strong_widget_state_containers.

- Renamed the old WidgetStateContainers struct to StrongWidgetStateContainers.

-- Renamed the WeakWidgetObject trait to WidgetObject, the WeakWidgetAdapter struct to WidgetAdapter, WeakWidgetObject to WidgetObject and updated the relevant parts of the project with these changes. (Dealt with in the added section)

-- Renamed WeakWidgetStateContainers struct to WidgetStateContainers. (Dealt with in the added section)

2adc2a9f2cf507e2cf7c6b981c097e5eb8fee201

- Updated RcByPtr import statements throughout the project.

787261790a3f2fdb963263887d492d4327b30f96

-- Changed the library version to be "0.4.0-beta". (Will be changed again)

-- Updated the Corlib dependency to "0.4.0”. (Irrelevant)

- Updated the gtk dependency version to "0.9.6".

-- Wrote some documentation. (Added Documentation?)

4daf6e40712ce3397cfac5a250cc1f301af88639

- Updated the corlib dependency to version 0.4.1.

-- In the package.metadata.docs.rs section of the Cargo.toml the features configuration flag has been set to ["strong_widget_state"]. (Adjusted)

- In the package.metadata.docs.rs section of the Cargo.toml the features configuration flag has been set to [\"strong_widget_state\"].

cad8a7c7846ae5852bfbe07b33ff339a92f85845

-- In the rc_conversions module, the value parameters of the to_rc_dyn_wsc, to_rc_dyn_strong_wsc (New in this version),
to_rc_dyn_swo and the to_rc_dyn_any (Ditto) functions now take references instead of values and now clone the referenced values internally as well. The project has been updated to reflect these changes.

-- In TimeOut, the time_out_fn parameters are now passed by reference and cloned. (Remove)

8a784dfb45a465b963ece0b6efbfe7ac0b7ea77d

- Updated the readme.

faa23bb314315563e59cf4da8b588736390716b5

- Updated the libadwaita dependency version to 0.7.2.

-- Started work on the changelog notes. (Remove)

-- Further edited the readme and updated other documentation. (Remove)



Fixed

- Fixed an error in the capacity method of WidgetStateContainers. It now calls the capacity method of the widget_state field instead of calling it recursively.

a6aff03e551abdaed58faaaf1097a6cd611b8f66

- Corrected a dependency error in the time_out module.

0f7df212920b051cd8c48da18a1cd39ad1d7d6e7

-- Fixed an issue where WidgetStateContainers wasn’t reacting to widget destruction correctly. (Elabborate)



Removed 

-- Removed the corlib::AsAny supertrait from the LookUpApplicationObject (Disabled - b8c308f034761c83b210a1b0a94bd6e06cc01700), ApplicationStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynApplicationStateContainer) and WidgetStateContainer (Renamed - b20f216e08a82263b78495c5d4100935e9f49ec7 - DynWidgetStateContainer) traits.

--

-- Removed AdwApplcationWindowState (0f7df212920b051cd8c48da18a1cd39ad1d7d6e7)

--

8a3a5ec4b0adae1504be1182edeac4a4361132cd

-- Removed TimeOutWithParent (Added - efd18f92ed92a8d2e89152d1218f4eb9ff3303cf)

0f7df212920b051cd8c48da18a1cd39ad1d7d6e7

-- Removed AdwApplicationWindowState, AdwWindowState, ClearStateContainersOnDrop (Irrelevant - Added in this version), GtkWindowState, StrongAdwApplicationWindowState (ditto), StrongAdwWindowState (ditto) and StrongGtkWindowState. (ditto)

-- Removed some old code. (Ok)

787261790a3f2fdb963263887d492d4327b30f96

- Removed the template directory.

4daf6e40712ce3397cfac5a250cc1f301af88639

-- Started removing cfg_if blocks. (Re-written)

- Removed some cfg_if blocks in the project lib file.



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







