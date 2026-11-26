'use strict';
function descriptor(d,mode){
 const rows={
 vivim_terminal_start:["Starting authorized terminal","Terminal start returned","An applied receipt is admission, not completion. For requested completed work, observe the same session using session_wait if exposed or session_output, starting both cursors at 0. Never replay an uncertain mutation under a new ID."],
 vivim_session_start:["Starting authorized session","Session start returned","Use returned planId as session_id. Observe requested results through terminal exit and drained output."],
 vivim_session_status:["Checking execution status","Session status received","Status alone cannot establish drained output; observe output using current cursors."],
 vivim_session_output:["Reading process output","Process output received","Advance both next cursors; continue observing running or undrained sessions within the requested task budget. Stop on denial, uncertainty or user cancellation."],
 vivim_session_wait:["Waiting for process output","Session observation received","A wait timeout is not a job timeout. Continue with both returned next cursors until terminal exit and closed, drained output. Report gaps/read failures. If wait is unavailable use session_output."],
 vivim_session_input:["Sending authorized session input","Session input returned","Observe the same session after intentional input."],
 vivim_session_cancel:["Requesting cancellation","Cancellation response received","Verify final state; acknowledgment alone is not termination."]
 };
 const r=rows[d.name];if(!r||mode==="A")return d;
 const out={...d,_meta:{...d._meta,"openai/toolInvocation/invoking":r[0],"openai/toolInvocation/invoked":r[1]}};
 if(mode==="C"||mode==="D")out.description=d.description+" "+r[2];
 if(mode==="D"&&["vivim_session_wait","vivim_session_output"].includes(d.name))out.outputSchema={type:"object",required:["observation","raw"],additionalProperties:false,properties:{raw:{type:"object"},observation:{type:"object",required:["state","nextAction"],properties:{state:{enum:["running","draining","succeeded","failed","cancelled","timed-out","incomplete-evidence","uncertain"]},nextAction:{enum:["observe","inspect","stop"]},sessionId:{type:"string"},stdoutCursor:{type:"string"},stderrCursor:{type:"string"},exitCode:{type:["integer","null"]}},additionalProperties:false}}};
 return out;
}

function observe(name,raw,mode){
 if(mode!=="D"||!["vivim_session_wait","vivim_session_output"].includes(name))return null;
 const s=raw?.status,a=raw?.stdout,b=raw?.stderr;
 const valid=c=>c&&Number.isSafeInteger(c.next)&&c.next>=0&&Number.isSafeInteger(c.observed)&&c.observed>=c.next&&["closed","gap","readFailed"].every(k=>typeof c[k]==="boolean");
 if(!s||typeof s.sessionId!=="string"||!valid(a)||!valid(b))return {state:"uncertain",nextAction:"inspect"};
 const v={sessionId:s.sessionId,stdoutCursor:String(a.next),stderrCursor:String(b.next),exitCode:Number.isInteger(s.exitCode)?s.exitCode:null};
 if(a.gap||b.gap||a.readFailed||b.readFailed)return {...v,state:"incomplete-evidence",nextAction:"inspect"};
 if(["running","starting"].includes(s.state))return {...v,state:"running",nextAction:"observe"};
 if(!["exited","timed-out"].includes(s.state))return {...v,state:"uncertain",nextAction:"inspect"};
 if(!a.closed||!b.closed||a.next!==a.observed||b.next!==b.observed)return {...v,state:"draining",nextAction:"observe"};
 if(s.cancelled===true||s.state==="timed-out")return {...v,state:s.cancelled===true?"cancelled":"timed-out",nextAction:"stop"};
 if(v.exitCode===0&&s.success===true)return {...v,state:"succeeded",nextAction:"stop"};
 if(v.exitCode!==null&&s.success===false)return {...v,state:"failed",nextAction:"stop"};
 return {...v,state:"uncertain",nextAction:"inspect"};
}

function decorate(name,raw,mode){const out={content:[{type:"text",text:JSON.stringify(raw)}]},o=observe(name,raw,mode);if(o)out.structuredContent={observation:o,raw};return out}
module.exports={descriptor,observe,decorate};
