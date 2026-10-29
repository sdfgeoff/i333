#!/usr/bin/env python3
"""Optional JSON compatibility oracle using the host YAJL library, without X.

The Rust products do not use or link YAJL. This test-only Python ctypes oracle
compares the safe Rust parser/generator through a temporary Cargo probe. Legacy
whitespace and typed callback limits are enabled explicitly. The report separates
malformed-Unicode strictness, invalid-input diagnostics and numeric spellings. Missing
libyajl skips the oracle; --require-oracle turns that skip into an error.
"""
import argparse
import ctypes as c
import ctypes.util
import json
import random
from pathlib import Path
import subprocess
from workspace_temp import temporary_directory

ROOT = Path(__file__).resolve().parents[1]
PROBE = r'''
use i3_json::{Event,EventHandler,Parser,ParserOptions,ParseError,ParseErrorKind,Generator};
use serde_json::{json,Value};
use std::{io::{self,BufRead},cell::RefCell,rc::Rc};
fn unhex(s:&str)->Vec<u8>{s.as_bytes().chunks(2).map(|v|u8::from_str_radix(std::str::from_utf8(v).unwrap(),16).unwrap()).collect()}
fn hex(b:&[u8])->String{b.iter().map(|b|format!("{b:02x}")).collect()}
struct Recorder{events:Rc<RefCell<Vec<Value>>>,mode:String,error:Option<String>,cancel:usize}
impl EventHandler for Recorder{
 fn on_event(&mut self,e:Event<'_>)->bool{
  let v=match e{Event::Null=>json!(["null"]),Event::Boolean(v)=>json!(["bool",v]),Event::String(v)=>json!(["string",hex(v)]),Event::MapKey(v)=>json!(["key",hex(v)]),Event::StartMap=>json!(["map"]),Event::EndMap=>json!(["end_map"]),Event::StartArray=>json!(["array"]),Event::EndArray=>json!(["end_array"]),Event::Number(v)=>{
    if self.mode=="none"{return true}
    if self.mode=="raw"{json!(["number",v])}else if v.contains(['.','e','E']){match v.parse::<f64>(){Ok(n)if n.is_finite()=>json!(["double",n]),_=>{self.error=Some("numeric overflow".into());return false}}}else{match v.parse::<i64>(){Ok(n)if n!=i64::MIN=>json!(["integer",n]),_=>{self.error=Some("integer overflow".into());return false}}}
  }};
  self.events.borrow_mut().push(v);self.events.borrow().len()!=self.cancel
 }
 fn error_message(&self)->Option<&str>{self.error.as_deref()}
}
fn status(r:Result<(),ParseError>)->u32{match r{Ok(())=>0,Err(e)if e.kind==ParseErrorKind::Canceled=>1,_=>2}}
fn parse(v:&Value)->Value{
 let events=Rc::new(RefCell::new(vec![]));let rec=Recorder{events:events.clone(),mode:v["mode"].as_str().unwrap_or("raw").into(),error:None,cancel:v["cancel"].as_u64().unwrap_or(0)as usize};let mut p=Parser::new(rec);
 let flags=v["flags"].as_u64().unwrap_or(0);*p.options_mut()=ParserOptions{allow_comments:flags&1!=0,allow_invalid_utf8:flags&2!=0,allow_trailing_garbage:flags&4!=0,allow_multiple_values:flags&8!=0,allow_legacy_whitespace:true,allow_partial_values:flags&16!=0};
 let mut steps=vec![];let mut last=0;for chunk in v["chunks"].as_array().unwrap(){last=status(p.parse(&unhex(chunk.as_str().unwrap())));steps.push(json!({"status":last,"consumed":p.bytes_consumed(),"events":events.borrow().clone()}));if last!=0{break}}
 let finish=if v["finish"].as_bool().unwrap_or(true){status(p.finish())}else{last};json!({"steps":steps,"finish":finish,"events":events.borrow().clone()})
}
fn generate(v:&Value)->Value{
 let mut g=Generator::new();let mut states=vec![];for op in v["ops"].as_array().unwrap(){let name=op[0].as_str().unwrap();let r=match name{"array"=>g.start_array(),"end_array"=>g.end_array(),"map"=>g.start_object(),"end_map"=>g.end_object(),"null"=>g.null(),"bool"=>g.boolean(op[1].as_bool().unwrap()),"integer"=>g.integer(op[1].as_i64().unwrap()),"double"=>g.double(op[1].as_str().unwrap().parse().unwrap()),"string"=>g.string(op[1].as_str().unwrap()),"clear"=>{g.clear();Ok(())},_=>panic!()};let status=match r{Ok(())=>0,Err(i3_json::GenerateError::KeysMustBeStrings)=>1,Err(i3_json::GenerateError::DepthExceeded)=>2,Err(i3_json::GenerateError::InvalidState)=>3,Err(i3_json::GenerateError::Complete)=>4,Err(i3_json::GenerateError::InvalidNumber)=>5,Err(i3_json::GenerateError::InvalidString)=>7};states.push(json!({"status":status,"bytes":hex(g.bytes())}));}json!(states)
}
fn main(){for line in io::stdin().lock().lines(){let v:Value=serde_json::from_str(&line.unwrap()).unwrap();let out=if v.get("ops").is_some(){generate(&v)}else{parse(&v)};println!("{out}");}}
'''

