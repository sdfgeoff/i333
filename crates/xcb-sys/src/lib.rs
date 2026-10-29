//! Shared raw XCB protocol and extension bindings.
//! Layouts and signatures preserve the existing Linux x86_64 native backend.
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub const XCB_ALLOW_ASYNC_BOTH: xcb_allow_t = 6;
pub const XCB_ALLOW_ASYNC_KEYBOARD: xcb_allow_t = 3;
pub const XCB_ALLOW_ASYNC_POINTER: xcb_allow_t = 0;
pub const XCB_ALLOW_REPLAY_KEYBOARD: xcb_allow_t = 5;
pub const XCB_ALLOW_REPLAY_POINTER: xcb_allow_t = 2;
pub const XCB_ALLOW_SYNC_BOTH: xcb_allow_t = 7;
pub const XCB_ALLOW_SYNC_KEYBOARD: xcb_allow_t = 4;
pub const XCB_ALLOW_SYNC_POINTER: xcb_allow_t = 1;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
pub const XCB_BUTTON_MASK_1: xcb_button_mask_t = 256;
pub const XCB_BUTTON_MASK_2: xcb_button_mask_t = 512;
pub const XCB_BUTTON_MASK_3: xcb_button_mask_t = 1024;
pub const XCB_BUTTON_MASK_4: xcb_button_mask_t = 2048;
pub const XCB_BUTTON_MASK_5: xcb_button_mask_t = 4096;
pub const XCB_BUTTON_MASK_ANY: xcb_button_mask_t = 32768;
pub const XCB_BUTTON_PRESS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const XCB_BUTTON_RELEASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const XCB_BUTTON_SCROLL_DOWN: ::core::ffi::c_int = 5;
pub const XCB_BUTTON_SCROLL_LEFT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const XCB_BUTTON_SCROLL_RIGHT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const XCB_BUTTON_SCROLL_UP: ::core::ffi::c_int = 4;
pub const XCB_CIRCULATE_LOWER_HIGHEST: xcb_circulate_t = 1;
pub const XCB_CIRCULATE_RAISE_LOWEST: xcb_circulate_t = 0;
pub const XCB_CLIENT_MESSAGE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const XCB_CLIP_ORDERING_UNSORTED: xcb_clip_ordering_t = 0;
pub const XCB_CLIP_ORDERING_YX_BANDED: xcb_clip_ordering_t = 3;
pub const XCB_CLIP_ORDERING_YX_SORTED: xcb_clip_ordering_t = 2;
pub const XCB_CLIP_ORDERING_Y_SORTED: xcb_clip_ordering_t = 1;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
pub const XCB_CONFIGURE_NOTIFY: ::core::ffi::c_int = 22;
pub const XCB_CONFIGURE_REQUEST: ::core::ffi::c_int = 23;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
pub const XCB_COPY_FROM_PARENT: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CURRENT_TIME: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
pub const XCB_DESTROY_NOTIFY: ::core::ffi::c_int = 17;
pub const XCB_ENTER_NOTIFY: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EXPOSE: ::core::ffi::c_int = 12;
pub const XCB_FILL_STYLE_OPAQUE_STIPPLED: xcb_fill_style_t = 3;
pub const XCB_FILL_STYLE_SOLID: xcb_fill_style_t = 0;
pub const XCB_FILL_STYLE_STIPPLED: xcb_fill_style_t = 2;
pub const XCB_FILL_STYLE_TILED: xcb_fill_style_t = 1;
pub const XCB_FOCUS_IN: ::core::ffi::c_int = 9;
pub const XCB_FOCUS_OUT: ::core::ffi::c_int = 10;
pub const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;
pub const XCB_GC_BACKGROUND: xcb_gc_t = 8;
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
pub const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
pub const XCB_GC_FILL_RULE: xcb_gc_t = 512;
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
pub const XCB_GC_FONT: xcb_gc_t = 16384;
pub const XCB_GC_FOREGROUND: xcb_gc_t = 4;
pub const XCB_GC_FUNCTION: xcb_gc_t = 1;
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
pub const XCB_GC_STIPPLE: xcb_gc_t = 2048;
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
pub const XCB_GC_TILE: xcb_gc_t = 1024;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;
pub const XCB_GRAB_ANY: xcb_grab_t = 0;
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
pub const XCB_GRAB_STATUS_ALREADY_GRABBED: xcb_grab_status_t = 1;
pub const XCB_GRAB_STATUS_FROZEN: xcb_grab_status_t = 4;
pub const XCB_GRAB_STATUS_INVALID_TIME: xcb_grab_status_t = 2;
pub const XCB_GRAB_STATUS_NOT_VIEWABLE: xcb_grab_status_t = 3;
pub const XCB_GRAB_STATUS_SUCCESS: xcb_grab_status_t = 0;
pub const XCB_GX_AND: xcb_gx_t = 1;
pub const XCB_GX_AND_INVERTED: xcb_gx_t = 4;
pub const XCB_GX_AND_REVERSE: xcb_gx_t = 2;
pub const XCB_GX_CLEAR: xcb_gx_t = 0;
pub const XCB_GX_COPY: xcb_gx_t = 3;
pub const XCB_GX_COPY_INVERTED: xcb_gx_t = 12;
pub const XCB_GX_EQUIV: xcb_gx_t = 9;
pub const XCB_GX_INVERT: xcb_gx_t = 10;
pub const XCB_GX_NAND: xcb_gx_t = 14;
pub const XCB_GX_NOOP: xcb_gx_t = 5;
pub const XCB_GX_NOR: xcb_gx_t = 8;
pub const XCB_GX_OR: xcb_gx_t = 7;
pub const XCB_GX_OR_INVERTED: xcb_gx_t = 13;
pub const XCB_GX_OR_REVERSE: xcb_gx_t = 11;
pub const XCB_GX_SET: xcb_gx_t = 15;
pub const XCB_GX_XOR: xcb_gx_t = 6;
pub const XCB_ICCCM_SIZE_HINT_BASE_SIZE: ::core::ffi::c_uint = 256;
pub const XCB_ICCCM_SIZE_HINT_P_ASPECT: ::core::ffi::c_uint = 128;
pub const XCB_ICCCM_SIZE_HINT_P_MAX_SIZE: ::core::ffi::c_uint = 32;
pub const XCB_ICCCM_SIZE_HINT_P_MIN_SIZE: ::core::ffi::c_uint = 16;
pub const XCB_ICCCM_SIZE_HINT_P_POSITION: ::core::ffi::c_uint = 4;
pub const XCB_ICCCM_SIZE_HINT_P_RESIZE_INC: ::core::ffi::c_uint = 64;
pub const XCB_ICCCM_SIZE_HINT_P_SIZE: ::core::ffi::c_uint = 8;
pub const XCB_ICCCM_SIZE_HINT_P_WIN_GRAVITY: ::core::ffi::c_uint = 512;
pub const XCB_ICCCM_SIZE_HINT_US_POSITION: ::core::ffi::c_uint = 1;
pub const XCB_ICCCM_SIZE_HINT_US_SIZE: ::core::ffi::c_uint = 2;
pub const XCB_ICCCM_WM_HINT_ICON_MASK: ::core::ffi::c_uint = 32;
pub const XCB_ICCCM_WM_HINT_ICON_PIXMAP: ::core::ffi::c_uint = 4;
pub const XCB_ICCCM_WM_HINT_ICON_POSITION: ::core::ffi::c_uint = 16;
pub const XCB_ICCCM_WM_HINT_ICON_WINDOW: ::core::ffi::c_uint = 8;
pub const XCB_ICCCM_WM_HINT_INPUT: ::core::ffi::c_uint = 1;
pub const XCB_ICCCM_WM_HINT_STATE: ::core::ffi::c_uint = 2;
pub const XCB_ICCCM_WM_HINT_WINDOW_GROUP: ::core::ffi::c_uint = 64;
pub const XCB_ICCCM_WM_HINT_X_URGENCY: ::core::ffi::c_uint = 256;
pub const XCB_ICCCM_WM_STATE_ICONIC: ::core::ffi::c_uint = 3;
pub const XCB_ICCCM_WM_STATE_NORMAL: ::core::ffi::c_uint = 1;
pub const XCB_ICCCM_WM_STATE_WITHDRAWN: ::core::ffi::c_uint = 0;
pub const XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
pub const XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
pub const XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;
pub const XCB_INPUT_FOCUS_FOLLOW_KEYBOARD: xcb_input_focus_t = 3;
pub const XCB_INPUT_FOCUS_NONE: xcb_input_focus_t = 0;
pub const XCB_INPUT_FOCUS_PARENT: xcb_input_focus_t = 2;
pub const XCB_INPUT_FOCUS_POINTER_ROOT: xcb_input_focus_t = 1;
pub const XCB_KEY_BUT_MASK_BUTTON_1: xcb_key_but_mask_t = 256;
pub const XCB_KEY_BUT_MASK_BUTTON_2: xcb_key_but_mask_t = 512;
pub const XCB_KEY_BUT_MASK_BUTTON_3: xcb_key_but_mask_t = 1024;
pub const XCB_KEY_BUT_MASK_BUTTON_4: xcb_key_but_mask_t = 2048;
pub const XCB_KEY_BUT_MASK_BUTTON_5: xcb_key_but_mask_t = 4096;
pub const XCB_KEY_BUT_MASK_CONTROL: xcb_key_but_mask_t = 4;
pub const XCB_KEY_BUT_MASK_LOCK: xcb_key_but_mask_t = 2;
pub const XCB_KEY_BUT_MASK_MOD_1: xcb_key_but_mask_t = 8;
pub const XCB_KEY_BUT_MASK_MOD_2: xcb_key_but_mask_t = 16;
pub const XCB_KEY_BUT_MASK_MOD_3: xcb_key_but_mask_t = 32;
pub const XCB_KEY_BUT_MASK_MOD_4: xcb_key_but_mask_t = 64;
pub const XCB_KEY_BUT_MASK_MOD_5: xcb_key_but_mask_t = 128;
pub const XCB_KEY_BUT_MASK_SHIFT: xcb_key_but_mask_t = 1;
pub const XCB_KEY_PRESS: ::core::ffi::c_int = 2;
pub const XCB_KEY_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XCB_MAPPING_KEYBOARD: xcb_mapping_t = 1;
pub const XCB_MAPPING_MODIFIER: xcb_mapping_t = 0;
pub const XCB_MAPPING_NOTIFY: ::core::ffi::c_int = 34;
pub const XCB_MAPPING_POINTER: xcb_mapping_t = 2;
pub const XCB_MAP_NOTIFY: ::core::ffi::c_int = 19;
pub const XCB_MAP_REQUEST: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const XCB_MAP_STATE_UNMAPPED: xcb_map_state_t = 0;
pub const XCB_MAP_STATE_UNVIEWABLE: xcb_map_state_t = 1;
pub const XCB_MAP_STATE_VIEWABLE: xcb_map_state_t = 2;
pub const XCB_MOD_MASK_1: xcb_mod_mask_t = 8;
pub const XCB_MOD_MASK_2: xcb_mod_mask_t = 16;
pub const XCB_MOD_MASK_3: xcb_mod_mask_t = 32;
pub const XCB_MOD_MASK_4: xcb_mod_mask_t = 64;
pub const XCB_MOD_MASK_5: xcb_mod_mask_t = 128;
pub const XCB_MOD_MASK_ANY: xcb_mod_mask_t = 32768;
pub const XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 4;
pub const XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 2;
pub const XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1;
pub const XCB_MOTION_NOTIFY: ::core::ffi::c_int = 6;
pub const XCB_NONE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const XCB_NOTIFY_DETAIL_ANCESTOR: xcb_notify_detail_t = 0;
pub const XCB_NOTIFY_DETAIL_INFERIOR: xcb_notify_detail_t = 2;
pub const XCB_NOTIFY_DETAIL_NONE: xcb_notify_detail_t = 7;
pub const XCB_NOTIFY_DETAIL_NONLINEAR: xcb_notify_detail_t = 3;
pub const XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_notify_detail_t = 4;
pub const XCB_NOTIFY_DETAIL_POINTER: xcb_notify_detail_t = 5;
pub const XCB_NOTIFY_DETAIL_POINTER_ROOT: xcb_notify_detail_t = 6;
pub const XCB_NOTIFY_DETAIL_VIRTUAL: xcb_notify_detail_t = 1;
pub const XCB_NOTIFY_MODE_GRAB: xcb_notify_mode_t = 1;
pub const XCB_NOTIFY_MODE_NORMAL: xcb_notify_mode_t = 0;
pub const XCB_NOTIFY_MODE_UNGRAB: xcb_notify_mode_t = 2;
pub const XCB_NOTIFY_MODE_WHILE_GRABBED: xcb_notify_mode_t = 3;
pub const XCB_NUM_LOCK: ::core::ffi::c_int = 0xff7f as ::core::ffi::c_int;
pub const XCB_PIXMAP_NONE: xcb_pixmap_enum_t = 0;
pub const XCB_PROPERTY_DELETE: xcb_property_t = 1;
pub const XCB_PROPERTY_NEW_VALUE: xcb_property_t = 0;
pub const XCB_PROPERTY_NOTIFY: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
pub const XCB_RANDR_MAJOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XCB_RANDR_MINOR_VERSION: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE: xcb_randr_notify_mask_t = 2;
pub const XCB_RANDR_NOTIFY_MASK_LEASE: xcb_randr_notify_mask_t = 128;
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE: xcb_randr_notify_mask_t = 4;
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY: xcb_randr_notify_mask_t = 8;
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_CHANGE: xcb_randr_notify_mask_t = 16;
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_PROPERTY: xcb_randr_notify_mask_t = 32;
pub const XCB_RANDR_NOTIFY_MASK_RESOURCE_CHANGE: xcb_randr_notify_mask_t = 64;
pub const XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE: xcb_randr_notify_mask_t = 1;
pub const XCB_RANDR_SCREEN_CHANGE_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_RESIZE_REQUEST: ::core::ffi::c_int = 25;
pub const XCB_SELECTION_CLEAR: ::core::ffi::c_int = 29;
pub const XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;
pub const XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
pub const XCB_SHAPE_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_SHAPE_SK_BOUNDING: xcb_shape_sk_t = 0;
pub const XCB_SHAPE_SK_CLIP: xcb_shape_sk_t = 1;
pub const XCB_SHAPE_SK_INPUT: xcb_shape_sk_t = 2;
pub const XCB_SHAPE_SO_INTERSECT: xcb_shape_so_t = 2;
pub const XCB_SHAPE_SO_INVERT: xcb_shape_so_t = 4;
pub const XCB_SHAPE_SO_SET: xcb_shape_so_t = 0;
pub const XCB_SHAPE_SO_SUBTRACT: xcb_shape_so_t = 3;
pub const XCB_SHAPE_SO_UNION: xcb_shape_so_t = 1;
pub const XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;
pub const XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
pub const XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
pub const XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;
pub const XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
pub const XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN: xcb_subwindow_mode_t = 0;
pub const XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS: xcb_subwindow_mode_t = 1;
pub const XCB_UNMAP_NOTIFY: ::core::ffi::c_int = 18;
pub const XCB_VISIBILITY_FULLY_OBSCURED: xcb_visibility_t = 2;
pub const XCB_VISIBILITY_NOTIFY: ::core::ffi::c_int = 15;
pub const XCB_VISIBILITY_PARTIALLY_OBSCURED: xcb_visibility_t = 1;
pub const XCB_VISIBILITY_UNOBSCURED: xcb_visibility_t = 0;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;
pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY: xcb_xkb_event_type_t = 1024;
pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE: xcb_xkb_event_type_t = 512;
pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY: xcb_xkb_event_type_t = 256;
pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY: xcb_xkb_event_type_t = 128;
pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY: xcb_xkb_event_type_t = 8;
pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: xcb_xkb_event_type_t = 2048;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY: xcb_xkb_event_type_t = 32;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY: xcb_xkb_event_type_t = 16;
pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY: xcb_xkb_event_type_t = 2;
pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY: xcb_xkb_event_type_t = 64;
pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY: xcb_xkb_event_type_t = 1;
pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY: xcb_xkb_event_type_t = 4;
pub const XCB_XKB_GROUP_1: xcb_xkb_group_t = 0;
pub const XCB_XKB_GROUP_2: xcb_xkb_group_t = 1;
pub const XCB_XKB_GROUP_3: xcb_xkb_group_t = 2;
pub const XCB_XKB_GROUP_4: xcb_xkb_group_t = 3;
pub const XCB_XKB_ID_ALL_XI_CLASS: xcb_xkb_id_t = 1280;
pub const XCB_XKB_ID_ALL_XI_ID: xcb_xkb_id_t = 1536;
pub const XCB_XKB_ID_DFLT_XI_CLASS: xcb_xkb_id_t = 768;
pub const XCB_XKB_ID_DFLT_XI_ID: xcb_xkb_id_t = 1024;
pub const XCB_XKB_ID_USE_CORE_KBD: xcb_xkb_id_t = 256;
pub const XCB_XKB_ID_USE_CORE_PTR: xcb_xkb_id_t = 512;
pub const XCB_XKB_ID_XI_NONE: xcb_xkb_id_t = 65280;
pub const XCB_XKB_MAJOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XCB_XKB_MAP_NOTIFY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XCB_XKB_MINOR_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_XKB_NEW_KEYBOARD_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XCB_XKB_NKN_DETAIL_DEVICE_ID: xcb_xkb_nkn_detail_t = 4;
pub const XCB_XKB_NKN_DETAIL_GEOMETRY: xcb_xkb_nkn_detail_t = 2;
pub const XCB_XKB_NKN_DETAIL_KEYCODES: xcb_xkb_nkn_detail_t = 1;
pub const XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS: xcb_xkb_per_client_flag_t = 4;
pub const XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT: xcb_xkb_per_client_flag_t = 1;
pub const XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE: xcb_xkb_per_client_flag_t = 2;
pub const XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: xcb_xkb_per_client_flag_t = 8;
pub const XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: xcb_xkb_per_client_flag_t = 16;
pub const XCB_XKB_STATE_NOTIFY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[repr(C)]
pub struct _XCBKeySymbols {
    _private: [u8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_color_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_alloc_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad1: [u8; 2],
    pub pixel: u32,
}
pub type xcb_allow_t = ::core::ffi::c_uint;
pub type xcb_atom_enum_t = ::core::ffi::c_uint;
pub type xcb_atom_t = u32;
pub type xcb_button_index_t = ::core::ffi::c_uint;
pub type xcb_button_mask_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_button_press_event_t {
    pub response_type: u8,
    pub detail: xcb_button_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}
pub type xcb_button_release_event_t = xcb_button_press_event_t;
pub type xcb_button_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}
pub type xcb_circulate_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_client_message_data_t {
    pub data8: [u8; 20],
    pub data16: [u16; 10],
    pub data32: [u32; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub type_0: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}
pub type xcb_clip_ordering_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_color_strings_t {
    pub bar_fg: *mut ::core::ffi::c_char,
    pub bar_bg: *mut ::core::ffi::c_char,
    pub sep_fg: *mut ::core::ffi::c_char,
    pub focus_bar_fg: *mut ::core::ffi::c_char,
    pub focus_bar_bg: *mut ::core::ffi::c_char,
    pub focus_sep_fg: *mut ::core::ffi::c_char,
    pub active_ws_fg: *mut ::core::ffi::c_char,
    pub active_ws_bg: *mut ::core::ffi::c_char,
    pub active_ws_border: *mut ::core::ffi::c_char,
    pub inactive_ws_fg: *mut ::core::ffi::c_char,
    pub inactive_ws_bg: *mut ::core::ffi::c_char,
    pub inactive_ws_border: *mut ::core::ffi::c_char,
    pub focus_ws_bg: *mut ::core::ffi::c_char,
    pub focus_ws_fg: *mut ::core::ffi::c_char,
    pub focus_ws_border: *mut ::core::ffi::c_char,
    pub urgent_ws_bg: *mut ::core::ffi::c_char,
    pub urgent_ws_fg: *mut ::core::ffi::c_char,
    pub urgent_ws_border: *mut ::core::ffi::c_char,
    pub binding_mode_bg: *mut ::core::ffi::c_char,
    pub binding_mode_fg: *mut ::core::ffi::c_char,
    pub binding_mode_border: *mut ::core::ffi::c_char,
}
pub type xcb_colormap_alloc_t = ::core::ffi::c_uint;
pub type xcb_colormap_t = u32;

pub type xcb_config_window_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_configure_request_event_t {
    pub response_type: u8,
    pub stack_mode: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub value_mask: u16,
}
#[repr(C)]
pub struct xcb_cursor_context_t {
    _private: [u8; 0],
}
pub type xcb_cursor_t = u32;
pub type xcb_cw_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: u8,
    pub pad0: u8,
    pub visuals_len: u16,
    pub pad1: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
}
pub type xcb_drawable_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_enter_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub mode: u8,
    pub same_screen_focus: u8,
}
pub type xcb_event_mask_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
    pub pad1: [u8; 2],
}
#[repr(C)]
pub struct xcb_extension_t {
    _private: [u8; 0],
}
pub type xcb_fill_style_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_focus_in_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub mode: u8,
    pub pad0: [u8; 3],
}
pub type xcb_focus_out_event_t = xcb_focus_in_event_t;
pub type xcb_font_t = u32;
pub type xcb_fontable_t = u32;
pub type xcb_gc_t = ::core::ffi::c_uint;
pub type xcb_gcontext_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_atom_name_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub name_len: u16,
    pub pad1: [u8; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub pad0: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_image_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub pad0: [u8; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: u8,
    pub revert_to: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_reply_t {
    pub response_type: u8,
    pub keycodes_per_modifier: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_0: xcb_atom_t,
    pub bytes_after: u32,
    pub value_len: u32,
    pub pad0: [u8; 12],
}
pub type xcb_get_property_type_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_selection_owner_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub owner: xcb_window_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type: u8,
    pub backing_store: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub _class: u16,
    pub bit_gravity: u8,
    pub win_gravity: u8,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub save_under: u8,
    pub map_is_installed: u8,
    pub map_state: u8,
    pub override_redirect: u8,
    pub colormap: xcb_colormap_t,
    pub all_event_masks: u32,
    pub your_event_mask: u32,
    pub do_not_propagate_mask: u16,
    pub pad0: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_keyboard_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_keyboard_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}
pub type xcb_grab_mode_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_pointer_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_grab_pointer_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}
pub type xcb_grab_status_t = ::core::ffi::c_uint;
pub type xcb_grab_t = ::core::ffi::c_uint;
pub type xcb_gx_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_icccm_get_wm_protocols_reply_t {
    pub atoms_len: u32,
    pub atoms: *mut xcb_atom_t,
    pub _reply: *mut xcb_get_property_reply_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_icccm_wm_hints_t {
    pub flags: i32,
    pub input: u32,
    pub initial_state: i32,
    pub icon_pixmap: xcb_pixmap_t,
    pub icon_window: xcb_window_t,
    pub icon_x: i32,
    pub icon_y: i32,
    pub icon_mask: xcb_pixmap_t,
    pub window_group: xcb_window_t,
}
pub type xcb_image_format_t = ::core::ffi::c_uint;
pub type xcb_input_focus_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
}
pub type xcb_key_but_mask_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type: u8,
    pub detail: xcb_keycode_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}
