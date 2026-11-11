#!/usr/bin/env node
// User-invoked local ZIP inspection: prints counts only, never chat content.
import path from 'node:path';
import {openZipArchive} from '../src/zip-reader.mjs';
import {parseJsonObjectArray} from '../src/json-array-stream.mjs';
const file=process.argv[2];
if(!file||!path.isAbsolute(file)){
  process.stderr.write('Usage: node bin/inspect-chatgpt-export.mjs /absolute/path/to/export.zip\n');
  process.exitCode=2;
}else{
  let zip;
  try{
    zip=await openZipArchive(file);
    const candidates=zip.entries.filter(e=>e.name==='conversations.json'||e.name.endsWith('/conversations.json'));
    if(candidates.length!==1)throw Error('Export must contain exactly one conversations.json');
    let conversations=0,messages=0,nonTextParts=0;
    for await(const c of parseJsonObjectArray(await zip.entryStream(candidates[0]))){
      conversations++;
      for(const node of Object.values(c.mapping??{})){
        if(!node?.message)continue;
        messages++;
        for(const part of node.message.content?.parts??[])if(typeof part!=='string')nonTextParts++;
      }
    }
    process.stdout.write(JSON.stringify({status:'inspected',conversations,messages,nonTextParts,
      archiveBytes:zip.size,conversationJsonBytes:candidates[0].size,
      coverage:'selected archive only; not all account history'},null,2)+'\n');
  }catch(err){process.stderr.write('Import inspection failed: '+(err.code??err.message)+'\n');process.exitCode=1;}
  finally{if(zip)await zip.close();}
}
