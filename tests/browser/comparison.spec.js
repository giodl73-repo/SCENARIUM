import { test, expect } from '@playwright/test';
import { readFile, mkdir } from 'node:fs/promises';
const fixtures=JSON.parse(await readFile('.cache/native-references.json','utf8'));
const examples=JSON.parse(await readFile('dist/SCENARIUM/examples.json','utf8'));
async function opened(page){await page.goto('./');await expect(page.locator('#results')).toBeVisible();}
async function download(page,id){const wait=page.waitForEvent('download');await page.locator('#'+id).click();return readFile(await (await wait).path(),'utf8');}
test('real WASM matches every native canonical response',async({page})=>{
 await page.goto('./');
 const actual=await page.evaluate(async cases=>{
  const w=new Worker(new URL('worker.js',location.href),{type:'module'});
  await new Promise((resolve,reject)=>{w.onmessage=e=>e.data.type==='ready'?resolve():reject(e.data);w.onerror=reject;});
  const results=[];for(let i=0;i<cases.length;i++)results.push(await new Promise((resolve,reject)=>{w.onmessage=e=>resolve(e.data.canonical);w.onerror=reject;w.postMessage({type:'compare',id:i,request:cases[i].input});}));w.terminate();return results;
 },fixtures);expect(actual).toEqual(fixtures.map(f=>f.canonical));
});
for(const [name,outcome] of Object.entries({mixed:'mixed',improved:'improved',regressed:'regressed',equivalent:'equivalent',target:'improved','error-findings':'improved'}))test('example '+name,async({page})=>{
 await opened(page);await page.locator(`[data-example="${name}"]`).click();await page.locator('#run').click();
 await expect(page.locator('#outcome')).toHaveText('Metric outcome: '+outcome);
 if(name==='error-findings'){await expect(page.locator('#finding-warning')).toBeVisible();await expect(page.locator('#candidate-evidence')).toContainText('example-error');}
 if(name==='target')await expect(page.locator('#deltas')).toContainText('Toward 90');
 expect(await download(page,'download')).toBe(JSON.parse(fixtures.find(f=>f.name===name).canonical).comparison_json);
});
test('own records, actionable rejection, edit invalidates exports',async({page})=>{
 await opened(page);const candidate=JSON.parse(examples.improved.candidate_json);candidate.payload.metrics.latency_ms.value=70;
 await page.locator('#candidate').fill(JSON.stringify(candidate));await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#download')).toBeDisabled();
 await page.locator('#run').click();await expect(page.locator('#outcome')).toContainText('improved');await expect(page.locator('#deltas')).toContainText('70');
 candidate.payload.run_id='forged';await page.locator('#candidate').fill(JSON.stringify(candidate));await page.locator('#run').click();await expect(page.locator('#error')).toBeVisible();await expect(page.locator('#results')).toBeHidden();
});
test('keyboard can choose, compare and restore examples',async({page})=>{
 await opened(page);await page.locator('[data-example="regressed"]').focus();await page.keyboard.press('Enter');
 await page.locator('#run').focus();await page.keyboard.press('Enter');await expect(page.locator('#outcome')).toContainText('regressed');
 await page.locator('#reset').focus();await page.keyboard.press('Enter');await expect(page.locator('#baseline')).toBeFocused();await expect(page.locator('#results')).toBeHidden();
 await page.locator('#run').focus();await page.keyboard.press('Enter');await expect(page.locator('#outcome')).toContainText('mixed');
});
test('uploaded CRLF and precise extensions survive both input exports and bad replacement',async({page})=>{
 await opened(page);
 for(const side of ['baseline','candidate']){
  const original=examples.mixed[side+'_json'].replace('"payload": {','"payload": {"extension":{"integer":18446744073709551615,"text":"Café 🌿"},').replaceAll('\n','\r\n');
  await page.locator('#'+side+'-file').setInputFiles({name:'record.json',mimeType:'application/json',buffer:Buffer.from(original)});
  await expect.poll(()=>page.locator('#'+side).inputValue()).toContain('extension');expect(await download(page,side+'-download')).toBe(original);
  await page.locator('#'+side+'-file').setInputFiles({name:'bad.json',mimeType:'application/json',buffer:Buffer.from([0xff])});
  await expect(page.locator('#error')).toBeVisible();expect(await download(page,side+'-download')).toBe(original);
 }
 await page.locator('#run').click();await expect(page.locator('#unknown-warning')).toContainText('/payload/extension');
 await expect(page.locator('#baseline-known')).not.toContainText('extension');
});
test('oversize file keeps previous input',async({page})=>{
 await opened(page);const before=await download(page,'baseline-download');await page.locator('#baseline-file').setInputFiles({name:'large.json',mimeType:'application/json',buffer:Buffer.alloc(1048577,32)});
 await expect(page.locator('#error')).toContainText('1 MiB');expect(await download(page,'baseline-download')).toBe(before);
});
test('late local read cannot overwrite an edit',async({page})=>{
 await opened(page);await page.evaluate(()=>{File.prototype.arrayBuffer=()=>new Promise(resolve=>window.finishRead=()=>resolve(new TextEncoder().encode('stale').buffer));});
 await page.locator('#baseline-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});
 await page.locator('#baseline').fill('my newer draft');await page.evaluate(()=>window.finishRead());expect(await page.locator('#baseline').inputValue()).toBe('my newer draft');await expect(page.locator('#run')).toBeEnabled();
});
test('file deadline recovers comparison and retains draft',async({page})=>{
 await opened(page);await page.clock.install();const before=await page.locator('#baseline').inputValue();
 await page.evaluate(()=>{File.prototype.arrayBuffer=()=>new Promise(()=>{});});
 await page.locator('#baseline-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});
 await expect(page.locator('#run')).toBeDisabled();await page.clock.fastForward(5001);await expect(page.locator('#error')).toContainText('five seconds');await expect(page.locator('#run')).toBeEnabled();expect(await page.locator('#baseline').inputValue()).toBe(before);
});
test('startup deadline retains independent input downloads',async({page})=>{
 await page.addInitScript(()=>{window.Worker=class {postMessage(){}terminate(){}};});await page.clock.install();await page.goto('./');await page.locator('#baseline').fill('retained');await page.clock.fastForward(12001);
 await expect(page.locator('#retry')).toBeVisible();expect(await download(page,'baseline-download')).toBe('retained');await expect(page.locator('a[href="./source.zip"]')).toBeVisible();
});
test('compute deadline survives input edit and fences late result',async({page})=>{
 await page.addInitScript(()=>{window.Worker=class {constructor(){window.fakeWorker=this;setTimeout(()=>this.onmessage?.({data:{type:'ready'}}),20);}postMessage(data){this.last=data;}terminate(){}};});
 await page.clock.install();await page.goto('./');await page.clock.fastForward(21);await expect(page.locator('#run')).toBeDisabled();await page.locator('#baseline').fill('retained draft');await page.clock.fastForward(10001);await expect(page.locator('#retry')).toBeVisible();
 await page.evaluate(()=>window.fakeWorker.onmessage({data:{id:window.fakeWorker.last.id,canonical:'{}'}}));await expect(page.locator('#results')).toBeHidden();expect(await download(page,'baseline-download')).toBe('retained draft');
});
test('no JavaScript retains examples and downloadable source',async({browser,baseURL})=>{
 const context=await browser.newContext({javaScriptEnabled:false});const page=await context.newPage();await page.goto(baseURL);await expect(page.locator('#static-help')).toBeVisible();await expect(page.locator('#baseline')).toContainText('scenarium.v1');
 const response=await page.request.get(new URL('source.zip',baseURL).href);expect(response.ok()).toBeTruthy();expect((await response.body()).subarray(0,2).toString()).toBe('PK');await context.close();
});
test('artifacts are downloadable and correct content types',async({request,baseURL})=>{
 const wasm=await request.get(new URL('pkg/scenarium_web_bg.wasm',baseURL).href);expect(wasm.headers()['content-type']).toContain('application/wasm');expect([...((await wasm.body()).subarray(0,4))]).toEqual([0,97,115,109]);
 for(const name of ['BUILD.md','LICENSE','THIRD-PARTY-NOTICES.txt','rust/COPYRIGHT-library.html','examples.json','source/web/adapter/Cargo.lock','source/src/compare.rs'])expect((await request.get(new URL(name,baseURL).href)).ok(),name).toBeTruthy();
 const provenance=await(await request.get(new URL('provenance.json',baseURL).href)).json();expect(provenance.schema).toBe('scenarium.pages-build.v1');
});
for(const width of [1440,390])test('readable results at '+width,async({page})=>{
 await page.setViewportSize({width,height:1000});await opened(page);await page.locator('[data-example="error-findings"]').click();await page.locator('#run').click();await expect(page.locator('#finding-warning')).toBeVisible();
 expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBeTruthy();
 await mkdir('.cache/review',{recursive:true});await page.screenshot({path:`.cache/review/results-${width}.png`,fullPage:true});
});
