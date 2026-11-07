import {assertSafePath} from './evidence.mjs';
export const ALLOWLIST=Object.freeze({
 'hq-unit': ['node','--test','hq/test/live-graph-wave1-a.test.mjs'],
 'lane-b-unit':['node','--test','hq/test/wave1-kb-read.test.mjs'],
 'lane-c-unit':['node','--test','hq/live-graph/wave1/lane-c/test/*.test.mjs'],
 'rust-unit':['cargo','test','--manifest-path','research/rust-knowledge-graph/Cargo.toml','--features','importer'],
 'compiler-unit':['cargo','test','--manifest-path','research/comment-program-rust/Cargo.toml']
});
export function authorizeRecipe(recipe,workspace,{nativeUnlocked=false}={}){
 if(!nativeUnlocked)return {allowed:false,reason:'MASTER_FINAL_HANDOFF_REQUIRED'};
 const allowed=ALLOWLIST[recipe?.id];if(!allowed)return {allowed:false,reason:'RECIPE_NOT_REGISTERED'};
 if(!Array.isArray(recipe.argv)||JSON.stringify(recipe.argv)!==JSON.stringify(allowed))return {allowed:false,reason:'ARGV_NOT_PINNED'};
 if(recipe.cwd!=='.')return {allowed:false,reason:'CWD_NOT_PINNED'};
 try {const candidate=assertSafePath(workspace,'hq',{mustExist:false});if(!candidate)return {allowed:false,reason:'WORKSPACE_UNAVAILABLE'}}catch(e){return {allowed:false,reason:e.message}}
 return {allowed:true,reason:'PINNED_RECIPE_ONLY',argv:[...allowed]};
}
