"""Build the bounded SCENARIUM demo, native references and inspectable source custody."""
import argparse, hashlib, html, json, os, shutil, subprocess, zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]

def capture(*args):return subprocess.check_output(args,cwd=ROOT).decode('utf-8').strip()
def run(*args):subprocess.run(args,cwd=ROOT,check=True)
def pin(path):return {'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,default=Path('dist/SCENARIUM'));a=p.parse_args()
 output=(ROOT/a.output).resolve();dist=(ROOT/'dist').resolve()
 if output==dist or not output.is_relative_to(dist):raise ValueError('Output must be a child of dist')
 if capture('wasm-bindgen','--version')!='wasm-bindgen 0.2.127':raise ValueError('Use wasm-bindgen 0.2.127')
 run('cargo','+1.95.0','build','--locked','--release','--target','wasm32-unknown-unknown','--manifest-path','web/adapter/Cargo.toml','--lib','--features','wasm')
 run('cargo','+1.95.0','build','--locked','--manifest-path','web/adapter/Cargo.toml','--bin','scenarium-compare')
 metadata=json.loads(capture('cargo','+1.95.0','metadata','--manifest-path','web/adapter/Cargo.toml','--locked','--format-version','1','--filter-platform','wasm32-unknown-unknown','--features','wasm'))
 if output.exists():
  def linked(path):return path.is_symlink() or bool(getattr(path.lstat(),'st_file_attributes',0)&1024)
  if linked(output) or any(linked(p) for p in output.rglob('*')):raise ValueError('Linked output refused')
  shutil.rmtree(output)
 output.mkdir(parents=True)
 target=Path(metadata['target_directory']);run('wasm-bindgen',str(target/'wasm32-unknown-unknown/release/scenarium_web.wasm'),'--target','web','--out-dir',str(output/'pkg'))
 binary=target/'debug'/('scenarium-compare.exe' if os.name=='nt' else 'scenarium-compare')
 examples=json.loads(capture(str(binary),'--examples'));sample=examples['mixed']
 page=(ROOT/'web/index.html').read_text(encoding='utf-8')
 for side in ('baseline','candidate'):page=page.replace('{{'+side.upper()+'}}',html.escape(sample[side+'_json']))
 page=page.replace('{{EXAMPLES}}',json.dumps(examples,ensure_ascii=False).replace('<','\\u003c'))
 (output/'index.html').write_text(page,encoding='utf-8')
 for name in ('app.js','worker.js','style.css'):shutil.copyfile(ROOT/'web'/name,output/name)
 for name in ('LICENSE','BUILD.md'):shutil.copyfile(ROOT/name,output/name)
 shutil.copytree(ROOT/'tools/licenses/rust',output/'rust')
 (output/'examples.json').write_text(json.dumps(examples,indent=2)+'\n',encoding='utf-8')
 # Include notices for the portable adapter's actual resolved dependency closure,
 # including build-time proc macros, without adding consumer dependencies.
 packages={p['id']:p for p in metadata['packages']};nodes={n['id']:n for n in metadata['resolve']['nodes']}
 first=next(p['id'] for p in metadata['packages'] if p['name']=='scenarium-web');pending=[first];visited=set();notices=[];dependencies=[]
 while pending:
  current=pending.pop()
  if current in visited:continue
  visited.add(current);pending.extend(nodes[current]['dependencies']);pkg=packages[current]
  if pkg['source'] is None:continue
  base=Path(pkg['manifest_path']).parent
  files=sorted({f for pattern in ('LICENSE*','COPYRIGHT*','NOTICE*') for f in base.glob(pattern) if f.is_file()})
  if not files:raise ValueError('No license notice found for '+pkg['name'])
  dependencies.append({'name':pkg['name'],'version':pkg['version'],'license':pkg['license']})
  notices.append('\n\n===== '+pkg['name']+' '+pkg['version']+' ('+str(pkg['license'])+') =====\n')
  for f in files:notices.append('\n--- '+f.name+' ---\n'+f.read_text(encoding='utf-8'))
 notice='SCENARIUM portable adapter dependency closure, including build-time proc macros.\nThe record core and portable adapter use no filesystem or network dependencies. Original licenses follow.\nRust standard-library notices: rust/COPYRIGHT-library.html and rust/licenses/.\n'+''.join(notices)
 (output/'THIRD-PARTY-NOTICES.txt').write_text(notice,encoding='utf-8')
 dirty=bool(capture('git','status','--porcelain','--untracked-files=all'));commit=capture('git','rev-parse','HEAD')
 tracked=capture('git','ls-files','-z').split('\0')
 if dirty:
  with zipfile.ZipFile(output/'source.zip','w',compression=zipfile.ZIP_DEFLATED) as archive:
   for name in tracked:
    path=ROOT/name
    if path.is_file():
     if not path.resolve().is_relative_to(ROOT):raise ValueError('External source path refused')
     archive.write(path,name)
 else:run('git','archive','--format=zip','--output='+str(output/'source.zip'),'HEAD')
 with zipfile.ZipFile(output/'source.zip') as archive:
  for name in ('web/adapter/src/lib.rs','src/compare.rs','src/model.rs','web/adapter/Cargo.lock','Cargo.lock','BUILD.md'):
   expected=(ROOT/name).read_bytes() if dirty else subprocess.check_output(['git','show',commit+':'+name],cwd=ROOT)
   if archive.read(name)!=expected:raise ValueError('Source ZIP differs from build source: '+name)
 for name in ('web/adapter/src/lib.rs','src/compare.rs','src/model.rs','web/adapter/Cargo.lock','Cargo.lock'):
  dest=output/'source'/name;dest.parent.mkdir(parents=True,exist_ok=True)
  dest.write_bytes((ROOT/name).read_bytes() if dirty else subprocess.check_output(['git','show',commit+':'+name],cwd=ROOT))
 provenance={'schema':'scenarium.pages-build.v1','commit':commit,'dirty':dirty,'source':'https://github.com/giodl73-repo/SCENARIUM',
   'rust':capture('rustc','+1.95.0','--version'),'bindings':capture('wasm-bindgen','--version'),'dependencies':sorted(dependencies,key=lambda p:p['name']),
   'example_pairs':6,'delivery_budget_bytes':5_000_000}
 (output/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n',encoding='utf-8');(output/'.nojekyll').touch()
 cache=ROOT/'.cache';cache.mkdir(exist_ok=True);binary=target/'debug'/('scenarium-compare.exe' if os.name=='nt' else 'scenarium-compare')
 cases=[]
 def case(name,record):
  path=cache/'request.json';path.write_text(record,encoding='utf-8',newline='')
  canonical=capture(str(binary),str(path));json.loads(canonical);cases.append({'name':name,'input':record,'canonical':canonical})
 for name,pair in examples.items():case(name,json.dumps(pair))
 def altered(name,side,mutate):
  pair=dict(sample);doc=json.loads(pair[side+'_json']);mutate(doc);pair[side+'_json']=json.dumps(doc,ensure_ascii=False);case(name,json.dumps(pair,ensure_ascii=False))
 altered('wrong-schema','baseline',lambda d:d.update(schema='scenarium.v9'))
 altered('forged-identity','candidate',lambda d:d['payload'].update(run_id='forged'))
 altered('metric-mismatch','candidate',lambda d:d['payload']['metrics'].pop('latency_ms'))
 altered('direction-mismatch','candidate',lambda d:d['payload']['metrics']['latency_ms'].update(direction={'kind':'higher_is_better'}))
 altered('unknown-large-integer','baseline',lambda d:d['payload'].update(extension={'precise':18446744073709551615,'text':'Café 🌿 <img src=x onerror=alert(1)>'}))
 altered('no-metrics','baseline',lambda d:d['payload'].update(metrics={}))
 altered('nonfinite-value','candidate',lambda d:d['payload']['metrics']['latency_ms'].update(value='NaN'))
 altered('over-findings','baseline',lambda d:d['payload'].update(findings=[{'severity':'error','code':'x','location':'fixture','message':'synthetic'}]*257))
 altered('over-metrics','baseline',lambda d:d['payload'].update(metrics={str(i):{'name':str(i),'value':i,'direction':{'kind':'higher_is_better'}} for i in range(257)}))
 for name,text in [('invalid-json','[broken'),('deep-json','['*33+'0'+']'*33),('bom','\ufeff'+sample['baseline_json']),('over-bytes',' '*1048577)]:
  case(name,json.dumps({**sample,'baseline_json':text}))
 case('invalid-envelope','{}')
 case('over-envelope',' '*3145729)
 (cache/'native-references.json').write_text(json.dumps(cases,indent=2)+'\n',encoding='utf-8')
 files={p.relative_to(output).as_posix():pin(p) for p in sorted(output.rglob('*')) if p.is_file()}
 size=sum(p['bytes'] for p in files.values())
 if size>=5_000_000:raise ValueError('Static demo exceeds the 5 MB delivery budget')
 (cache/'site-inventory.json').write_text(json.dumps({'commit':commit,'dirty':dirty,'bytes':size,'files':files},indent=2)+'\n')
 print(json.dumps({'output':str(output),'bytes':size,'files':len(files),'native_reference_cases':len(cases),'dirty':dirty}))

if __name__=='__main__':main()