CB0 = c.CFUNCTYPE(c.c_int, c.c_void_p)
CBINT = c.CFUNCTYPE(c.c_int, c.c_void_p, c.c_longlong)
CBBOOL = c.CFUNCTYPE(c.c_int, c.c_void_p, c.c_int)
CBDOUBLE = c.CFUNCTYPE(c.c_int, c.c_void_p, c.c_double)
CBBYTES = c.CFUNCTYPE(c.c_int, c.c_void_p, c.c_void_p, c.c_size_t)

class Callbacks(c.Structure):
    _fields_ = [("null", CB0), ("boolean", CBBOOL), ("integer", CBINT),
                ("double", CBDOUBLE), ("number", CBBYTES), ("string", CBBYTES),
                ("map", CB0), ("key", CBBYTES), ("end_map", CB0),
                ("array", CB0), ("end_array", CB0)]

class Oracle:
    def __init__(self, library):
        self.lib = c.CDLL(library)
        self.lib.yajl_alloc.argtypes = [c.POINTER(Callbacks), c.c_void_p, c.c_void_p]
        self.lib.yajl_alloc.restype = c.c_void_p
        self.lib.yajl_parse.argtypes = [c.c_void_p, c.c_void_p, c.c_size_t]
        self.lib.yajl_parse.restype = c.c_int
        self.lib.yajl_config.argtypes = [c.c_void_p, c.c_int]
        self.lib.yajl_config.restype = c.c_int
        self.lib.yajl_complete_parse.argtypes = [c.c_void_p]
        self.lib.yajl_complete_parse.restype = c.c_int
        self.lib.yajl_get_bytes_consumed.argtypes = [c.c_void_p]
        self.lib.yajl_get_bytes_consumed.restype = c.c_size_t
        self.lib.yajl_free.argtypes = [c.c_void_p]
        self.lib.yajl_gen_alloc.argtypes = [c.c_void_p]
        self.lib.yajl_gen_alloc.restype = c.c_void_p
        self.lib.yajl_gen_free.argtypes = [c.c_void_p]
        self.lib.yajl_gen_get_buf.argtypes = [c.c_void_p, c.POINTER(c.c_void_p), c.POINTER(c.c_size_t)]
        for name in ["array_open", "array_close", "map_open", "map_close", "null"]:
            getattr(self.lib,"yajl_gen_"+name).argtypes = [c.c_void_p]
        self.lib.yajl_gen_string.argtypes = [c.c_void_p,c.c_void_p,c.c_size_t]
        self.lib.yajl_gen_integer.argtypes = [c.c_void_p,c.c_longlong]
        self.lib.yajl_gen_double.argtypes = [c.c_void_p,c.c_double]
        self.lib.yajl_gen_bool.argtypes = [c.c_void_p,c.c_int]
        self.lib.yajl_gen_clear.argtypes = [c.c_void_p]

    def parse(self, case):
        events = []
        def event(value):
            events.append(value)
            return int(len(events) != case.get("cancel",0))
        zero = lambda name: CB0(lambda ctx: event([name]))
        byte = lambda name: CBBYTES(lambda ctx,ptr,n: event([name,c.string_at(ptr,n).hex()]))
        mode=case.get("mode","raw")
        callbacks=Callbacks(zero("null"),CBBOOL(lambda ctx,n:event(["bool",bool(n)])),
            CBINT(lambda ctx,n:event(["integer",n])) if mode=="typed" else CBINT(),
            CBDOUBLE(lambda ctx,n:event(["double",n])) if mode=="typed" else CBDOUBLE(),
            CBBYTES(lambda ctx,p,n:event(["number",c.string_at(p,n).decode("ascii")])) if mode=="raw" else CBBYTES(),
            byte("string"),zero("map"),byte("key"),zero("end_map"),zero("array"),zero("end_array"))
        handle=self.lib.yajl_alloc(c.byref(callbacks),None,None)
        for flag in [1,2,4,8,16]:
            if case.get("flags",0)&flag:
                assert self.lib.yajl_config(handle,flag,c.c_int(1))==1
        steps=[];last=0
        try:
            for chunk in case["chunks"]:
                data=bytes.fromhex(chunk)
                last=self.lib.yajl_parse(handle,data,len(data))
                steps.append(dict(status=last,consumed=self.lib.yajl_get_bytes_consumed(handle),events=list(events)))
                if last: break
            finish=self.lib.yajl_complete_parse(handle) if case.get("finish",True) else last
            return dict(steps=steps,finish=finish,events=events)
        finally:self.lib.yajl_free(handle)

    def generate(self,case):
        handle=self.lib.yajl_gen_alloc(None);states=[]
        names=dict(array="array_open",end_array="array_close",map="map_open",end_map="map_close",null="null",bool="bool",integer="integer",double="double",string="string")
        try:
            for op in case["ops"]:
                if op[0]=="clear": self.lib.yajl_gen_clear(handle);status=0
                else:
                    args=[]
                    if op[0]=="string":
                        raw=op[1].encode();args=[raw,len(raw)]
                    elif op[0]=="double":args=[float(op[1])]
                    elif len(op)>1:args=[op[1]]
                    status=getattr(self.lib,"yajl_gen_"+names[op[0]])(handle,*args)
                ptr=c.c_void_p();length=c.c_size_t()
                self.lib.yajl_gen_get_buf(handle,c.byref(ptr),c.byref(length))
                states.append(dict(status=status,bytes=c.string_at(ptr,length.value).hex()))
            return states
        finally:self.lib.yajl_gen_free(handle)


