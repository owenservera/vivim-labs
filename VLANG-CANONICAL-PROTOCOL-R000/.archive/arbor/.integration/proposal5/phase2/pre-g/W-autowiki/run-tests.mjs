import {runNegativeTests} from './negative-tests.mjs';
try {
 const result=await runNegativeTests();
 for(const row of result.cases) console.log('PASS',row.name);
 console.log(JSON.stringify({suite:result.suite,passed:result.passed,failed:0,exitCode:0}));
} catch(error) {
 console.error('FAIL',error?.stack||String(error));
 process.exitCode=1;
}