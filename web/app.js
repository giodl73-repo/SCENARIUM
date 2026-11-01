const $=id=>document.getElementById(id),ids=['baseline','candidate'];
$('static-help').hidden=true;
const examples=JSON.parse($('examples-data').textContent);
let worker,generation=0,revision=0,serial=0,active=null,ready=false,busy=false,startupTimer,computeTimer,result=null,canonical=null;
const files={baseline:{token:0,reading:false,timer:null,original:null},candidate:{token:0,reading:false,timer:null,original:null}};
function controls(){
  $('run').disabled=!ready||busy||ids.some(id=>files[id].reading);
  for(const id of ['download','report-download'])$(id).disabled=!result;
  for(const id of ids)$(id+'-download').disabled=!$(id).value&&files[id].original===null;
}
function clearResults(){result=canonical=null;$('results').hidden=true;$('deltas').replaceChildren();controls();}
function invalidated(){revision++;clearResults();$('error').hidden=true;if(ready)$('status').textContent=busy?'Inputs changed. Waiting for the previous comparison.':'Inputs changed. Compare again to refresh results.';}
function cancelFile(id){files[id].token++;files[id].reading=false;clearTimeout(files[id].timer);}
function edited(id){cancelFile(id);files[id].original=null;invalidated();}
function diagnostic(phase,detail){clearResults();$('error').hidden=false;$('error-title').textContent=phase[0].toUpperCase()+phase.slice(1)+' needs attention';$('error-message').textContent=detail.message||'Comparison could not finish.';$('diagnostic').textContent=JSON.stringify(detail,null,2);}
function engineError(message,detail=''){
  generation++;ready=busy=false;active=null;clearTimeout(startupTimer);clearTimeout(computeTimer);worker?.terminate();worker=null;
  diagnostic('engine',{message,...(detail?{detail}:{})});$('status').textContent='Engine unavailable. Inputs are retained; original/source downloads still work.';$('retry').hidden=false;controls();
}
function pretty(value){return JSON.stringify(value,null,2);}
function render(response){
  const comparison=JSON.parse(response.comparison_json).payload;
  $('outcome').textContent=`Metric outcome: ${response.outcome}`;
  $('identities').textContent=`Baseline: ${comparison.baseline_run_id} · Candidate: ${comparison.candidate_run_id}`;
  $('run-statuses').textContent=`Findings-derived run status · Baseline: ${response.baseline.run_status} · Candidate: ${response.candidate.run_status}`;
  $('finding-warning').hidden=response.baseline.run_status!=='error'&&response.candidate.run_status!=='error';
  $('finding-warning').textContent='Error findings remain present. A metric improvement does not erase them or authorize adoption.';
  const extras=ids.flatMap(id=>response[id].unknown_fields.map(path=>`${id}${path}`));
  $('unknown-warning').hidden=!extras.length;$('unknown-warning').textContent=`Unknown fields ignored by native decoding: ${extras.join(', ')}. Use input downloads to preserve them; known-field reports omit extensions.`;
  for(const delta of comparison.deltas){
    const row=document.createElement('tr');
    const direction=delta.direction.kind==='target'?`Toward ${delta.direction.target}`:delta.direction.kind.replaceAll('_',' ');
    const format=n=>new Intl.NumberFormat(undefined,{maximumSignificantDigits:8}).format(n);
    for(const value of [delta.metric,direction,format(delta.baseline),format(delta.candidate),format(delta.beneficial_change)]){const cell=document.createElement('td');cell.textContent=value;row.append(cell);}$('deltas').append(row);
  }
  for(const id of ids){const text=response[id].known_json;const payload=JSON.parse(text).payload;$(id+'-known').textContent=text;$(id+'-evidence').textContent=pretty({findings:payload.findings,provenance:payload.provenance});}
  $('results').hidden=false;
}
function run(){
  if(!ready||busy||ids.some(id=>files[id].reading))return;
  clearResults();$('error').hidden=true;
  for(const id of ids)if($(id).value.length>1048576||new TextEncoder().encode($(id).value).length>1048576){diagnostic(id,{message:'Run exceeds the 1 MiB UTF-8 limit.'});$('status').textContent='Reduce the input and compare again.';return;}
  const request=JSON.stringify({baseline_json:$('baseline').value,candidate_json:$('candidate').value});
  if(new TextEncoder().encode(request).length>3*1048576){diagnostic('request',{message:'Envelope exceeds 3 MiB UTF-8 bytes.'});return;}
  busy=true;active={id:++serial,revision};controls();$('status').textContent='Comparing in Rust…';const id=active.id,epoch=generation;
  computeTimer=setTimeout(()=>{if(epoch===generation&&active?.id===id)engineError('Comparison exceeded its deadline. Retry, then compare your retained inputs.');},10000);
  try{worker.postMessage({type:'compare',id,request});}catch(error){engineError('The worker could not accept these inputs.',String(error));}
}
function start(){
  generation++;const epoch=generation;worker?.terminate();clearTimeout(startupTimer);clearTimeout(computeTimer);ready=busy=false;active=null;clearResults();$('error').hidden=true;$('retry').hidden=true;$('status').textContent='Loading Rust engine…';
  startupTimer=setTimeout(()=>{if(epoch===generation&&!ready)engineError('Rust did not start before its deadline. Retry to load the engine again.');},12000);
  try{worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});}catch(error){engineError('The browser could not create the Rust worker.',String(error));return;}
  worker.onerror=event=>{event.preventDefault();if(epoch===generation)engineError('The Rust worker failed.',event.message);};
  worker.onmessage=({data})=>{
    if(epoch!==generation)return;
    if(data.type==='startup-error'){engineError('Rust could not be downloaded or initialized.',data.message);return;}
    if(data.type==='ready'){clearTimeout(startupTimer);ready=true;$('status').textContent='Rust engine ready. Compare your records.';controls();if(revision===0)run();return;}
    if(!active||active.id!==data.id)return;clearTimeout(computeTimer);const submitted=active.revision;busy=false;active=null;
    if(data.type==='compute-error'){engineError('The Rust worker could not finish comparison.',data.message);return;}
    if(submitted!==revision){$('status').textContent='Inputs changed. Compare again to refresh results.';controls();return;}
    let response;try{response=JSON.parse(data.canonical);}catch{engineError('The worker returned an unreadable response.');return;}
    if(!response.ok){diagnostic(response.phase,response.diagnostic);$('status').textContent='Fix the input and compare again.';controls();return;}
    result=response;canonical=data.canonical;render(response);$('status').textContent='Comparison complete. Deltas describe the current supplied records.';controls();
  };
}
function download(name,text){const url=URL.createObjectURL(new Blob([text],{type:'application/json;charset=utf-8'}));const link=document.createElement('a');link.href=url;link.download=name;link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
function loadExample(name){for(const id of ids){cancelFile(id);files[id].original=null;$(id+'-file').value='';$(id).value=examples[name][id+'_json'];}invalidated();$('baseline').focus();}
$('comparison-form').addEventListener('submit',e=>{e.preventDefault();run();});
for(const id of ids){
  $(id).addEventListener('input',()=>edited(id));
  $(id+'-download').addEventListener('click',()=>download(id+'-input.json',files[id].original??$(id).value));
  $(id+'-file').addEventListener('change',async()=>{
    const file=$(id+'-file').files[0];if(!file)return;cancelFile(id);invalidated();const token=files[id].token;
    if(file.size>1048576){diagnostic(id,{message:'Choose a UTF-8 JSON file no larger than 1 MiB.'});$(id+'-file').value='';return;}
    files[id].reading=true;controls();files[id].timer=setTimeout(()=>{if(files[id].token===token){cancelFile(id);diagnostic(id,{message:'Local file read exceeded five seconds. The previous draft is retained.'});controls();}},5000);
    try{const text=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(await file.arrayBuffer());if(files[id].token===token){$(id).value=text;edited(id);files[id].original=text;}}
    catch{if(files[id].token===token)diagnostic(id,{message:'File could not be read as valid UTF-8. The previous input is retained.'});}
    finally{if(files[id].token===token){files[id].reading=false;clearTimeout(files[id].timer);}$(id+'-file').value='';controls();}
  });
}
for(const button of document.querySelectorAll('[data-example]'))button.addEventListener('click',()=>loadExample(button.dataset.example));
$('reset').addEventListener('click',()=>loadExample('mixed'));$('retry').addEventListener('click',start);
$('download').addEventListener('click',()=>{if(result)download('comparison.scenarium.json',result.comparison_json);});
$('report-download').addEventListener('click',()=>{if(result)download('comparison-workbench.json',canonical);});
start();
