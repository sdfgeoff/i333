#!/usr/bin/env python3
"""Owned private Xvfb/fake-IPC bar status, reload and click parity.

Only spawned bar/server/status fixtures receive input or termination. No inherited
DISPLAY/socket is used. Status commands run a fixed private Python fixture, never
user configuration. Results include source fixtures and original binary hashes.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shlex
import shutil
import socket
import struct
import subprocess
import sys
import threading
import time

from Xlib import X
from Xlib.ext import xtest
from workspace_temp import make_tempdir

_spec = importlib.util.spec_from_file_location('input_parity', Path(__file__).with_name('test-input-parity.py'))
_input = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_input)

PRODUCER = r'''
import json, os, pathlib, sys, threading, time
mode=sys.argv[1]
with open('starts','ab') as log: log.write(b'start\n')
def clicks():
    for line in sys.stdin.buffer:
        with open('clicks','ab') as log: log.write(line)
threading.Thread(target=clicks,daemon=True).start()
if mode=='json':
    sys.stdout.buffer.write(b'{"version":1,"click_events":true}\n[\n');sys.stdout.buffer.flush()
else:
    # Legacy first-line detection installs the flat block without drawing it.
    sys.stdout.buffer.write(b'warmup\n');sys.stdout.buffer.flush();time.sleep(.15)
frames={
 '1':[{'full_text':'ALPHA','name':'alpha','color':'#ff0000','min_width':90,'align':'right'}],
 '2':[{'full_text':'BETA','name':'beta','color':'#00ffff','border':'#ffffff','border_left':2,'separator':False}],
 '3':[{'full_text':'LONG '*300,'short_text':'B','name':'pair'}, {'full_text':'LONG '*300,'short_text':'C','name':'pair'}],
 '4':[{'full_text':'AFTER','name':'after'}, {'full_text':'RELOAD','name':'reload'}],
}
with open('control','rb',buffering=0) as control:
    pathlib.Path('ready').touch()
    first=True
    while True:
        step=control.read(1)
        if not step: break
        key=step.decode()
        if mode=='json':
            data=(b'' if first else b',')+json.dumps(frames[key],separators=(',',':')).encode()+b'\n'
            first=False
        else: data=b'PLAIN one\r\n' if key=='1' else b'second\x00ignored\n'
        if mode=='json' and key=='2':
            for byte in data:
                sys.stdout.buffer.write(bytes([byte]));sys.stdout.buffer.flush();time.sleep(.001)
        else: sys.stdout.buffer.write(data);sys.stdout.buffer.flush()
        pathlib.Path('phase').write_text(key)
'''
TIMEOUT = 12


def wait_for(predicate, description):
    deadline = time.monotonic() + TIMEOUT
    while time.monotonic() < deadline:
        if predicate():
            return
        time.sleep(.02)
    raise AssertionError('Timed out: ' + description)


class FakeIPC:
    def __init__(self, directory, command):
        self.path = directory / 'i.sock'
        self.listener = socket.socket(socket.AF_UNIX)
        spelling = min((str(self.path), os.path.relpath(self.path)), key=lambda value: len(os.fsencode(value)))
        assert len(os.fsencode(spelling)) < 108
        self.listener.bind(spelling)
        self.listener.listen(1)
        self.listener.settimeout(TIMEOUT)
        self.client = None
        self.lock = threading.Lock()
        self.errors = []
        self.config = {'id':'fixture','mode':'dock','position':'bottom','font':'fixed',
                       'workspace_buttons':False,'status_command':command,
                       'colors':{'background':'#000000','statusline':'#ffffff'}}
        self.thread = threading.Thread(target=self.serve, daemon=True)
        self.thread.start()

    def send(self, kind, value):
        payload = json.dumps(value,separators=(',',':')).encode()
        with self.lock:
            self.client.sendall(b'i3-ipc' + struct.pack('=II',len(payload),kind) + payload)

    def serve(self):
        try:
            client, _ = self.listener.accept()
            self.client = client
            client.settimeout(TIMEOUT)
            while True:
                header = _input.receive(client,14)
                if header is None:
                    return
                magic, length, kind = struct.unpack('=6sII',header)
                assert magic == b'i3-ipc'
                payload = _input.receive(client,length) if length else b''
                assert payload is not None
                if kind == 6:
                    self.send(kind,self.config)
                elif kind == 3:
                    self.send(kind,[{'name':'screen','active':True,'primary':True,
                                    'rect':{'x':0,'y':0,'width':1024,'height':768},'current_workspace':'1'}])
                elif kind == 1:
                    self.send(kind,[])
                elif kind == 2:
                    self.send(kind,{'success':True})
                else:
                    assert kind in (0,11), ('unexpected IPC request',kind,payload)
                    self.send(kind,[{'success':True}])
        except (BrokenPipeError,ConnectionResetError,OSError):
            # An owned fixture closes this socket during bounded cleanup.
            if self.client is not None and self.client.fileno() != -1:
                self.errors.append('IPC socket failed before cleanup')
        except BaseException as error:
            self.errors.append(repr(error))

    def close(self):
        if self.client is not None:
            client,self.client = self.client,None
            try:
                client.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass
            client.close()
        self.listener.close()
        self.thread.join(timeout=3)
        assert not self.thread.is_alive(), 'fake IPC thread survived cleanup'
        self.path.unlink(missing_ok=True)


def rendered(connection, child, previous=None):
    deadline = time.monotonic() + TIMEOUT
    stable = None
    while time.monotonic() < deadline:
        assert child.poll() is None, ('bar exited before rendering',child.returncode)
        for window in _input.mapped_windows(connection.screen().root):
            geometry = window.get_geometry()
            if geometry.width == 1024 and 10 <= geometry.height < 100:
                image = window.get_image(0,0,geometry.width,geometry.height,X.ZPixmap,0xffffffff)
                colors = {image.data[index:index+4] for index in range(0,len(image.data),4)}
                result = {'geometry':[geometry.x,geometry.y,geometry.width,geometry.height],
                          'pixels_sha256':hashlib.sha256(image.data).hexdigest()}
                if len(colors)>1 and result == stable and result != previous:
                    return window,result,image.data
                stable = result if len(colors)>1 else None
        connection.sync()
        time.sleep(.05)
    raise AssertionError('bar did not complete stable foreground/text rendering')


def invoke(binary,label,mode,connection,environment,server,work):
    directory = work / mode
    directory.mkdir(exist_ok=True)
    for name in ('ready','phase','starts','clicks'):
        (directory/name).unlink(missing_ok=True)
    control = directory/'control'
    control.unlink(missing_ok=True)
    os.mkfifo(control)
    descriptor = os.open(control,os.O_RDWR)
    script = directory/'producer.py'
    script.write_text(PRODUCER)
    command = ' '.join(shlex.quote(value) for value in (sys.executable,'-u',str(script),mode))
    ipc = FakeIPC(directory,command)
    child = None
    snapshots = []
    try:
        _input.require_private(connection,environment,server)
        assert not _input.mapped_windows(connection.screen().root)
        env = environment.copy()
        env.update(HOME=str(directory),XDG_CONFIG_HOME=str(directory),TMPDIR=str(directory))
        with (work/f'{label}-{mode}.log').open('wb') as log:
            child = subprocess.Popen([str(binary),'-b','fixture','-s','i.sock'],env=env,cwd=directory,stdout=log,stderr=log)
        wait_for(lambda:(directory/'ready').exists() or child.poll() is not None,'private producer startup')
        assert child.poll() is None
        for step in ('1234' if mode=='json' else '12'):
            if step=='4':
                ipc.config = {**ipc.config,'separator_symbol':'|','colors':{'background':'#101010','statusline':'#00ff00'}}
                ipc.send(0x80000004,ipc.config)
                # Replies and output reconfiguration are processed before the
                # next status frame; snapshot requires its changed foreground.
                time.sleep(.1)
            os.write(descriptor,step.encode())
            wait_for(lambda:(directory/'phase').exists() and (directory/'phase').read_text()==step,'producer phase '+step)
            window,snapshot,pixels = rendered(connection,child,snapshots[-1] if snapshots else None)
            snapshots.append(snapshot)
            (work/f'{label}-{mode}-{step}.pixels').write_bytes(pixels)
            if mode=='json' and step=='3':
                _input.require_private(connection,environment,server)
                x,y,width,height=snapshot['geometry']
                xtest.fake_input(connection,X.MotionNotify,x=x+width-3,y=y+height//2)
                xtest.fake_input(connection,X.ButtonPress,1)
                xtest.fake_input(connection,X.ButtonRelease,1)
                connection.sync()
                wait_for(lambda:(directory/'clicks').exists() and (directory/'clicks').read_bytes().count(b'\n')>=2,'private status click')
        assert (directory/'starts').read_bytes()==b'start\n', 'reload restarted identical status command'
        clicks = (directory/'clicks').read_bytes() if mode=='json' else b''
        assert not ipc.errors,ipc.errors
        ipc.close()
        status = child.wait(timeout=TIMEOUT)
        assert status == 0, ('IPC EOF should exit normally',status)
        connection.sync()
        wait_for(lambda:not _input.mapped_windows(connection.screen().root),'bar window cleanup')
        return {'status':status,'snapshots':snapshots,'clicks_hex':clicks.hex(),'starts':1}
    finally:
        # Only Popen handles created by this fixture are signalled. Closing the
        # control pipe also lets its fixed producer exit if the bar was killed.
        os.close(descriptor)
        _input.stop(child)
        ipc.close()
        control.unlink(missing_ok=True)
        connection.sync()


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference',type=Path)
    group=parser.add_mutually_exclusive_group()
    group.add_argument('--candidate',type=Path)
    group.add_argument('--rust-bin-dir',type=Path)
    parser.add_argument('--xvfb',type=Path,default=shutil.which('Xvfb'))
    args=parser.parse_args()
    if args.xvfb is None:
        parser.error('Private Xvfb required; no desktop fallback is permitted')
    candidate=(args.candidate or (args.rust_bin_dir or _input.cargo_bin_dir())/'i3bar').resolve()
    binaries=[('candidate',candidate)]
    if args.reference:
        binaries.insert(0,('reference',args.reference.resolve()))
    work=Path(make_tempdir(prefix='bar-stream-'))
    results,hashes={},{}
    try:
        with _input.private_server(args.xvfb.resolve(),work) as (connection,environment,server):
            for label,binary in binaries:
                hashes[label]=hashlib.sha256(binary.read_bytes()).hexdigest()
                results[label]={mode:invoke(binary,label,mode,connection,environment,server,work) for mode in ('json','plain')}
            if args.reference:
                assert results['candidate']==results['reference'],results
        print('PASS: 2 private bar stream workflows, 6 rasters, reload and click framing'+(' match external C' if args.reference else ' satisfy fixtures'))
    finally:
        (work/'results.json').write_text(json.dumps({'binaries':hashes,'producer_sha256':hashlib.sha256(PRODUCER.encode()).hexdigest(),'results':results},indent=2)+'\n')
        print(f'Private bar stream artifacts: {work}')


if __name__=='__main__':
    main()