def cases():
    valid=[b'null',b'true',b'false',b'0',b'-0',b'-42',b'1.25e+10',b'1e-400',
           b'9223372036854775808',b'1e400',b'[]',b'{}',b'[null,true,false,1,"a"]',
           b'{"duplicate":1,"duplicate":2}',b'"a\\n\\t\\r\\b\\f\\/\\\\\\\""',
           b'"\\u0000\\u0041\\uD83D\\uDE00"','{"é":"😀"}'.encode(),b' \t\r\n[0] \t\r\n']
    invalid=[b'',b' ',b'\0',b'\x0bnull',b'\x0cnull',b'NaN',b'Infinity',b'-Infinity',b'undefined',
             b'[',b'{',b'"',b'"unterminated',b'"raw\nnewline"',b'"\\x00"',b'"\\uZZZZ"',
             b'"\\uD800"',b'"\\uDC00"',b'"\\uD800x"',b'"\xff"',b'"\xc0\xaf"',b'"\xed\xa0\x80"',
             b'01',b'-01',b'+1',b'.1',b'1.',b'1e',b'1e+',b'1e-',b'1..2',b'1-2',
             b'[1,]',b'[,1]',b'{"x":}',b'{"x" 1}',b'{1:2}',b'{"x":1,}',b'[]]',b'[}',b'{]',b'tru',b'truX',b'trueX',b'null false']
    count=0
    for raw in valid+invalid:
        for split in [None,*range(len(raw)+1)]:
            chunks=[raw] if split is None else [raw[:split],raw[split:]]
            for mode in ["raw","typed","none"]:
                count+=1
                yield dict(name=f"grammar-{count}",chunks=[s.hex() for s in chunks],mode=mode,strict_unicode=raw in [b'"\\uD800"',b'"\\uDC00"',b'"\\uD800x"',b'"\xc0\xaf"',b'"\xed\xa0\x80"'])
    special=[(b'/*c*/[1,//line\n2]',1,True),(b'//end',1,True),(b'/*unfinished',1,True),
             (b'null true {} [] 1',8,True),(b'nulltrue',8,True),(b'01',8,True),
             (b'{"version":1}\n[[{"full_text":"a"}]]',4,False),
             (b'[[],[{"full_text":"a"}],[{"full_text":"b"}]',0,False),
             (b'[[],[{"full_text":"a"}],[{"full_text":"b"}]',16,True),
             (b'"\xff"',2,True),(b'"\xc0\xaf"',2,True),(b'"\xed\xa0\x80"',2,True),(b'"\\uDC00"',2,True),(b'[1,',16,True),
             (b'"unfinished',16,True),(b'1e',16,True)]
    for n,(raw,flags,finish) in enumerate(special):
        for split in [None,*range(len(raw)+1)]:
            chunks=[raw] if split is None else [raw[:split],raw[split:]]
            yield dict(name=f"flags-{n}-{split}",chunks=[s.hex() for s in chunks],flags=flags,finish=finish,mode="raw",strict_unicode=raw==b'"\\uDC00"')
    rng=random.Random(0x1333)
    def value(depth=0):
        if depth < 4 and rng.randrange(3) == 0:
            if rng.randrange(2):
                return [value(depth+1) for _ in range(rng.randrange(5))]
            return {str(index)+rng.choice(["é","key","😀"]):value(depth+1) for index in range(rng.randrange(5))}
        return rng.choice([None,False,True,0,-1,2**63-1,-2**63,2**63,
                           0.1,-0.0,1e-200,1e200,"","text","a\0b","line\ntext","é😀","\\\"/\t"])
    for n in range(128):
        raw=json.dumps(value(),ensure_ascii=bool(n%2),separators=(",", ":")).encode()
        boundaries=sorted(rng.randrange(len(raw)+1) for _ in range(4))
        chunks=[raw[a:b] for a,b in zip([0]+boundaries,boundaries+[len(raw)])]
        for mode in ["raw","typed","none"]:
            yield dict(name=f"generated-valid-{n}-{mode}",chunks=[chunk.hex() for chunk in chunks],mode=mode)
    for depth in [1,2,10,127,128,129,256]:
        raw=b"["*depth+b"0"+b"]"*depth
        yield dict(name=f"nested-valid-{depth}",chunks=[raw.hex()])
    updates=[b"["]
    for n in range(20):
        update=json.dumps([dict(full_text=f"status {n} é😀",urgent=bool(n%2))],ensure_ascii=False,separators=(",", ":")).encode()
        updates.append((b"," if n else b"")+update)
    yield dict(name="open-status-updates",chunks=[chunk.hex() for chunk in updates],finish=False)
    for n in range(1,9):
        yield dict(name=f"cancel-{n}",chunks=[b'{"x":[null,true,1,"a"]}'.hex()],cancel=n)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--offline",action="store_true")
    parser.add_argument("--require-oracle",action="store_true")
    parser.add_argument("--report",type=Path)
    args=parser.parse_args()
    library=ctypes.util.find_library("yajl")
    if not library:
        print("SKIP: optional libyajl oracle is not installed")
        return int(args.require_oracle)
    oracle=Oracle(library)
    parse_cases=list(cases())
    gen_cases=[dict(name="generator-stream",ops=[["array"],["clear"],["map"],["string","x"],["string","a\n\t\0😀"],["end_map"],["clear"],["map"],["string","n"],["integer",-42],["end_map"],["end_array"]]),
               dict(name="generator-values",ops=[["array"],["null"],["bool",True],["bool",False],["double","1.25"],["integer",9223372036854775807],["end_array"]]),
               dict(name="generator-retry-nonfinite",ops=[["array"],["double","NaN"],["null"],["end_array"]]),
               dict(name="generator-retry-map-key",ops=[["map"],["integer",1],["string","key"],["bool",True],["end_map"]]),
               *[dict(name="generator-double-"+v,ops=[["double",v]]) for v in ["0.0","-0.0","0.1","0.3333333333333333","1e100","1e-100","1.7976931348623157e308"]],
               *[dict(name="generator-nonfinite-"+v,ops=[["double",v]]) for v in ["NaN","inf","-inf"]]]
    all_cases=parse_cases+gen_cases
    with temporary_directory(prefix="i333-json-oracle-") as directory:
        directory=Path(directory);(directory/"src").mkdir()
        (directory/"Cargo.toml").write_text('[package]\nname="i333-json-oracle"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[dependencies]\ni3-json={path='+json.dumps(str(ROOT/"crates/i3-json"))+'}\nserde_json="=1.0.154"\n')
        (directory/"src/main.rs").write_text(PROBE)
        command=["cargo","build","--quiet","--manifest-path",str(directory/"Cargo.toml")]
        if args.offline:command.append("--offline")
        subprocess.run(command,check=True,cwd=ROOT)
        run=subprocess.run([str(directory/"target/debug/i333-json-oracle")],input=''.join(json.dumps(case)+'\n' for case in all_cases),text=True,capture_output=True,check=True)
    outputs=[json.loads(line) for line in run.stdout.splitlines()]
    differences=[]
    diagnostics=[]
    strict_unicode=[]
    number_format=[]
    for case,actual in zip(all_cases,outputs,strict=True):
        expected=oracle.generate(case) if "ops" in case else oracle.parse(case)
        if actual == expected:
            continue
        difference=dict(case=case,rust=actual,yajl=expected)
        if "ops" not in case and case.get("strict_unicode") and actual["finish"] == 2 and expected["finish"] == 0:
            strict_unicode.append(difference)
        elif "ops" not in case and actual["finish"] == expected["finish"] == 2 and actual["events"] == expected["events"]:
            diagnostics.append(difference)
        elif case["name"].startswith("generator-double-") and all(a["status"] == b["status"] for a,b in zip(actual,expected)):
            try:
                equivalent=all(json.loads(bytes.fromhex(a["bytes"])) == json.loads(bytes.fromhex(b["bytes"])) for a,b in zip(actual,expected))
            except (ValueError,UnicodeError):
                equivalent=False
            (number_format if equivalent else differences).append(difference)
        else:
            differences.append(difference)
    report=dict(cases=len(all_cases),differences=differences,invalid_input_diagnostic_progress=diagnostics,deliberate_strict_unicode=strict_unicode,equivalent_number_format=number_format)
    if args.report:args.report.write_text(json.dumps(report,indent=2,allow_nan=False)+'\n')
    print(f'{len(all_cases)} JSON oracle cases; {len(differences)} unexpected differences; '
          f'{len(diagnostics)} invalid-input diagnostic differences; '
          f'{len(strict_unicode)} deliberate Unicode strictness cases; '
          f'{len(number_format)} equivalent number encodings')
    for difference in differences[:12]:print(json.dumps(difference,ensure_ascii=True))
    return bool(differences)

if __name__=="__main__":raise SystemExit(main())
