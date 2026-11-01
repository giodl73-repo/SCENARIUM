import init,{compare} from './pkg/scenarium_web.js';
try{await init({module_or_path:new URL('./pkg/scenarium_web_bg.wasm',import.meta.url)});self.postMessage({type:'ready'});}catch(error){self.postMessage({type:'startup-error',message:String(error)});}
self.onmessage=({data})=>{if(data.type!=='compare')return;try{self.postMessage({type:'result',id:data.id,canonical:compare(data.request)});}catch(error){self.postMessage({type:'compute-error',id:data.id,message:String(error)});}};
