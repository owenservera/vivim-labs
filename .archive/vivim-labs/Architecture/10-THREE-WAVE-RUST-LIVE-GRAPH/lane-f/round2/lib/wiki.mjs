import {inspectProvenance} from './hqkgx1.mjs';
export function compareWiki({exchange,pages,qualityReport,nativeReceipt}={}){
 const issues=[];
 if(!exchange?.manifest?.graph_sha256||!Array.isArray(exchange?.nodes)||!Array.isArray(exchange?.edges))issues.push({code:'EXCHANGE_REQUIRED'});
 if(!pages||!(pages instanceof Map)||!pages.size)issues.push({code:'ORIGINAL_HQ_WIKI_MISSING'});
 if(qualityReport?.schema!=='vivim.wave1.wiki-quality/1')issues.push({code:'D_ORACLE_MISSING'});
 if(qualityReport?.exchangeDigest!==exchange?.manifest?.graph_sha256)issues.push({code:'D_EXCHANGE_DRIFT'});
 if(!nativeReceipt||nativeReceipt.evidenceClass!=='observed-native'||nativeReceipt.exchangeDigest!==exchange?.manifest?.graph_sha256||!nativeReceipt.journalSha256)issues.push({code:'RUST_NATIVE_ATTESTATION_MISSING'});
 const counts={code:0,semantic:0,command:0,orphan:0};
 if(exchange?.nodes){const nodes=new Map(exchange.nodes.map(n=>[n.id,n]));const edges=exchange.edges??[];
  for(const n of exchange.nodes){const m=n.metadata??{};if(m.source_lane==='A')counts.code++;if(m.source_lane==='B')counts.semantic++;if(m.source_lane==='C')counts.command++;}
  for(const e of edges)if(!nodes.has(e.from)||!nodes.has(e.to))counts.orphan++;
  const dChains=qualityReport?.chains?.filter(c=>c.verified)??[];
  if(!dChains.length)issues.push({code:'NO_VERIFIED_D_CHAIN'});
  for(const chain of dChains){
   for(const id of [chain.code,chain.semantic,chain.command])if(!nodes.has(id))issues.push({code:'D_CHAIN_GRAPH_ID_MISSING',id});
   if(!pages?.has(chain.page))issues.push({code:'D_CHAIN_PAGE_MISSING',page:chain.page});
  }
 }
 if(exchange&&inspectProvenance(exchange).findings.length)issues.push({code:'SOURCE_PROVENANCE_MISSING'});
 return {status:issues.length?'BLOCKED':'READY_FOR_INDEPENDENT_QUERY_REPLAY',issues,counts};
}
