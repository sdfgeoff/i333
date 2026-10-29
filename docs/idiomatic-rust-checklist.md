# Remaining-file Rust review

51 of 66 files reviewed; 15 remain.

Checked entries were individually reviewed and modernized where needed. Native ABI and intrusive-tree boundaries remain explicit. Detailed findings and tests are in `idiomatic-rust-checklist.json`.

1. [x] `crates/i3/build.rs` — reviewed idiomatic
2. [x] `crates/i3/src/application.rs` — modernized
3. [ ] `crates/i3/src/bindings.rs` — pending
4. [ ] `crates/i3/src/click.rs` — pending
5. [x] `crates/i3/src/commands.rs` — modernized
6. [x] `crates/i3/src/commands_parser.rs` — modernized
7. [ ] `crates/i3/src/con.rs` — pending
8. [x] `crates/i3/src/config.rs` — modernized
9. [x] `crates/i3/src/config_directives.rs` — modernized
10. [x] `crates/i3/src/config_parser.rs` — modernized
11. [ ] `crates/i3/src/drag.rs` — pending
12. [ ] `crates/i3/src/floating.rs` — pending
13. [ ] `crates/i3/src/handlers.rs` — pending
14. [x] `crates/i3/src/ipc.rs` — modernized
15. [x] `crates/i3/src/load_layout.rs` — modernized
16. [x] `crates/i3/src/main.rs` — reviewed native
17. [ ] `crates/i3/src/manage.rs` — pending
18. [ ] `crates/i3/src/move.rs` — pending
19. [x] `crates/i3/src/parser_util.rs` — modernized
20. [x] `crates/i3/src/randr.rs` — modernized
21. [x] `crates/i3/src/regex.rs` — reviewed idiomatic
22. [ ] `crates/i3/src/render.rs` — pending
23. [ ] `crates/i3/src/resize.rs` — pending
24. [x] `crates/i3/src/restore_layout.rs` — modernized
25. [ ] `crates/i3/src/scratchpad.rs` — pending
26. [x] `crates/i3/src/sighandler.rs` — modernized
27. [x] `crates/i3/src/startup.rs` — modernized
28. [ ] `crates/i3/src/tiling_drag.rs` — pending
29. [ ] `crates/i3/src/tree.rs` — pending
30. [x] `crates/i3/src/types.rs` — reviewed native
31. [x] `crates/i3/src/util.rs` — modernized
32. [x] `crates/i3/src/version.rs` — modernized
33. [ ] `crates/i3/src/workspace.rs` — pending
34. [ ] `crates/i3/src/x.rs` — pending
35. [x] `crates/i3/src/xcb.rs` — modernized
36. [x] `crates/i3/tests/xkb_native.rs` — modernized
37. [x] `crates/i3-build/src/lib.rs` — reviewed idiomatic
38. [x] `crates/i3-common/src/safewrappers.rs` — modernized
39. [x] `crates/i3-common/src/strndup.rs` — removed
40. [x] `crates/i3-json/src/compat.rs` — reviewed native
41. [x] `crates/i3-json/src/generator.rs` — reviewed idiomatic
42. [x] `crates/i3-json/src/lib.rs` — reviewed idiomatic
43. [x] `crates/i3-json/src/parser.rs` — reviewed idiomatic
44. [x] `crates/i3-tools/build.rs` — reviewed idiomatic
45. [x] `crates/i3-tools/src/bin/i3-config-wizard/application.rs` — modernized
46. [x] `crates/i3-tools/src/bin/i3-config-wizard/main.rs` — reviewed native
47. [x] `crates/i3-tools/src/bin/i3-dump-log/application.rs` — reviewed idiomatic
48. [x] `crates/i3-tools/src/bin/i3-dump-log/main.rs` — reviewed idiomatic
49. [x] `crates/i3-tools/src/bin/i3-input/keysym2ucs.rs` — reviewed idiomatic
50. [x] `crates/i3-tools/src/bin/i3-input/main.rs` — reviewed native
51. [x] `crates/i3-tools/src/bin/i3-msg/application.rs` — reviewed idiomatic
52. [x] `crates/i3-tools/src/bin/i3-msg/main.rs` — reviewed idiomatic
53. [x] `crates/i3-tools/src/bin/i3-nagbar/main.rs` — reviewed native
54. [x] `crates/i3bar/build.rs` — reviewed idiomatic
55. [x] `crates/i3bar/src/application.rs` — modernized
56. [x] `crates/i3bar/src/child.rs` — modernized
57. [x] `crates/i3bar/src/main.rs` — reviewed native
58. [x] `crates/i3bar/src/xcb.rs` — modernized
59. [x] `crates/libev-sys/build.rs` — reviewed idiomatic
60. [x] `crates/libev-sys/src/lib.rs` — reviewed native
61. [x] `crates/startup-notification-sys/build.rs` — reviewed idiomatic
62. [x] `crates/startup-notification-sys/src/lib.rs` — reviewed native
63. [x] `crates/startup-notification-sys/tests/native.rs` — reviewed native
64. [x] `crates/xcb-sys/build.rs` — reviewed idiomatic
65. [x] `crates/xcb-sys/src/lib.rs` — reviewed native
66. [x] `crates/xcb-sys/tests/native.rs` — reviewed native
