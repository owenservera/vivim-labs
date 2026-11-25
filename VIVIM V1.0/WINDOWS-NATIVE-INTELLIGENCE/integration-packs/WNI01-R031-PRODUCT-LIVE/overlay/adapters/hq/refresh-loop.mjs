// @hq:context Serialize source-change hints and explicit refreshes through the incumbent session.
// @hq:invariant Filesystem notifications are hints, not proof of source coverage or authorization.
// @hq:why Never silently consume changes observed while the existing index/compiler/Wiki runs.
import {watch,realpathSync,statSync} from 'node:fs';
import {relative,isAbsolute,sep} from 'node:path';
import {setTimeout as settle} from 'node:timers/promises';
export function createRefreshLoop({refresh,onError=()=>{},debounceMs=150,settleMs=25,maxPasses=3,observe=true}){
 if(typeof refresh!=='function'||typeof onError!=='function'||!Number.isSafeInteger(debounceMs)||debounceMs<10||debounceMs>5000||!Number.isSafeInteger(settleMs)||settleMs<1||settleMs>1000||!Number.isSafeInteger(maxPasses)||maxPasses<1||maxPasses>5)throw Error('REFRESH_CONFIG');
 let version=0,committed=-1,failed=false,closed=false,timer=null,autoQueued=false,tail=Promise.resolve(),queued=0,last=null;
 function clearTimer(){if(timer!==null){clearTimeout(timer);timer=null;}}
 function enqueue(task){
  if(closed)return Promise.reject(Error('REFRESH_CLOSED'));
  if(queued>=64)return Promise.reject(Error('REFRESH_QUEUE_LIMIT'));
  queued++;
  const result=tail.then(()=>{if(closed)throw Error('REFRESH_CLOSED');return task();});
  tail=result.catch(()=>{}).finally(()=>{queued--;});return result;
 }
 function schedule(){
  if(!observe||failed||closed||autoQueued)return;
  clearTimer();timer=setTimeout(()=>{
   timer=null;autoQueued=true;
   enqueue(async()=>{if(committed!==version&&!failed)await perform();})
    .catch(e=>{if(!closed)onError(e);})
    .finally(()=>{autoQueued=false;/* No unbounded automatic retries on failure. */});
  },debounceMs);
 }
 function invalidate(){
  if(closed)return;
  if(version===Number.MAX_SAFE_INTEGER){lost();return;}
  version++;schedule();
 }
 function lost(){failed=true;committed=-1;clearTimer();}
 async function perform(){
  clearTimer();
  for(let pass=0;pass<maxPasses;pass++){
   const before=version;
   committed=-1;
   const next=await refresh();
   // The incumbent refresh has synchronous portions. Allow queued native
   // notifications to be delivered before publishing a settled status.
   if(observe)await settle(settleMs);
   if(closed)throw Error('REFRESH_CLOSED');
   if(failed)throw Error('SOURCE_OBSERVER_UNAVAILABLE');
   if(before===version){
    committed=before;last=next;clearTimer();return next;
   }
  }
  throw Error('SOURCE_REFRESH_UNSETTLED');
 }
 function requireCurrent(){if(failed||committed!==version||last===null)throw Error(failed?'SOURCE_OBSERVER_UNAVAILABLE':'SOURCE_REFRESH_REQUIRED');}
 return {invalidate,lost,enqueue,perform,requireCurrent,status:()=>({observing:observe,available:!failed,dirty:committed!==version,revision:version,settledRevision:committed}),async close(){closed=true;clearTimer();await tail;}};
}
// Watch only the already admitted root. No paths or file contents are published.
// This is existing Node/Windows observation plumbing, not a coverage proof;
// root replacement, silent notification loss and provider limitations remain.
export function observeRoot({source,data,loop}){
 const root=realpathSync(source),out=realpathSync(data);
 const inside=relative(root,out);
 if(!statSync(root).isDirectory()||inside===''||(!isAbsolute(inside)&&inside!=='..'&&!inside.startsWith('..'+sep)))throw Error('WATCH_OUTPUT_INSIDE_SOURCE');
 const initial=statSync(root,{bigint:true});
 let watcher;
 try{
  watcher=watch(root,{recursive:true,persistent:false},()=>{
   try{const now=statSync(root,{bigint:true});if(now.dev!==initial.dev||now.ino!==initial.ino){loop.lost();watcher.close();return;}}catch{loop.lost();watcher.close();return;}
   loop.invalidate();
  });
  watcher.on('error',()=>{loop.lost();watcher.close();});
  watcher.on('close',()=>loop.lost());
 }catch(e){loop.lost();throw e;}
 return {close:()=>watcher.close()};
}