pub type xcb_key_release_event_t = xcb_key_press_event_t;
pub type xcb_key_symbols_t = _XCBKeySymbols;
pub type xcb_keycode_t = u8;
pub type xcb_keysym_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_map_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub override_redirect: u8,
    pub pad1: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_map_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
}
pub type xcb_map_state_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_mapping_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub request: u8,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
    pub pad1: u8,
}
pub type xcb_mapping_t = ::core::ffi::c_uint;
pub type xcb_mod_mask_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_motion_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}
pub type xcb_notify_detail_t = ::core::ffi::c_uint;
pub type xcb_notify_mode_t = ::core::ffi::c_uint;
pub type xcb_pixmap_enum_t = ::core::ffi::c_uint;
pub type xcb_pixmap_t = u32;
pub type xcb_prop_mode_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_property_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: u8,
    pub pad1: [u8; 3],
}
pub type xcb_property_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u8,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_font_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_font_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [u8; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [u8; 4],
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub properties_len: u16,
    pub draw_direction: u8,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub char_infos_len: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: u16,
    pub pad0: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_text_extents_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_text_extents_reply_t {
    pub response_type: u8,
    pub draw_direction: u8,
    pub sequence: u16,
    pub length: u32,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub overall_ascent: i16,
    pub overall_descent: i16,
    pub overall_width: i32,
    pub overall_left: i32,
    pub overall_right: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_tree_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_query_tree_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub parent: xcb_window_t,
    pub children_len: u16,
    pub pad1: [u8; 14],
}
pub type xcb_randr_crtc_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub rotations: u16,
    pub num_outputs: u16,
    pub num_possible_outputs: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_monitors_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_monitors_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub nMonitors: u32,
    pub nOutputs: u32,
    pub pad1: [u8; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_info_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub crtc: xcb_randr_crtc_t,
    pub mm_width: u32,
    pub mm_height: u32,
    pub connection: u8,
    pub subpixel_order: u8,
    pub num_crtcs: u16,
    pub num_modes: u16,
    pub num_preferred: u16,
    pub num_clones: u16,
    pub name_len: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub output: xcb_randr_output_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub num_crtcs: u16,
    pub num_outputs: u16,
    pub num_modes: u16,
    pub names_len: u16,
    pub pad1: [u8; 8],
}
pub type xcb_randr_mode_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_monitor_info_iterator_t {
    pub data: *mut xcb_randr_monitor_info_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_monitor_info_t {
    pub name: xcb_atom_t,
    pub primary: u8,
    pub automatic: u8,
    pub nOutput: u16,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub width_in_millimeters: u32,
    pub height_in_millimeters: u32,
}
pub type xcb_randr_notify_mask_t = ::core::ffi::c_uint;
pub type xcb_randr_output_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_query_version_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_randr_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_resize_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub width: u16,
    pub height: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_selection_clear_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
}
pub type xcb_set_mode_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [u8; 4],
}
pub type xcb_shape_kind_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_notify_event_t {
    pub response_type: u8,
    pub shape_kind: xcb_shape_kind_t,
    pub sequence: u16,
    pub affected_window: xcb_window_t,
    pub extents_x: i16,
    pub extents_y: i16,
    pub extents_width: u16,
    pub extents_height: u16,
    pub server_time: xcb_timestamp_t,
    pub shaped: u8,
    pub pad0: [u8; 11],
}
pub type xcb_shape_op_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_extents_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_extents_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub bounding_shaped: u8,
    pub clip_shaped: u8,
    pub pad1: [u8; 2],
    pub bounding_shape_extents_x: i16,
    pub bounding_shape_extents_y: i16,
    pub bounding_shape_extents_width: u16,
    pub bounding_shape_extents_height: u16,
    pub clip_shape_extents_x: i16,
    pub clip_shape_extents_y: i16,
    pub clip_shape_extents_width: u16,
    pub clip_shape_extents_height: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_version_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_shape_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
