//! Typed IPC values. Serde handles normal JSON; the small event fallback keeps
//! i3's historical treatment of invalid native bytes and non-finite numbers.
#![forbid(unsafe_code)]
use i3_json::Generator;
use serde::{Serialize, Serializer};
use std::ffi::CStr;

pub(super) trait WireValue: Serialize {
    fn valid(&self) -> bool {
        true
    }
    fn depth(&self) -> usize {
        0
    }
    fn omitted(&self) -> bool {
        false
    }
    fn legacy(&self, output: &mut Generator);
    fn write(&self, output: &mut Generator) {
        // Decide before touching the generator: a failed depth check poisons
        // its state, whereas the old event sequence retains a partial prefix.
        if self.valid() && output.nesting_depth().saturating_add(self.depth()) <= 128 {
            let _ = output.serialized(self);
        } else {
            self.legacy(output);
        }
    }
}

/// Native text keeps its original bytes until the JSON boundary. Invalid UTF-8
/// is rejected without consuming a value, rather than repaired or made null.
#[derive(Clone, Debug)]
pub(super) struct NativeText(Vec<u8>);
impl NativeText {
    pub fn bytes(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
    pub fn cstr(value: &CStr) -> Self {
        Self::bytes(value.to_bytes())
    }
}
impl Serialize for NativeText {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(std::str::from_utf8(&self.0).map_err(serde::ser::Error::custom)?)
    }
}
impl WireValue for NativeText {
    fn valid(&self) -> bool {
        std::str::from_utf8(&self.0).is_ok()
    }
    fn legacy(&self, output: &mut Generator) {
        if let Ok(value) = std::str::from_utf8(&self.0) {
            let _ = output.string(value);
        }
    }
}
impl WireValue for bool {
    fn legacy(&self, output: &mut Generator) {
        let _ = output.boolean(*self);
    }
}
impl WireValue for i64 {
    fn legacy(&self, output: &mut Generator) {
        let _ = output.integer(*self);
    }
}
impl WireValue for f64 {
    fn valid(&self) -> bool {
        self.is_finite()
    }
    fn legacy(&self, output: &mut Generator) {
        let _ = output.double(*self);
    }
}
impl WireValue for &'static str {
    fn legacy(&self, output: &mut Generator) {
        let _ = output.string(self);
    }
}
impl<T: WireValue> WireValue for Option<T> {
    fn valid(&self) -> bool {
        self.as_ref().is_none_or(WireValue::valid)
    }
    fn depth(&self) -> usize {
        self.as_ref().map_or(0, WireValue::depth)
    }
    fn legacy(&self, output: &mut Generator) {
        if let Some(value) = self {
            value.legacy(output);
        } else {
            let _ = output.null();
        }
    }
}
impl<T: WireValue> WireValue for Vec<T> {
    fn valid(&self) -> bool {
        self.iter().all(WireValue::valid)
    }
    fn depth(&self) -> usize {
        1 + self.iter().map(WireValue::depth).max().unwrap_or(0)
    }
    fn legacy(&self, output: &mut Generator) {
        let _ = output.start_array();
        for value in self {
            value.legacy(output);
        }
        let _ = output.end_array();
    }
}
/// A missing field is distinct from a present JSON null.
#[derive(Clone, Debug, Default)]
pub(super) struct Omitted<T>(pub Option<T>);
impl<T> Omitted<T> {
    pub fn is_omitted(&self) -> bool {
        self.0.is_none()
    }
}
impl<T: Serialize> Serialize for Omitted<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
impl<T: WireValue> WireValue for Omitted<T> {
    fn valid(&self) -> bool {
        self.0.valid()
    }
    fn depth(&self) -> usize {
        self.0.depth()
    }
    fn omitted(&self) -> bool {
        self.is_omitted()
    }
    fn legacy(&self, output: &mut Generator) {
        self.0.legacy(output);
    }
}
// Each schema is declared once: Serde and the exceptional compatibility path
// share field order and omission rules without a second hand-maintained builder.
macro_rules! wire_object {
    ($name:ident { $( $(#[$attr:meta])* $field:ident : $kind:ty $(=> $key:literal)? ),* $(,)? }) => {
        #[derive(Serialize)]
        pub(super) struct $name { $( $(#[$attr])* $(#[serde(rename = $key)])? pub $field: $kind, )* }
        impl WireValue for $name {
            fn valid(&self) -> bool { true $(&& self.$field.valid())* }
            fn depth(&self) -> usize { 1 + [0 $(, self.$field.depth())*].into_iter().max().unwrap() }
            fn legacy(&self, output: &mut Generator) {
                let _ = output.start_object();
                $(if !self.$field.omitted() {
                    let _ = output.string(wire_object!(@key $field $(, $key)?));
                    self.$field.legacy(output);
                })*
                let _ = output.end_object();
            }
        }
    };
    (@key $field:ident, $key:literal) => { $key };
    (@key $field:ident) => { stringify!($field).strip_prefix("r#").unwrap_or(stringify!($field)) };
}
wire_object!(Shutdown { change: &'static str });
wire_object!(BindingState { name: NativeText });
wire_object!(Version {
    major: i64,
    minor: i64,
    patch: i64,
    human_readable: NativeText,
    loaded_config_file_name: NativeText,
    included_config_file_names: Vec<NativeText>,
});
wire_object!(Tick {
    first: bool,
    payload: NativeText
});
wire_object!(Binding {
    input_code: i64,
    input_type: &'static str,
    symbol: Option<NativeText>,
    command: NativeText,
    mods: Vec<&'static str>,
    event_state_mask: Vec<&'static str>,
});
wire_object!(BindingEvent {
    change: NativeText,
    mode: NativeText,
    binding: Binding
});
wire_object!(Rectangle {
    x: i64,
    y: i64,
    width: i64,
    height: i64
});
wire_object!(Workspace {
    id: i64,
    num: i64,
    name: NativeText,
    visible: bool,
    focused: bool,
    rect: Rectangle,
    output: NativeText,
    urgent: bool,
});
wire_object!(Output {
    name: NativeText,
    active: bool,
    primary: bool,
    rect: Rectangle,
    current_workspace: Option<NativeText>,
});
wire_object!(IncludedConfig {
    path: NativeText,
    raw_contents: NativeText,
    variable_replaced_contents: NativeText,
});
wire_object!(Config { config: NativeText, included_configs: Vec<IncludedConfig> });

#[cfg(test)]
mod tests {
    use super::*;
    fn bytes<T: WireValue>(value: &T) -> Vec<u8> {
        let mut output = Generator::new();
        value.write(&mut output);
        output.bytes().to_vec()
    }
    wire_object!(ConditionalNames {
        r#type: NativeText,
        #[serde(skip_serializing_if = "Omitted::is_omitted")]
        optional: Omitted<i64>,
        renamed: bool => "wire_name",
    });
    #[test]
    fn renamed_raw_and_omitted_fields_match_both_paths() {
        for optional in [Omitted(None), Omitted(Some(2))] {
            let value = ConditionalNames {
                r#type: NativeText::bytes(b"con"),
                optional,
                renamed: true,
            };
            let typed = bytes(&value);
            let mut legacy = Generator::new();
            value.legacy(&mut legacy);
            assert_eq!(typed, legacy.bytes());
            assert!(std::str::from_utf8(&typed)
                .unwrap()
                .contains("\"type\":\"con\""));
            assert!(std::str::from_utf8(&typed)
                .unwrap()
                .contains("\"wire_name\":true"));
        }
        assert_eq!(
            bytes(&ConditionalNames {
                r#type: NativeText::bytes(b"\xff"),
                optional: Omitted(None),
                renamed: true
            }),
            br#"{"type":"wire_name"}"#
        );
    }
    #[test]
    fn ordinary_message_schemas_keep_order_nulls_and_unsigned_rectangles() {
        let output = Output {
            name: NativeText::bytes(b"HDMI-1"),
            active: true,
            primary: false,
            rect: Rectangle {
                x: u32::MAX as i64,
                y: 0,
                width: 1920,
                height: 1080,
            },
            current_workspace: None,
        };
        assert_eq!(bytes(&output), br#"{"name":"HDMI-1","active":true,"primary":false,"rect":{"x":4294967295,"y":0,"width":1920,"height":1080},"current_workspace":null}"#);
        let binding = Binding {
            input_code: 0,
            input_type: "keyboard",
            symbol: Some(NativeText::bytes(b"")),
            command: NativeText::bytes(b"nop \"text\""),
            mods: vec!["shift", "Mod4"],
            event_state_mask: vec!["shift", "Mod4"],
        };
        assert_eq!(bytes(&binding), br#"{"input_code":0,"input_type":"keyboard","symbol":"","command":"nop \"text\"","mods":["shift","Mod4"],"event_state_mask":["shift","Mod4"]}"#);
    }
    #[test]
    fn tick_payload_keeps_explicit_nul_and_invalid_utf8_partial_output() {
        assert_eq!(
            bytes(&Tick {
                first: false,
                payload: NativeText::bytes(b"a\0b\n")
            }),
            br#"{"first":false,"payload":"a\u0000b\n"}"#
        );
        assert_eq!(
            bytes(&Tick {
                first: false,
                payload: NativeText::bytes(b"\xff")
            }),
            br#"{"first":false,"payload""#
        );
        assert_eq!(
            bytes(&vec![
                NativeText::bytes(b"first"),
                NativeText::bytes(b"\xff"),
                NativeText::bytes(b"first")
            ]),
            br#"["first","first"]"#
        );
    }
    #[test]
    fn invalid_native_fields_keep_following_key_value_state() {
        let config = Config {
            config: NativeText::bytes(b"\xff"),
            included_configs: vec![],
        };
        // An invalid string does not consume the value after "config". The
        // following field name fills it; its array is then rejected as a key.
        assert_eq!(bytes(&config), br#"{"config":"included_configs"}"#);
        let version = Version {
            major: 4,
            minor: 25,
            patch: 0,
            human_readable: NativeText::bytes(b"\xff"),
            loaded_config_file_name: NativeText::bytes(b"path"),
            included_config_file_names: vec![],
        };
        assert_eq!(bytes(&version), br#"{"major":4,"minor":25,"patch":0,"human_readable":"loaded_config_file_name","path":"included_config_file_names"}"#);
    }
    #[test]
    fn serde_and_event_paths_agree_for_complete_normal_messages() {
        let value = Config {
            config: NativeText::bytes(b"font monospace 8\n"),
            included_configs: vec![IncludedConfig {
                path: NativeText::bytes(b"path"),
                raw_contents: NativeText::bytes(b"raw"),
                variable_replaced_contents: NativeText::bytes(b"replaced"),
            }],
        };
        let normal = bytes(&value);
        let mut events = Generator::new();
        value.legacy(&mut events);
        assert_eq!(normal, events.bytes());
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&normal).unwrap()["included_configs"][0]
                ["path"],
            "path"
        );
    }
    #[test]
    fn depth_fallback_keeps_legacy_prefix_and_nonfinite_values_stay_rejected() {
        let mut output = Generator::new();
        for _ in 0..128 {
            output.start_array().unwrap();
        }
        let before = output.bytes().to_vec();
        Tick {
            first: false,
            payload: NativeText::bytes(b"text"),
        }
        .write(&mut output);
        assert_eq!(output.bytes(), before);
        assert_eq!(
            output.boolean(true),
            Err(i3_json::GenerateError::InvalidState)
        );
        // Matching closes still close a parent after a sticky error.
        assert!(output.end_array().is_ok());
        let mut output = Generator::new();
        output.start_array().unwrap();
        f64::NAN.write(&mut output);
        f64::NEG_INFINITY.write(&mut output);
        (-0.0_f64).write(&mut output);
        output.end_array().unwrap();
        assert_eq!(output.bytes(), b"[-0.0]");
    }
}
