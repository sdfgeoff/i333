#!/usr/bin/env python3
"""Opt-in titlebar controls tested exclusively on a newly allocated Xvfb display."""
import argparse
import importlib.util
from pathlib import Path

from Xlib import X, XK
from Xlib.ext import xtest

spec = importlib.util.spec_from_file_location(
    'private_headless', Path(__file__).with_name('test-headless-parity.py'))
private = importlib.util.module_from_spec(spec)
spec.loader.exec_module(private)


def find_window(tree, window):
    if tree.get('window') == window:
        return tree
    for child in tree.get('nodes', []) + tree.get('floating_nodes', []):
        found = find_window(child, window)
        if found:
            return found
    return None


def exercise(binary, xvfb):
    # Every event is sent to this explicit private connection. Ambient DISPLAY
    # is never opened, changed, queried, or used to locate an application.
    with private.session(binary, xvfb, extra_config=(
            'default_border normal 2\n'
            'bindsym Shift+button1 mark --add fixture-title-binding\n')) as (connection, ipc):
        assert connection.has_extension('XTEST'), 'private Xvfb requires XTEST'
        root = connection.screen().root
        delete = connection.intern_atom('WM_DELETE_WINDOW')
        protocols = connection.intern_atom('WM_PROTOCOLS')
        checks = []

        def barrier():
            connection.sync()
            private.request(ipc, 4)
            connection.sync()

        def command(text):
            result = private.request(ipc, 0, text)
            assert result and all(item.get('success') for item in result), (text, result)
            barrier()

        def node(window):
            return find_window(private.request(ipc, 4), window.id)

        def create(name):
            window = root.create_window(50, 50, 320, 240, 0,
                connection.screen().root_depth, X.InputOutput, X.CopyFromParent,
                background_pixel=0x242424,
                event_mask=X.StructureNotifyMask | X.ButtonPressMask | X.ButtonReleaseMask)
            window.set_wm_name(name)
            window.set_wm_class('titlebar-test', 'TitlebarTest')
            window.set_wm_protocols([delete])
            window.map()
            connection.sync()
            private.wait_for(lambda: node(window) is not None, 'private client mapping')
            return window

        def position(window, action='close'):
            value = node(window)
            assert value is not None, 'target is still managed'
            frame = window.query_tree().parent
            origin = frame.get_geometry()
            deco = value.get('actual_deco_rect', value['deco_rect'])
            size = deco['height'] - 4
            assert size >= 6, ('usable normal titlebar', value)
            x = deco['x'] + deco['width'] - 2 - size
            if action == 'fullscreen':
                x -= size + 2
            return (origin.x + x + size // 2,
                    origin.y + deco['y'] + 2 + size // 2)

        def motion(point):
            xtest.fake_input(connection, X.MotionNotify, x=point[0], y=point[1])
            barrier()

        def button(kind):
            xtest.fake_input(connection, kind, 1)
            connection.flush()

        def click(point):
            motion(point)
            button(X.ButtonPress)
            button(X.ButtonRelease)
            barrier()

        def events():
            connection.sync()
            result = []
            while connection.pending_events():
                result.append(connection.next_event())
            return result

        def deleted(window, observed):
            return any(event.type == X.ClientMessage and event.window.id == window.id
                       and event.client_type == protocols and event.data[1][0] == delete
                       for event in observed)

        def raster(window):
            value = node(window)
            deco = value.get('actual_deco_rect', value['deco_rect'])
            frame = window.query_tree().parent
            geometry = frame.get_geometry()
            # Gap offsets can leave deco_rect extending beyond the actual frame;
            # GetImage requires its requested rectangle fully inside the drawable.
            x, y = max(0, deco['x']), max(0, deco['y'])
            width = min(deco['width'], geometry.width - x)
            height = min(deco['height'], geometry.height - y)
            assert width > 0 and height > 0, (value, geometry._data)
            return frame.get_image(x, y, width, height, X.ZPixmap, 0xffffffff).data


        def reload_buttons(setting):
            config = ipc.parent / 'config'
            lines = [line for line in config.read_text().splitlines()
                     if not line.startswith('titlebar_buttons ')]
            config.write_text('\n'.join(lines) + '\ntitlebar_buttons ' + setting + '\n')
            command('reload')

        first = create('titlebar-first')
        second = create('titlebar-second')
        motion((1279, 799))
        events()
        click(position(first))
        assert not deleted(first, events()), 'default controls must be inert'
        assert node(first)['fullscreen_mode'] == 0
        checks.append('default off')

        reload_buttons('fullscreen close')
        command(f'[id="{second.id}"] focus')
        motion((1279, 799))
        idle = raster(first)
        point = position(first, 'fullscreen')
        motion(point)
        private.wait_for(lambda: raster(first) != idle, 'hover repaint')
        hover = raster(first)
        button(X.ButtonPress)
        barrier()
        assert node(first)['fullscreen_mode'] == 0, 'activation must wait for release'
        private.wait_for(lambda: raster(first) != hover, 'pressed repaint')
        button(X.ButtonRelease)
        barrier()
        private.wait_for(lambda: node(first)['fullscreen_mode'] == 1,
                         'clicked nonfocused target fullscreen')
        assert node(second)['fullscreen_mode'] == 0
        command(f'[id="{first.id}"] fullscreen disable')
        checks.append('hover/press pixels and release-target fullscreen')

        # Leaving cancels permanently for this press, including reentry.
        point = position(first, 'fullscreen')
        motion(point)
        button(X.ButtonPress)
        barrier()
        motion((point[0] - 80, point[1]))
        motion(point)
        button(X.ButtonRelease)
        barrier()
        assert node(first)['fullscreen_mode'] == 0
        checks.append('leave cancels without rearm')

        shift = connection.keysym_to_keycode(XK.string_to_keysym('Shift_L'))
        xtest.fake_input(connection, X.KeyPress, shift)
        click(position(first))
        xtest.fake_input(connection, X.KeyRelease, shift)
        barrier()
        assert 'fixture-title-binding' in private.request(ipc, 5)
        assert not deleted(first, events())
        checks.append('modified title binding preserved')

        # Normal client clicks must still be replayed to the application.
        events()
        origin = first.query_tree().parent.get_geometry()
        client_rect = node(first)['window_rect']
        click((origin.x + client_rect['x'] + 30, origin.y + client_rect['y'] + 30))
        private.wait_for(lambda: connection.pending_events() > 0, 'replayed client click')
        assert any(event.type == X.ButtonPress and event.window.id == first.id
                   for event in events())
        checks.append('client-area replay')

        events()
        motion(position(first))
        button(X.ButtonPress)
        barrier()
        assert not deleted(first, events()), 'close must wait for release'
        button(X.ButtonRelease)
        barrier()
        observed = []
        def received_delete():
            observed.extend(events())
            return deleted(first, observed)
        private.wait_for(received_delete, 'WM_DELETE target event')
        assert not deleted(second, observed)
        assert node(first) is not None and node(second) is not None
        assert first.get_attributes().map_state == X.IsViewable
        checks.append('WM_DELETE only; applications remain alive')

        for layout in ('tabbed', 'stacking'):
            command(f'layout {layout}')
            events()
            click(position(first))
            assert node(first)['fullscreen_mode'] == 0
            assert not deleted(first, events())
        command('layout splith')
        command(f'[id="{first.id}"] border pixel 2')
        events()
        # No title exists, so use the old normal-title location on the border.
        origin = first.query_tree().parent.get_geometry()
        click((origin.x + max(2, origin.width - 12), origin.y + 1))
        assert node(first)['fullscreen_mode'] == 0
        assert not deleted(first, events())
        command(f'[id="{first.id}"] border normal 2')
        checks.append('tabbed/stacked/pixel borders excluded')

        doomed = create('titlebar-doomed')
        motion(position(doomed, 'fullscreen'))
        button(X.ButtonPress)
        barrier()
        doomed.destroy()
        connection.sync()
        private.wait_for(lambda: node(doomed) is None, 'destroy pressed target')
        button(X.ButtonRelease)
        barrier()
        assert node(second) is not None and node(second)['fullscreen_mode'] == 0
        checks.append('destroyed target cannot activate replacement')

        motion((1279, 799))
        before_disabled = raster(first)
        reload_buttons('none')
        private.wait_for(lambda: raster(first) != before_disabled, 'disabled controls repaint')
        events()
        click(position(first))
        assert not deleted(first, events())
        assert node(first)['fullscreen_mode'] == 0
        checks.append('reload none removes controls')
        for label in checks:
            print('PASS titlebar: ' + label, flush=True)
        first.destroy()
        second.destroy()
        connection.sync()
    return len(checks)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', required=True, type=Path)
    parser.add_argument('--xvfb', required=True)
    args = parser.parse_args()
    print(f'{exercise(args.candidate.resolve(), args.xvfb)} private titlebar workflows passed')


if __name__ == '__main__':
    main()
