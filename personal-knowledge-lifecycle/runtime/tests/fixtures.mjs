import {deflateRawSync} from 'node:zlib';
import {crc32Update} from '../src/zip-reader.mjs';
export function makeZip(entries, {corruptCrc=false}={}) {
  const chunks=[], central=[];let offset=0;
  for(const [name,dataInput,method=8] of entries){
    const data=Buffer.isBuffer(dataInput)?dataInput:Buffer.from(dataInput);
    const zipped=method===8?deflateRawSync(data):data;
    const fn=Buffer.from(name,'utf8');
    const crc=((crc32Update(0xffffffff,data)^0xffffffff)>>>0);
    const local=Buffer.alloc(30);
    local.writeUInt32LE(0x04034b50,0);local.writeUInt16LE(20,4);local.writeUInt16LE(0x800,6);
    local.writeUInt16LE(method,8);local.writeUInt32LE(corruptCrc?crc^1:crc,14);
    local.writeUInt32LE(zipped.length,18);local.writeUInt32LE(data.length,22);local.writeUInt16LE(fn.length,26);
    chunks.push(local,fn,zipped);
    const cd=Buffer.alloc(46);cd.writeUInt32LE(0x02014b50,0);cd.writeUInt16LE(20,4);cd.writeUInt16LE(20,6);
    cd.writeUInt16LE(0x800,8);cd.writeUInt16LE(method,10);cd.writeUInt32LE(corruptCrc?crc^1:crc,16);
    cd.writeUInt32LE(zipped.length,20);cd.writeUInt32LE(data.length,24);cd.writeUInt16LE(fn.length,28);
    cd.writeUInt32LE(offset,42);
    central.push(cd,fn);offset+=local.length+fn.length+zipped.length;
  }
  const dir=Buffer.concat(central);const eocd=Buffer.alloc(22);eocd.writeUInt32LE(0x06054b50,0);
  eocd.writeUInt16LE(entries.length,8);eocd.writeUInt16LE(entries.length,10);
  eocd.writeUInt32LE(dir.length,12);eocd.writeUInt32LE(offset,16);
  return Buffer.concat([...chunks,dir,eocd]);
}
export const conversation=(i)=>({id:'thread-'+i,title:'Discussion '+i,create_time:1720000000+i,
 update_time:1730000000+i,current_node:'n2',mapping:{
 n1:{parent:null,children:['n2'],message:{id:'m'+i+'a',author:{role:'user'},content:{content_type:'text',parts:['What is '+i+'?']}}},
 n2:{parent:'n1',children:[],message:{id:'m'+i+'b',author:{role:'assistant'},content:{content_type:'text',parts:['Answer '+i]}}}}});
export const conversationJson=(n)=>JSON.stringify(Array.from({length:n},(_,i)=>conversation(i)));
