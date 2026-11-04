import test from 'node:test';
import assert from 'node:assert/strict';
import { register } from '../hooks/register.js';
function harness() {
  const handlers = new Map(), messages = [];
  register((name, filter, hook) => handlers.set(name, hook ?? filter));
  const $ = { command: { register: async () => {} }, ui: { log: (m) => messages.push(m) } };
  const next = async (e) => ({ e });
  return { handlers, messages, $, next };
}
async function step(h, e = {model:'sonnet'}) {
  let captured;
  async function* next(event) {
    captured = event;
    yield {type:'delta',text:'ok'};
    return {usage:{model:event.model,input_tokens:10,output_tokens:2}};
  }
  const it = h.handlers.get('turn.step')(h.$,e,next);
  while (!(await it.next()).done) {}
  return captured;
}
test('default is off and does not override', async () => {
  const h=harness();
  await h.handlers.get('prompt.submit')(h.$,{text:'Liste arquivos do src'},h.next);
  assert.equal((await step(h)).model,'sonnet');
});
test('auto routes main but not subagents',async()=>{
  const h=harness();
  await h.handlers.get('command.run')(h.$,{args:'auto'});
  await h.handlers.get('prompt.submit')(h.$,{text:'Liste arquivos do src'},h.next);
  assert.equal((await step(h)).model,'haiku');
  assert.equal((await step(h,{model:'opus',agentId:'sub-1'})).model,'opus');
  assert.match((await h.handlers.get('command.run')(h.$,{args:'stats'})).text,/Haiku 1/);
});
test('force, mode and off',async()=>{
  const h=harness();
  await h.handlers.get('command.run')(h.$,{args:'auto'});
  await h.handlers.get('command.run')(h.$,{args:'mode quality'});
  await h.handlers.get('prompt.submit')(h.$,{text:'Crie um componente Angular'},h.next);
  assert.equal((await step(h)).model,'opus');
  await h.handlers.get('command.run')(h.$,{args:'force haiku'});
  assert.equal((await step(h)).model,'haiku');
  await h.handlers.get('command.run')(h.$,{args:'off'});
  assert.equal((await step(h)).model,'sonnet');
});
