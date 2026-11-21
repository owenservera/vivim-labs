/**
 * Pure structural validation of the pre-G prompt cohort.
 * These checks cannot certify source truth, tests, Host rights, G status or device acceptance.
 */
export const COHORT_VALIDATION_SCHEMA='arbor.proposal5.foundations.prompt-cohort-validation/1';
const hex40=/^[a-f0-9]{40}$/;
const validText=x=>typeof x==='string'&&x.trim().length>0;
const pathBase='.integration/proposal5/foundations/';
const ids=['PG00','PG01','PG02','PG03','PGZ'];
const checkUnique=(items)=>new Set(items).size===items.length;
const issue=(errors,code,where)=>errors.push({code,where});
export function validatePromptCohort({cohort,waves}={}) {
  const errors=[];
  if(!cohort||!Array.isArray(cohort.lanes)||!waves||!Array.isArray(waves.waves))
    return {schema:COHORT_VALIDATION_SCHEMA,status:'INVALID',errors:[{code:'BAD_INPUT',where:'cohort/waves'}],counts:null,effectPerformed:false};
  if(cohort.schema!=='arbor.proposal5.foundations.prompt-cohort/1')issue(errors,'SCHEMA','cohort');
  if(cohort.repository!=='owenservera/ARBOR')issue(errors,'REPO','cohort');
  if(cohort.mobilePolicy!=='PWA_FIRST_APK_DEFERRED')issue(errors,'MOBILE_SCOPE','cohort');
  if(cohort.status!=='PROMPTS_COMMITTED_NOT_WORKERS_EXECUTED')issue(errors,'UNSAFE_PROGRESS_CLAIM','cohort');
  if(cohort.GStatusAtAuthoring!=='ABSENT_NOT_RUN')issue(errors,'G_TRUTH_OVERCLAIM','cohort');
  if(cohort.programBaseRef!=='work/p5-pre-g-convergence-r001')issue(errors,'G_BASE_COLLISION','cohort');
  if(!cohort.sharedContractPath.startsWith(pathBase+'prompts/'))issue(errors,'MISSING_CONTRACT','shared');
  if(cohort.lanes.length!==5||!checkUnique(cohort.lanes.map(l=>l.id))||
     !ids.every(x=>cohort.lanes.some(l=>l.id===x)))issue(errors,'LANE_SET','cohort');
  if(!checkUnique(cohort.lanes.map(l=>l.workerBranch)))issue(errors,'BRANCH_COLLISION','cohort');
  if(!checkUnique(cohort.lanes.map(l=>l.exclusiveWritePrefix)))issue(errors,'WRITE_COLLISION','cohort');
  const waveTasks=new Set(waves.waves.filter(w=>w.stage==='PRE_G').flatMap(w=>w.tasks.map(t=>t.id)));
  let count=0;
  for(const l of cohort.lanes) {
    if(!ids.includes(l.id))issue(errors,'UNKNOWN_LANE',String(l.id));
    if(!l.promptPath.startsWith(pathBase+'prompts/')||!l.promptPath.endsWith('.md'))
      issue(errors,'BAD_PROMPT_PATH',l.id);
    if(l.exclusiveWritePrefix!==pathBase+'proofs/'+(l.id==='PGZ'?'PGZ-convergence':
      l.id==='PG00'?'PG00-source-truth':l.id==='PG01'?'PG01-native-abi':
      l.id==='PG02'?'PG02-semantic-fabric':'PG03-pwa-storage')+'/')
      issue(errors,'WRITE_PREFIX_MISMATCH',l.id);
    if(!/^work\/p5-fa-pg(?:00|01|02|03|z)-[a-z0-9-]+-r001$/.test(l.workerBranch))
      issue(errors,'UNEXPECTED_BRANCH',l.id);
    if(l.canMerge!==false||l.writesToProduct!==false||l.canBlockG!==false||
       l.disposition!=='PREPARED_NOT_PROMOTED'||l.status!=='READY_TO_LAUNCH_NOT_STARTED')
      issue(errors,'UNAUTHORIZED_WORKER_POWER',l.id);
    if(!Array.isArray(l.minimumArtifacts)||!l.minimumArtifacts.includes('RUN-LEDGER.json')||
      !l.minimumArtifacts.includes('SOURCE-DELTA.json')&&l.id!=='PGZ')
      issue(errors,'MISSING_LEDGER',l.id);
    if(!Array.isArray(l.inputs)||l.inputs.some(ref=>!Object.hasOwn(cohort.frozen,ref)&&!Object.hasOwn(cohort.supports,ref)))
      issue(errors,'UNKNOWN_INPUT_REF',l.id);
    if(l.id==='PGZ'){
      if(l.phase!=='AFTER_AVAILABLE_WORKERS'||l.runOrder!==2||
         !Array.isArray(l.waitFor)||!ids.slice(0,4).every(x=>l.waitFor.includes(x)))
        issue(errors,'REFEREE_ORDER','PGZ');
      if(l.taskIds.length)issue(errors,'REFEREE_TASKS','PGZ');
    } else {
      if(l.phase!=='PARALLEL_PRE_G'||l.runOrder!==1||l.taskIds.length!==8)
        issue(errors,'PRE_G_ORDER',l.id);
      if(l.taskIds.some(t=>!waveTasks.has(t)))issue(errors,'MISSING_WAVE_TASK',l.id);
    }
    count+=l.taskIds.length;
  }
  for(const group of ['frozen','supports'])for(const [key,ref] of Object.entries(cohort[group]??{})){
    if(!validText(key)||!validText(ref.ref)||!hex40.test(ref.sha))
      issue(errors,'INVALID_SOURCE_SNAPSHOT',key);
  }
  if(count!==32||!checkUnique(cohort.lanes.flatMap(l=>l.taskIds)))
    issue(errors,'TASK_COVERAGE','cohort');
  return {schema:COHORT_VALIDATION_SCHEMA,
    status:errors.length?'INVALID':'STRUCTURALLY_READY_PROMPTS_NOT_EXECUTED',
    counts:{lanes:cohort.lanes.length,parallel:cohort.lanes.filter(l=>l.phase==='PARALLEL_PRE_G').length,referees:cohort.lanes.filter(l=>l.id==='PGZ').length,coveredPreGTasks:count},
    errors,effectPerformed:false};
}
