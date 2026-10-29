//! Native libev bindings for the default Linux x86_64 watcher layout.
//!
//! Callbacks deliberately retain the C ABI required by libev. Watchers must
//! remain at stable addresses while active; constructors create inactive values.
#![no_std]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

#[repr(C)]
pub struct ev_loop {
    _private: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_timer {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int) -> ()>,
    pub at: ev_tstamp,
    pub repeat: ev_tstamp,
}

pub type ev_tstamp = ::core::ffi::c_double;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher, ::core::ffi::c_int) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_list {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb:
        Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_list, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_io {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_signal {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub signum: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_prepare {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_watcher_time {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb:
        Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_watcher_time, ::core::ffi::c_int) -> ()>,
    pub at: ev_tstamp,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_child {
    pub active: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub cb: Option<unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int) -> ()>,
    pub next: *mut ev_watcher_list,
    pub flags: ::core::ffi::c_int,
    pub pid: ::core::ffi::c_int,
    pub rpid: ::core::ffi::c_int,
    pub rstatus: ::core::ffi::c_int,
}

pub const EV_ERROR: ::core::ffi::c_int = -2147483648;
pub const EV_CUSTOM: ::core::ffi::c_int = 16777216;
pub const EV_ASYNC: ::core::ffi::c_int = 524288;
pub const EV_CLEANUP: ::core::ffi::c_int = 262144;
pub const EV_FORK: ::core::ffi::c_int = 131072;
pub const EV_EMBED: ::core::ffi::c_int = 65536;
pub const EV_CHECK: ::core::ffi::c_int = 32768;
pub const EV_PREPARE: ::core::ffi::c_int = 16384;
pub const EV_IDLE: ::core::ffi::c_int = 8192;
pub const EV_STAT: ::core::ffi::c_int = 4096;
pub const EV_CHILD: ::core::ffi::c_int = 2048;
pub const EV_SIGNAL: ::core::ffi::c_int = 1024;
pub const EV_PERIODIC: ::core::ffi::c_int = 512;
pub const EV_TIMEOUT: ::core::ffi::c_int = 256;
pub const EV_TIMER: ::core::ffi::c_int = 256;
pub const EV_IO: ::core::ffi::c_int = 1;
pub const EV__IOFDSET: ::core::ffi::c_int = 128;
pub const EV_WRITE: ::core::ffi::c_int = 2;
pub const EV_READ: ::core::ffi::c_int = 1;
pub const EV_NONE: ::core::ffi::c_int = 0;
pub const EV_UNDEF: ::core::ffi::c_int = -1;
pub const EVBREAK_ALL: ::core::ffi::c_int = 2;
pub const EVBREAK_ONE: ::core::ffi::c_int = 1;
pub const EVBREAK_CANCEL: ::core::ffi::c_int = 0;

pub const EVFLAG_AUTO: ::core::ffi::c_uint = 0;
pub const EVFLAG_NOENV: ::core::ffi::c_uint = 0x0100_0000;

#[link(name = "ev")]
unsafe extern "C" {
    pub fn ev_default_loop(flags: ::core::ffi::c_uint) -> *mut ev_loop;
    pub fn ev_loop_destroy(loop_0: *mut ev_loop);
    pub fn ev_run(loop_0: *mut ev_loop, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ev_unref(loop_0: *mut ev_loop);
    pub fn ev_feed_event(
        loop_0: *mut ev_loop,
        w: *mut ::core::ffi::c_void,
        revents: ::core::ffi::c_int,
    );
    pub fn ev_io_start(loop_0: *mut ev_loop, w: *mut ev_io);
    pub fn ev_signal_start(loop_0: *mut ev_loop, w: *mut ev_signal);
    pub fn ev_signal_stop(loop_0: *mut ev_loop, w: *mut ev_signal);
    pub fn ev_prepare_start(loop_0: *mut ev_loop, w: *mut ev_prepare);
    pub fn ev_prepare_stop(loop_0: *mut ev_loop, w: *mut ev_prepare);
    pub fn ev_break(loop_0: *mut ev_loop, how: ::core::ffi::c_int);
    pub fn ev_io_stop(loop_0: *mut ev_loop, w: *mut ev_io);
    pub fn ev_timer_start(loop_0: *mut ev_loop, w: *mut ev_timer);
    pub fn ev_timer_stop(loop_0: *mut ev_loop, w: *mut ev_timer);
    pub fn ev_child_start(loop_0: *mut ev_loop, w: *mut ev_child);
    pub fn ev_child_stop(loop_0: *mut ev_loop, w: *mut ev_child);
    pub fn ev_timer_again(loop_0: *mut ev_loop, w: *mut ev_timer);
    pub fn ev_loop_new(flags: ::core::ffi::c_uint) -> *mut ev_loop;
}

/// C's compatibility alias for running an event loop.
///
/// # Safety
/// `loop_` is a live native loop and all active watchers remain at stable addresses.
#[inline]
pub unsafe fn ev_loop(loop_: *mut ev_loop, flags: ::core::ffi::c_int) {
    unsafe {
        ev_run(loop_, flags);
    }
}

/// C's compatibility alias for stopping a loop.
///
/// # Safety
/// `loop_` is live and called on its owning thread.
#[inline]
pub unsafe fn ev_unloop(loop_: *mut ev_loop, how: ::core::ffi::c_int) {
    unsafe {
        ev_break(loop_, how);
    }
}

/// Destroy the native default loop, when one exists.
///
/// # Safety
/// No remaining active watchers may reference the default loop afterwards.
#[inline]
pub unsafe fn ev_default_destroy() {
    let loop_ = unsafe { ev_default_loop(0) };
    if !loop_.is_null() {
        unsafe {
            ev_loop_destroy(loop_);
        }
    }
}

impl ev_timer {
    /// Initialize an inactive relative timer, equivalent to `ev_timer_init`.
    pub fn new(
        after: ev_tstamp,
        repeat: ev_tstamp,
        callback: unsafe extern "C" fn(*mut ev_loop, *mut ev_timer, ::core::ffi::c_int),
    ) -> Self {
        Self {
            active: 0,
            pending: 0,
            priority: 0,
            data: core::ptr::null_mut(),
            cb: Some(callback),
            at: after,
            repeat,
        }
    }
}
impl ev_io {
    /// Initialize an inactive file watcher, equivalent to `ev_io_init`.
    pub fn new(
        fd: ::core::ffi::c_int,
        events: ::core::ffi::c_int,
        callback: unsafe extern "C" fn(*mut ev_loop, *mut ev_io, ::core::ffi::c_int),
    ) -> Self {
        Self {
            active: 0,
            pending: 0,
            priority: 0,
            data: core::ptr::null_mut(),
            cb: Some(callback),
            next: core::ptr::null_mut(),
            fd,
            events: events | EV__IOFDSET,
        }
    }
}
impl ev_prepare {
    /// Initialize an inactive preparation watcher, equivalent to `ev_prepare_init`.
    pub fn new(
        callback: unsafe extern "C" fn(*mut ev_loop, *mut ev_prepare, ::core::ffi::c_int),
    ) -> Self {
        Self {
            active: 0,
            pending: 0,
            priority: 0,
            data: core::ptr::null_mut(),
            cb: Some(callback),
        }
    }
}
impl ev_signal {
    /// Initialize an inactive signal watcher, equivalent to `ev_signal_init`.
    pub fn new(
        signum: ::core::ffi::c_int,
        callback: unsafe extern "C" fn(*mut ev_loop, *mut ev_signal, ::core::ffi::c_int),
    ) -> Self {
        Self {
            active: 0,
            pending: 0,
            priority: 0,
            data: core::ptr::null_mut(),
            cb: Some(callback),
            next: core::ptr::null_mut(),
            signum,
        }
    }
}
impl ev_child {
    /// Initialize an inactive child watcher, equivalent to `ev_child_init`.
    pub fn new(
        pid: ::core::ffi::c_int,
        trace: bool,
        callback: unsafe extern "C" fn(*mut ev_loop, *mut ev_child, ::core::ffi::c_int),
    ) -> Self {
        Self {
            active: 0,
            pending: 0,
            priority: 0,
            data: core::ptr::null_mut(),
            cb: Some(callback),
            next: core::ptr::null_mut(),
            flags: trace.into(),
            pid,
            rpid: 0,
            rstatus: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn watcher_layout_matches_default_linux_abi() {
        assert_eq!(size_of::<ev_watcher>(), 32);
        assert_eq!(size_of::<ev_watcher_list>(), 40);
        assert_eq!(size_of::<ev_watcher_time>(), 40);
        assert_eq!(size_of::<ev_io>(), 48);
        assert_eq!(size_of::<ev_timer>(), 48);
        assert_eq!(size_of::<ev_prepare>(), 32);
        assert_eq!(size_of::<ev_signal>(), 48);
        assert_eq!(size_of::<ev_child>(), 56);
        assert_eq!(offset_of!(ev_watcher, data), 16);
        assert_eq!(offset_of!(ev_watcher, cb), 24);
        assert_eq!(offset_of!(ev_io, fd), 40);
        assert_eq!(offset_of!(ev_timer, at), 32);
        assert_eq!(offset_of!(ev_timer, repeat), 40);
    }

    struct Outcome {
        calls: usize,
        events: ::core::ffi::c_int,
    }
    unsafe extern "C" fn fired(
        loop_: *mut ev_loop,
        timer: *mut ev_timer,
        events: ::core::ffi::c_int,
    ) {
        // SAFETY: The test supplies a live Outcome in data until ev_run returns.
        unsafe {
            let outcome = &mut *(*timer).data.cast::<Outcome>();
            outcome.calls += 1;
            outcome.events = events;
            ev_break(loop_, EVBREAK_ONE);
        }
    }
    #[test]
    fn one_shot_timer_runs_on_a_new_private_loop() {
        // SAFETY: Loop/timer/context are owned only by this test. The watcher
        // remains at the same address until stopped and the private loop freed.
        unsafe {
            let loop_ = ev_loop_new(EVFLAG_AUTO | EVFLAG_NOENV);
            assert!(!loop_.is_null());
            let mut outcome = Outcome {
                calls: 0,
                events: 0,
            };
            let mut timer = ev_timer::new(0.001, 0.0, fired);
            timer.data = core::ptr::from_mut(&mut outcome).cast();
            ev_timer_start(loop_, &mut timer);
            ev_run(loop_, 0);
            ev_timer_stop(loop_, &mut timer);
            ev_loop_destroy(loop_);
            assert_eq!(outcome.calls, 1);
            assert_ne!(outcome.events & EV_TIMER, 0);
            assert_eq!(timer.active, 0);
        }
    }
}
