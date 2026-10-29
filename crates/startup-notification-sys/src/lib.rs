//! Raw startup-notification bindings.
//! Layouts and signatures preserve the existing Linux x86_64 native backend.
#![allow(non_camel_case_types, non_upper_case_globals)]
use xcb_sys::{xcb_connection_t, xcb_generic_event_t};
pub const SN_MONITOR_EVENT_CANCELED: SnMonitorEventType = 3;
pub const SN_MONITOR_EVENT_CHANGED: SnMonitorEventType = 2;
pub const SN_MONITOR_EVENT_COMPLETED: SnMonitorEventType = 1;
pub const SN_MONITOR_EVENT_INITIATED: SnMonitorEventType = 0;
#[repr(C)]
pub struct SnDisplay {
    _private: [u8; 0],
}
pub type SnFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[repr(C)]
pub struct SnLauncheeContext {
    _private: [u8; 0],
}
#[repr(C)]
pub struct SnLauncherContext {
    _private: [u8; 0],
}
#[repr(C)]
pub struct SnMonitorContext {
    _private: [u8; 0],
}
#[repr(C)]
pub struct SnMonitorEvent {
    _private: [u8; 0],
}
pub type SnMonitorEventFunc =
    Option<unsafe extern "C" fn(*mut SnMonitorEvent, *mut ::core::ffi::c_void) -> ()>;
pub type SnMonitorEventType = ::core::ffi::c_uint;
#[repr(C)]
pub struct SnStartupSequence {
    _private: [u8; 0],
}
pub type SnXcbDisplayErrorTrapPop =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
pub type SnXcbDisplayErrorTrapPush =
    Option<unsafe extern "C" fn(*mut SnDisplay, *mut xcb_connection_t) -> ()>;
pub type sn_bool_t = ::core::ffi::c_int;
unsafe extern "C" {
    pub fn sn_display_unref(display: *mut SnDisplay);
    pub fn sn_launchee_context_complete(context: *mut SnLauncheeContext);
    pub fn sn_launchee_context_new_from_environment(
        display: *mut SnDisplay,
        screen: ::core::ffi::c_int,
    ) -> *mut SnLauncheeContext;
    pub fn sn_launchee_context_setup_window(
        context: *mut SnLauncheeContext,
        xwindow: ::core::ffi::c_ulong,
    );
    pub fn sn_launchee_context_unref(context: *mut SnLauncheeContext);
    pub fn sn_launcher_context_complete(context: *mut SnLauncherContext);
    pub fn sn_launcher_context_get_startup_id(
        context: *mut SnLauncherContext,
    ) -> *const ::core::ffi::c_char;
    pub fn sn_launcher_context_initiate(
        context: *mut SnLauncherContext,
        launcher_name: *const ::core::ffi::c_char,
        launchee_name: *const ::core::ffi::c_char,
        timestamp: ::core::ffi::c_ulong,
    );
    pub fn sn_launcher_context_new(
        display: *mut SnDisplay,
        screen: ::core::ffi::c_int,
    ) -> *mut SnLauncherContext;
    pub fn sn_launcher_context_ref(context: *mut SnLauncherContext);
    pub fn sn_launcher_context_set_description(
        context: *mut SnLauncherContext,
        description: *const ::core::ffi::c_char,
    );
    pub fn sn_launcher_context_set_name(
        context: *mut SnLauncherContext,
        name: *const ::core::ffi::c_char,
    );
    pub fn sn_launcher_context_setup_child_process(context: *mut SnLauncherContext);
    pub fn sn_launcher_context_unref(context: *mut SnLauncherContext);
    pub fn sn_monitor_context_new(
        display: *mut SnDisplay,
        screen: ::core::ffi::c_int,
        event_func: SnMonitorEventFunc,
        event_func_data: *mut ::core::ffi::c_void,
        free_data_func: SnFreeFunc,
    ) -> *mut SnMonitorContext;
    pub fn sn_monitor_event_get_startup_sequence(
        event: *mut SnMonitorEvent,
    ) -> *mut SnStartupSequence;
    pub fn sn_monitor_event_get_type(event: *mut SnMonitorEvent) -> SnMonitorEventType;
    pub fn sn_startup_sequence_get_id(
        sequence: *mut SnStartupSequence,
    ) -> *const ::core::ffi::c_char;
    pub fn sn_xcb_display_new(
        xconnection: *mut xcb_connection_t,
        push_trap_func: SnXcbDisplayErrorTrapPush,
        pop_trap_func: SnXcbDisplayErrorTrapPop,
    ) -> *mut SnDisplay;
    pub fn sn_xcb_display_process_event(
        display: *mut SnDisplay,
        xevent: *mut xcb_generic_event_t,
    ) -> sn_bool_t;
}

unsafe extern "C" {
    pub fn sn_malloc0(n_bytes: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn sn_realloc(
        mem: *mut ::core::ffi::c_void,
        n_bytes: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    pub fn sn_free(mem: *mut ::core::ffi::c_void);
}
