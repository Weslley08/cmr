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

test('counts requested model separately when provider does not report actual model',async()=>{
  const h=harness();
  await h.handlers.get('prompt.submit')(h.$,{text:'Liste os arquivos do src. Não modifique nada.'},h.proceed);
  async function* next(event){
    yield {type:'delta',text:'ok'};
    return {usage:{input_tokens:4,output_tokens:2}};
  }
  const stream=h.handlers.get('turn.step')(h.$,{model:'sonnet'},next);
  while(!(await stream.next()).done){}
  const stats=(await h.handlers.get('command.run')(h.$,{args:'stats'})).text;
  assert.match(stats,/requests \(selected aliases\): Haiku 1/);
  assert.match(stats,/provider-reported: Haiku 0, Sonnet 0, Opus 0/);
  assert.match(stats,/unverified 1/);
});

test('negated model mentions never override balanced routing, but /cmr force does',async()=>{
  const h=harness();
  await h.handlers.get('prompt.submit')(h.$,{
    text:'Investigue a arquitetura e a segurança. Não force Haiku, Sonnet ou Opus.'
  },h.proceed);
  assert.equal((await step(h)).model,'opus','architecture and security require Opus in balanced mode');
  await h.handlers.get('command.run')(h.$,{args:'force sonnet'});
  assert.equal((await step(h)).model,'sonnet','an explicit /cmr force still wins');
  await h.handlers.get('command.run')(h.$,{args:'force none'});
  assert.equal((await step(h)).model,'opus','force none restores automatic selection');
});
