# Contributing

## i3status/i3lock bug reports and feature requests

Note that bug reports and feature requests for related projects should be filed in the corresponding repositories for [i3status](https://github.com/i3/i3status) and [i3lock](https://github.com/i3/i3lock).

## AI contribution policy

i3 accepts contributions created with AI assistance.

Contributions must disclose AI assistance,
e.g. “Created with help by Claude Opus 4.5”.

By submitting such a contribution, you represent that:

1. You have reviewed and understood the code you are submitting.
2. You have made sufficient modifications or creative decisions such that
   you can offer the contribution under the BSD-3-Clause license.
3. To your knowledge, the contribution does not reproduce
   copyrighted material from other projects.

Here are a few examples for illustration:

1. A purely AI-generated pull request (“vibe-coded”) cannot be accepted, because
  the copyright status of such output is uncertain and you may not be able to
  license it under BSD-3-Clause.
2. It is considered impolite to send an LLM’s writing (posting “slop” issues). We
  want to hear your own voice and see that you have spent enough thoughts and
  time to write us a message.
3. It can be very effective to ask a coding assistant to confirm and explain a
  bug, but then verify correctness of the generated explanation by implementing
  and testing your own solution. This process saves you time for the mechanical
  steps, but ensures you understood the problem and solution.

## i3 bug reports

1. Read the [debugging instructions](https://i3wm.org/docs/debugging.html).
2. Make sure you include a link to your logfile in your report (section 3).
3. Make sure you include the i3 version number in your report (section 1).
4. Please be aware that we cannot support compatibility issues with
   closed-source software, as digging into compatibility problems without
   having access to the source code is too time-consuming. Additionally,
   experience has shown that often, the software in question is responsible for
   the issue. Please raise an issue with the software in question, not i3.
5. Please note that i3 does not support compositors (e.g. compton). If you
   encountered the issue you are about to report while using a compositor,
   please try reproducing it without a compositor.

## i3 feature requests

1. Read the [project goals](https://i3wm.org) on the website and make sure that
   they are compatible with the feature you want to suggest.
2. We are generally happy with the current feature set of i3 and instead focus
   on maintenance such as stability and fixing bugs. New features will rarely
   be considered if they require additional configuration and/or commands, or
   if they add significant complexity (either through the exposed configuration
   or mental complexity) to the project.
3. Explain in detail what problem the feature addresses and why existing
   features fall short.
4. Consider whether the feature could instead be implemented using the
   [IPC](https://i3wm.org/docs/ipc.html) or other external tooling.

## Working on the Rust port

1. Build with `cargo build --workspace --locked` and `cargo build --workspace --locked --release`.
   See [repository details](../docs/development.md) for native dependencies.
2. Preserve i3 behavior, existing function boundaries, native layouts and FFI
   ownership contracts. Explain intentional behavior changes in the pull request.
3. Run `cargo test --workspace --locked`, then
   `python3 scripts/test-cargo-build.py` to verify a fresh standalone build.
   Add focused regression coverage for behavior changes; use the IPC/headless
   runners described in [PORTING.md](../PORTING.md).
4. Runtime tests must use private sockets and headless X servers. Never restart
   or send commands/input to a developer's active desktop session.
5. C sources and build files are no longer maintained in this checkout. Use the
   exact upstream revision in the porting notes for external source comparisons
   or regression helpers. Retain original copyright and license notices.
6. Describe the problem, resulting behavior and validation. Keep architecture
   changes separate from compatibility fixes where practical.
