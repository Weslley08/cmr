import test from 'node:test';
import assert from 'node:assert/strict';
import {register} from '../hooks/register.mjs';

function harness(){
  const handlers=new Map(),log=[];
  register((name,matcher,fn)=>handlers.set(name,fn??matcher));
  const $={command:{register:async()=>{}},ui:{log:s=>log.push(s)}};
  const proceed=async e=>({e});
  return {handlers,log,$,proceed};
}
async function step(h,e={model:'sonnet'}){
  let sent;
  async function* next(input){
    sent=input;
    yield {type:'delta',text:'hello'};
    return {usage:{model:input.model,input_tokens:3,output_tokens:2,cache_read_input_tokens:1}};
  }
  const output=h.handlers.get('turn.step')(h.$,e,next);
  while(!(await output.next()).done){}
  return sent;
}
test('routes and leaves subagents untouched',async()=>{
  const h=harness();
  await h.handlers.get('prompt.submit')(h.$,{text:'Liste arquivos do src'},h.proceed);
  assert.equal((await step(h)).model,'haiku');
  assert.equal((await step(h,{model:'opus',agentId:'other'})).model,'opus');
  assert.match((await h.handlers.get('command.run')(h.$,{args:'stats'})).text,/Haiku 1/);
});
test('mode / force / off',async()=>{
  const h=harness();
  await h.handlers.get('command.run')(h.$,{args:'mode quality'});
  await h.handlers.get('prompt.submit')(h.$,{text:'Crie um componente Angular'},h.proceed);
  assert.equal((await step(h)).model,'opus');
  await h.handlers.get('command.run')(h.$,{args:'force haiku'});
  assert.equal((await step(h)).model,'haiku');
  await h.handlers.get('command.run')(h.$,{args:'off'});
  assert.equal((await step(h)).model,'sonnet');
});
test('no prompt preserves user model',async()=>{
  const h=harness();
  assert.equal((await step(h,{model:'opus'})).model,'opus');
});