pub type xcb_shape_sk_t = ::core::ffi::c_uint;
pub type xcb_shape_so_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_size_hints_t {
    pub flags: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,
    pub width_inc: i32,
    pub height_inc: i32,
    pub min_aspect_num: i32,
    pub min_aspect_den: i32,
    pub max_aspect_num: i32,
    pub max_aspect_den: i32,
    pub base_width: i32,
    pub base_height: i32,
    pub win_gravity: u32,
}
pub type xcb_stack_mode_t = ::core::ffi::c_uint;
pub type xcb_subwindow_mode_t = ::core::ffi::c_uint;
pub type xcb_timestamp_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_translate_coordinates_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_translate_coordinates_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub child: xcb_window_t,
    pub dst_x: i16,
    pub dst_y: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: u8,
    pub pad1: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visibility_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub state: u8,
    pub pad1: [u8; 3],
}
pub type xcb_visibility_t = ::core::ffi::c_uint;
pub type xcb_visualid_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub _class: u8,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
pub type xcb_window_class_t = ::core::ffi::c_uint;
pub type xcb_window_enum_t = ::core::ffi::c_uint;
pub type xcb_window_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_is_active_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_is_active_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub number: u32,
    pub pad1: [u8; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xinerama_screen_info_t {
    pub x_org: i16,
    pub y_org: i16,
    pub width: u16,
    pub height: u16,
}
pub type xcb_xkb_device_spec_t = u16;
pub type xcb_xkb_event_type_t = ::core::ffi::c_uint;
pub type xcb_xkb_group_t = ::core::ffi::c_uint;
pub type xcb_xkb_id_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_new_keyboard_notify_event_t {
    pub response_type: u8,
    pub xkbType: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub deviceID: u8,
    pub oldDeviceID: u8,
    pub minKeyCode: xcb_keycode_t,
    pub maxKeyCode: xcb_keycode_t,
    pub oldMinKeyCode: xcb_keycode_t,
    pub oldMaxKeyCode: xcb_keycode_t,
    pub requestMajor: u8,
    pub requestMinor: u8,
    pub changed: u16,
    pub pad0: [u8; 14],
}
pub type xcb_xkb_nkn_detail_t = ::core::ffi::c_uint;
pub type xcb_xkb_per_client_flag_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_reply_t {
    pub response_type: u8,
    pub deviceID: u8,
    pub sequence: u16,
    pub length: u32,
    pub supported: u32,
    pub value: u32,
    pub autoCtrls: u32,
    pub autoCtrlsValues: u32,
    pub pad0: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
    pub response_type: u8,
    pub xkbType: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub deviceID: u8,
    pub mods: u8,
    pub baseMods: u8,
    pub latchedMods: u8,
    pub lockedMods: u8,
    pub group: u8,
    pub baseGroup: i16,
    pub latchedGroup: i16,
    pub lockedGroup: u8,
    pub compatState: u8,
    pub grabMods: u8,
    pub compatGrabMods: u8,
    pub lookupMods: u8,
    pub compatLoockupMods: u8,
    pub ptrBtnState: u16,
    pub changed: u16,
    pub keycode: xcb_keycode_t,
    pub eventType: u8,
    pub requestMajor: u8,
    pub requestMinor: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    pub sequence: ::core::ffi::c_uint,
}
#[repr(C)]
pub struct xcb_xrm_database_t {
    _private: [u8; 0],
}
unsafe extern "C" {
    pub fn xcb_alloc_color(
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: u16,
        green: u16,
        blue: u16,
    ) -> xcb_alloc_color_cookie_t;
    pub fn xcb_alloc_color_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_reply_t;
    pub fn xcb_allow_events(
        c: *mut xcb_connection_t,
        mode: u8,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_atom_name_by_screen(
        base: *const ::core::ffi::c_char,
        screen: u8,
    ) -> *mut ::core::ffi::c_char;
    pub fn xcb_aux_find_visual_by_attrs(
        screen: *mut xcb_screen_t,
        class_: i8,
        depth: i8,
    ) -> *mut xcb_visualtype_t;
    pub fn xcb_aux_get_depth_of_visual(screen: *mut xcb_screen_t, id: xcb_visualid_t) -> u8;
    pub fn xcb_aux_get_screen(
        c: *mut xcb_connection_t,
        screen: ::core::ffi::c_int,
    ) -> *mut xcb_screen_t;
    pub fn xcb_aux_sync(c: *mut xcb_connection_t);
    pub fn xcb_change_active_pointer_grab(
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_gc(
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_save_set(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_change_window_attributes_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_circulate_window(
        c: *mut xcb_connection_t,
        direction: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_clear_area(
        c: *mut xcb_connection_t,
        exposures: u8,
        window: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_close_font(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t;
    pub fn xcb_configure_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_configure_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_connect(
        displayname: *const ::core::ffi::c_char,
        screenp: *mut ::core::ffi::c_int,
    ) -> *mut xcb_connection_t;
    pub fn xcb_connection_has_error(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    pub fn xcb_copy_area(
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_colormap(
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_colormap_checked(
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_gc(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_gc_checked(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_pixmap(
        c: *mut xcb_connection_t,
        depth: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_pixmap_checked(
        c: *mut xcb_connection_t,
        depth_0: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_create_window_checked(
        c: *mut xcb_connection_t,
        depth_0: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_cursor_context_free(ctx: *mut xcb_cursor_context_t);
    pub fn xcb_cursor_context_new(
        conn_0: *mut xcb_connection_t,
        screen: *mut xcb_screen_t,
        ctx_0: *mut *mut xcb_cursor_context_t,
    ) -> ::core::ffi::c_int;
    pub fn xcb_cursor_load_cursor(
        ctx_0: *mut xcb_cursor_context_t,
        name: *const ::core::ffi::c_char,
    ) -> xcb_cursor_t;
    pub fn xcb_delete_property(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_depth_next(i: *mut xcb_depth_iterator_t);
    pub fn xcb_depth_visuals_iterator(R: *const xcb_depth_t) -> xcb_visualtype_iterator_t;
    pub fn xcb_destroy_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    pub fn xcb_discard_reply(c: *mut xcb_connection_t, sequence: ::core::ffi::c_uint);
    pub fn xcb_disconnect(c: *mut xcb_connection_t);
    pub fn xcb_flush(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    pub fn xcb_free_colormap(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t;
    pub fn xcb_free_cursor(c: *mut xcb_connection_t, cursor_0: xcb_cursor_t) -> xcb_void_cookie_t;
    pub fn xcb_free_gc(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t;
    pub fn xcb_free_pixmap(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t;
    pub fn xcb_generate_id(c: *mut xcb_connection_t) -> u32;
    pub fn xcb_get_atom_name(
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t;
    pub fn xcb_get_atom_name_name(R: *const xcb_get_atom_name_reply_t) -> *mut ::core::ffi::c_char;
    pub fn xcb_get_atom_name_name_length(R: *const xcb_get_atom_name_reply_t)
        -> ::core::ffi::c_int;
    pub fn xcb_get_atom_name_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_atom_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_atom_name_reply_t;
    pub fn xcb_get_extension_data(
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t;
    pub fn xcb_get_file_descriptor(c: *mut xcb_connection_t) -> ::core::ffi::c_int;
    pub fn xcb_get_geometry(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    pub fn xcb_get_geometry_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t;
    pub fn xcb_get_geometry_unchecked(
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t;
    pub fn xcb_get_image_data(R: *const xcb_get_image_reply_t) -> *mut u8;
    pub fn xcb_get_image_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_image_reply_t;
    pub fn xcb_get_image_unchecked(
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
    ) -> xcb_get_image_cookie_t;
    pub fn xcb_get_input_focus(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t;
    pub fn xcb_get_input_focus_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_input_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_input_focus_reply_t;
    pub fn xcb_get_modifier_mapping(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t;
    pub fn xcb_get_modifier_mapping_keycodes(
        R: *const xcb_get_modifier_mapping_reply_t,
    ) -> *mut xcb_keycode_t;
    pub fn xcb_get_modifier_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_modifier_mapping_reply_t;
    pub fn xcb_get_property(
        c: *mut xcb_connection_t,
        _delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_get_property_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t;
    pub fn xcb_get_property_unchecked(
        c: *mut xcb_connection_t,
        _delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_0: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_get_property_value(R: *const xcb_get_property_reply_t) -> *mut ::core::ffi::c_void;
    pub fn xcb_get_property_value_length(R: *const xcb_get_property_reply_t) -> ::core::ffi::c_int;
    pub fn xcb_get_selection_owner(
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t;
    pub fn xcb_get_selection_owner_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_selection_owner_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_selection_owner_reply_t;
    pub fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;
    pub fn xcb_get_window_attributes(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
    pub fn xcb_get_window_attributes_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_window_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_window_attributes_reply_t;
    pub fn xcb_get_window_attributes_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t;
    pub fn xcb_grab_button(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: u8,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_key(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
        key: xcb_keycode_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_void_cookie_t;
    pub fn xcb_grab_keyboard(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_grab_keyboard_cookie_t;
    pub fn xcb_grab_keyboard_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_keyboard_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_keyboard_reply_t;
    pub fn xcb_grab_pointer(
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t;
    pub fn xcb_grab_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_grab_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_pointer_reply_t;
    pub fn xcb_grab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_icccm_get_wm_hints(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_icccm_get_wm_hints_from_reply(
        hints: *mut xcb_icccm_wm_hints_t,
        reply: *mut xcb_get_property_reply_t,
    ) -> u8;
    pub fn xcb_icccm_get_wm_normal_hints(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_icccm_get_wm_normal_hints_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        hints: *mut xcb_size_hints_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> u8;
    pub fn xcb_icccm_get_wm_normal_hints_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_icccm_get_wm_protocols(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        wm_protocol_atom: xcb_atom_t,
    ) -> xcb_get_property_cookie_t;
    pub fn xcb_icccm_get_wm_protocols_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        protocols: *mut xcb_icccm_get_wm_protocols_reply_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> u8;
    pub fn xcb_icccm_get_wm_protocols_reply_wipe(
        protocols: *mut xcb_icccm_get_wm_protocols_reply_t,
    );
    pub fn xcb_icccm_get_wm_size_hints_from_reply(
        hints: *mut xcb_size_hints_t,
        reply: *mut xcb_get_property_reply_t,
    ) -> u8;
    pub fn xcb_icccm_get_wm_transient_for_from_reply(
        prop: *mut xcb_window_t,
        reply: *mut xcb_get_property_reply_t,
    ) -> u8;
    pub fn xcb_icccm_set_wm_hints(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        hints: *mut xcb_icccm_wm_hints_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_icccm_wm_hints_get_urgency(hints: *mut xcb_icccm_wm_hints_t) -> u32;
    pub fn xcb_icccm_wm_hints_set_input(hints: *mut xcb_icccm_wm_hints_t, input: u8);
    pub fn xcb_icccm_wm_hints_set_none(hints: *mut xcb_icccm_wm_hints_t);
    pub fn xcb_image_text_16(
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const ::core::ffi::c_char,
    ) -> xcb_intern_atom_cookie_t;
    pub fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;
    pub fn xcb_intern_atom_unchecked(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const ::core::ffi::c_char,
    ) -> xcb_intern_atom_cookie_t;
    pub fn xcb_is_cursor_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    pub fn xcb_is_function_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    pub fn xcb_is_keypad_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    pub fn xcb_is_misc_function_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    pub fn xcb_is_modifier_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    pub fn xcb_is_pf_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    pub fn xcb_is_private_keypad_key(keysym: xcb_keysym_t) -> ::core::ffi::c_int;
    pub fn xcb_key_press_lookup_keysym(
        syms: *mut xcb_key_symbols_t,
        event: *mut xcb_key_press_event_t,
        col: ::core::ffi::c_int,
    ) -> xcb_keysym_t;
    pub fn xcb_key_symbols_alloc(c: *mut xcb_connection_t) -> *mut xcb_key_symbols_t;
    pub fn xcb_key_symbols_free(syms: *mut xcb_key_symbols_t);
    pub fn xcb_key_symbols_get_keycode(
        syms: *mut xcb_key_symbols_t,
        keysym: xcb_keysym_t,
    ) -> *mut xcb_keycode_t;
    pub fn xcb_key_symbols_get_keysym(
        syms: *mut xcb_key_symbols_t,
        keycode: xcb_keycode_t,
        col: ::core::ffi::c_int,
    ) -> xcb_keysym_t;
    pub fn xcb_kill_client(c: *mut xcb_connection_t, resource: u32) -> xcb_void_cookie_t;
    pub fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    pub fn xcb_map_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_open_font_checked(
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: u16,
        name: *const ::core::ffi::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    pub fn xcb_prefetch_extension_data(c: *mut xcb_connection_t, ext: *mut xcb_extension_t);
    pub fn xcb_prefetch_maximum_request_length(c: *mut xcb_connection_t);
    pub fn xcb_query_font(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t;
    pub fn xcb_query_font_char_infos(R: *const xcb_query_font_reply_t) -> *mut xcb_charinfo_t;
    pub fn xcb_query_font_char_infos_length(R: *const xcb_query_font_reply_t)
        -> ::core::ffi::c_int;
    pub fn xcb_query_font_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_font_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_font_reply_t;
    pub fn xcb_query_pointer(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t;
    pub fn xcb_query_pointer_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_pointer_reply_t;
    pub fn xcb_query_text_extents(
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: u32,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t;
    pub fn xcb_query_text_extents_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_text_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_text_extents_reply_t;
    pub fn xcb_query_tree(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t;
    pub fn xcb_query_tree_children(R: *const xcb_query_tree_reply_t) -> *mut xcb_window_t;
    pub fn xcb_query_tree_children_length(R: *const xcb_query_tree_reply_t) -> ::core::ffi::c_int;
    pub fn xcb_query_tree_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_query_tree_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_tree_reply_t;
    pub fn xcb_query_tree_unchecked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t;
    pub fn xcb_randr_get_crtc_info(
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_crtc_info_cookie_t;
    pub fn xcb_randr_get_crtc_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_info_reply_t;
    pub fn xcb_randr_get_monitors(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        get_active: u8,
    ) -> xcb_randr_get_monitors_cookie_t;
    pub fn xcb_randr_get_monitors_monitors_iterator(
        R: *const xcb_randr_get_monitors_reply_t,
    ) -> xcb_randr_monitor_info_iterator_t;
    pub fn xcb_randr_get_monitors_monitors_length(
        R: *const xcb_randr_get_monitors_reply_t,
    ) -> ::core::ffi::c_int;
    pub fn xcb_randr_get_monitors_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_monitors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_monitors_reply_t;
    pub fn xcb_randr_get_output_info(
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_output_info_cookie_t;
    pub fn xcb_randr_get_output_info_name(R: *const xcb_randr_get_output_info_reply_t) -> *mut u8;
    pub fn xcb_randr_get_output_info_name_length(
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> ::core::ffi::c_int;
    pub fn xcb_randr_get_output_info_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_info_reply_t;
    pub fn xcb_randr_get_output_primary(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_output_primary_cookie_t;
    pub fn xcb_randr_get_output_primary_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_primary_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_primary_reply_t;
    pub fn xcb_randr_get_screen_resources_current(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_current_cookie_t;
    pub fn xcb_randr_get_screen_resources_current_outputs(
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_output_t;
    pub fn xcb_randr_get_screen_resources_current_outputs_length(
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> ::core::ffi::c_int;
    pub fn xcb_randr_get_screen_resources_current_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_resources_current_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_resources_current_reply_t;
    pub fn xcb_randr_monitor_info_next(i: *mut xcb_randr_monitor_info_iterator_t);
    pub fn xcb_randr_monitor_info_outputs(
        R: *const xcb_randr_monitor_info_t,
    ) -> *mut xcb_randr_output_t;
    pub fn xcb_randr_monitor_info_outputs_length(
        R: *const xcb_randr_monitor_info_t,
    ) -> ::core::ffi::c_int;
    pub fn xcb_randr_query_version(
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_randr_query_version_cookie_t;
    pub fn xcb_randr_query_version_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_version_reply_t;
    pub fn xcb_randr_select_input(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        enable: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_refresh_keyboard_mapping(
        syms: *mut xcb_key_symbols_t,
        event: *mut xcb_mapping_notify_event_t,
    ) -> ::core::ffi::c_int;
    pub fn xcb_reparent_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_reparent_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_request_check(
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t;
    pub fn xcb_screen_allowed_depths_iterator(R: *const xcb_screen_t) -> xcb_depth_iterator_t;
    pub fn xcb_send_event(
        c: *mut xcb_connection_t,
        propagate: u8,
        destination: xcb_window_t,
        event_mask: u32,
        event: *const ::core::ffi::c_char,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_input_focus(
        c: *mut xcb_connection_t,
        revert_to: u8,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_set_selection_owner(
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time_0: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_shape_combine(
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        source_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        source_window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_shape_mask(
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        source_bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_shape_query_extents(
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_query_extents_cookie_t;
    pub fn xcb_shape_query_extents_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_query_extents_reply_t;
    pub fn xcb_shape_query_version(c: *mut xcb_connection_t) -> xcb_shape_query_version_cookie_t;
    pub fn xcb_shape_query_version_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_query_version_reply_t;
    pub fn xcb_shape_rectangles(
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        ordering: u8,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_shape_select_input(
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
        enable: u8,
    ) -> xcb_void_cookie_t;
    pub fn xcb_translate_coordinates(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
    ) -> xcb_translate_coordinates_cookie_t;
    pub fn xcb_translate_coordinates_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_translate_coordinates_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_translate_coordinates_reply_t;
    pub fn xcb_ungrab_button(
        c: *mut xcb_connection_t,
        button: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_key(
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_keyboard(
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_ungrab_pointer(c: *mut xcb_connection_t, time: xcb_timestamp_t)
        -> xcb_void_cookie_t;
    pub fn xcb_ungrab_server(c: *mut xcb_connection_t) -> xcb_void_cookie_t;
    pub fn xcb_unmap_window(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    pub fn xcb_unmap_window_checked(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    pub fn xcb_visualtype_next(i: *mut xcb_visualtype_iterator_t);
    pub fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    pub fn xcb_warp_pointer(
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    ) -> xcb_void_cookie_t;
    pub fn xcb_xinerama_is_active(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t;
    pub fn xcb_xinerama_is_active_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_is_active_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_is_active_reply_t;
    pub fn xcb_xinerama_query_screens_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_query_screens_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_query_screens_reply_t;
    pub fn xcb_xinerama_query_screens_screen_info(
        R: *const xcb_xinerama_query_screens_reply_t,
    ) -> *mut xcb_xinerama_screen_info_t;
    pub fn xcb_xinerama_query_screens_screen_info_length(
        R: *const xcb_xinerama_query_screens_reply_t,
    ) -> ::core::ffi::c_int;
    pub fn xcb_xinerama_query_screens_unchecked(
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_query_screens_cookie_t;
    pub fn xcb_xkb_per_client_flags(
        c: *mut xcb_connection_t,
        deviceSpec: xcb_xkb_device_spec_t,
        change: u32,
        value: u32,
        ctrlsToChange: u32,
        autoCtrls: u32,
        autoCtrlsValues: u32,
    ) -> xcb_xkb_per_client_flags_cookie_t;
    pub fn xcb_xkb_per_client_flags_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_per_client_flags_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_per_client_flags_reply_t;
    pub fn xcb_xkb_select_events(
        c: *mut xcb_connection_t,
        deviceSpec: xcb_xkb_device_spec_t,
        affectWhich: u16,
        clear: u16,
        selectAll: u16,
        affectMap: u16,
        map: u16,
        details: *const ::core::ffi::c_void,
    ) -> xcb_void_cookie_t;
    pub fn xcb_xkb_use_extension(
        c: *mut xcb_connection_t,
        wantedMajor: u16,
        wantedMinor: u16,
    ) -> xcb_xkb_use_extension_cookie_t;
    pub fn xcb_xrm_database_free(database_0: *mut xcb_xrm_database_t);
    pub fn xcb_xrm_database_from_default(conn_0: *mut xcb_connection_t) -> *mut xcb_xrm_database_t;
    pub fn xcb_xrm_database_from_string(str: *const ::core::ffi::c_char)
        -> *mut xcb_xrm_database_t;
    pub fn xcb_xrm_resource_get_string(
        database_0: *mut xcb_xrm_database_t,
        res_name: *const ::core::ffi::c_char,
        res_class: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub static mut xcb_big_requests_id: xcb_extension_t;
    pub static mut xcb_randr_id: xcb_extension_t;
    pub static mut xcb_shape_id: xcb_extension_t;
    pub static mut xcb_xinerama_id: xcb_extension_t;
    pub static mut xcb_xkb_id: xcb_extension_t;
}

// Reuse maintained public ABI definitions wherever the native binding exposes them.
pub use as_raw_xcb_connection::xcb_connection_t;
pub use xcb::ffi::{xcb_generic_error_t, xcb_generic_event_t};
pub use xcb::x::{Char2b as xcb_char2b_t, Rectangle as xcb_rectangle_t};
