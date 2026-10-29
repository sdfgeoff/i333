#!/usr/bin/env python3
"""Compare the real status-header parser with upstream's native YAJL callbacks.

Plain cargo test supplies its already-built probe. Standalone runs can build
that same integration target, with no temporary copied implementation.
"""
import argparse
import ctypes as c
import importlib.util
import pathlib
import signal
import subprocess
import sys
ROOT=pathlib.Path(__file__).resolve().parents[1]
from cargo_probe import build_probe
spec=importlib.util.spec_from_file_location('oracle',ROOT/'scripts/test-json-parity.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
library=c.util.find_library('yajl')
if not library:
 raise SystemExit('libyajl is required for status-header compatibility tests; install the native test oracle dependency')
o=m.Oracle(library)
libc=c.CDLL(None);libc.strncasecmp.argtypes=[c.c_void_p,c.c_char_p,c.c_size_t];libc.strncasecmp.restype=c.c_int

def native(raw):
 state=[0,int(signal.SIGSTOP),int(signal.SIGCONT),None,0];key=None
 def integer(ctx,value):
  if key=='version': state[0]=c.c_uint32(value).value
  elif key=='stop_signal': state[1]=c.c_int(value).value
  elif key=='cont_signal': state[2]=c.c_int(value).value
  return 1
 def boolean(ctx,value):
  if key=='click_events':state[3]=str(int(bool(value)))
  return 1
 def map_key(ctx,ptr,size):
  nonlocal key
  for text in ['version','stop_signal','cont_signal','click_events']:
   if size==len(text) and libc.strncasecmp(ptr,text.encode(),size)==0:key=text;break
  return 1
 callbacks=m.Callbacks(m.CB0(),m.CBBOOL(boolean),m.CBINT(integer),m.CBDOUBLE(),m.CBBYTES(),m.CBBYTES(),m.CB0(),m.CBBYTES(map_key),m.CB0(),m.CB0(),m.CB0())
 handle=o.lib.yajl_alloc(c.byref(callbacks),None,None)
 try:
  assert o.lib.yajl_config(handle,4,c.c_int(1))==1
  status=o.lib.yajl_parse(handle,raw,len(raw))
  if status:state[:3]=[0,int(signal.SIGSTOP),int(signal.SIGCONT)]
  else:state[4]=o.lib.yajl_get_bytes_consumed(handle)
 finally:o.lib.yajl_free(handle)
 return ' '.join(str(v) if v is not None else '-' for v in state)

corpus=[b'',b'plain text\n',b'null',b'123',b'{}[[{',b'{"version":1}[[{"full_text":"text"}]]',b'{"version":1,"click_events":true,"stop_signal":0,"cont_signal":10}',b'{"VERSION":1,"unknown":7,"click_events":true,"unknown":false}',b'{"outer":{"version":2},"unknown":3}',b'{"version":1,"stop_signal":4294967297,"cont_signal":-4294967297}',b'{"click_events":true,"version":-9223372036854775808}',b'{"click_events":false,"version":9223372036854775808}',b'{"version":1e999,"click_events":true}',b'{"version":1.5}',b'{"version":null,"unknown":3}',b'{"ver\\u0000ion":1}',b'{"version":1,"unknown":[2,3]}',b'\x0b{"version":1}\x0c[',b'{"version":1,"stop_signal":"ignored","unknown":-2}']
cases=[]
for raw in corpus:
 cases.extend(raw[:split] for split in range(len(raw)+1))
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--probe',type=pathlib.Path,help='Use an already-built native_header executable; do not invoke Cargo')
parser.add_argument('--offline',action='store_true')
args=parser.parse_args()
probe=args.probe.resolve() if args.probe else build_probe('i3bar','native_header',args.offline)
process=subprocess.run([str(probe), '--probe'],input=''.join(raw.hex()+'\n' for raw in cases),text=True,capture_output=True,check=True,timeout=60)
actual=process.stdout.splitlines()
assert len(actual)==len(cases)
for raw,rust in zip(cases,actual):
 expected=native(raw)
 assert rust==expected,(raw,rust,expected)
print(f'PASS: {len(cases)} header/prefix comparisons against native YAJL callbacks')
