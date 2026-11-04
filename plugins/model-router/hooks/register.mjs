import {chooseModel} from './policy.mjs';

// No RTK or tool hooks are touched. One selected model is used across the main
// conversation's model requests for a prompt (including post-tool continuations).
export function register(on) {
  const state = {
    active: true,
    profile: 'balanced',
    forced: null,
    last: null,
    promptNumber: 0,
    loggedPrompt: 0,
    counts: {haiku:0,sonnet:0,opus:0},
    usage: {input:0,output:0,cacheRead:0,cacheWrite:0}
  };

  on('session.start', async ($, e, next) => {
    await $.command.register({
      name: 'cmr',
      description: 'CMR model router: status | auto | off | mode economy|balanced|quality | force haiku|sonnet|opus|none | stats',
      argumentHint: '[status|auto|off|mode PROFILE|force MODEL|stats]'
    });
    return next(e);
  });

  on('command.run', {command:'cmr'}, async ($, e) => {
    const [action='status', value] = String(e.args??'').trim().toLowerCase().split(/\s+/);
    if (action==='off') {state.active=false;return {text:'CMR OFF; Claude Code chooses the model.'};}
    if (action==='auto') {state.active=true;return {text:'CMR ON ('+state.profile+'). Model changes apply on the next prompt.'};}
    if (action==='mode' && ['economy','balanced','quality'].includes(value)) {
      state.profile=value;
      return {text:'CMR profile '+value+'; selection updates on the next prompt.'};
    }
    if (action==='force' && ['haiku','sonnet','opus','none'].includes(value)) {
      state.forced=value==='none'?null:value;
      return {text:'CMR override '+(state.forced??'none')+'.'};
    }
    if (action==='stats') {
      const u=state.usage,c=state.counts;
      return {text:'CMR requests: Haiku '+c.haiku+', Sonnet '+c.sonnet+', Opus '+c.opus+
        '; input '+u.input+', output '+u.output+', cache read '+u.cacheRead+
        ', cache write '+u.cacheWrite+'. Counters are session-only; no savings guarantee.'};
    }
    if (action==='status') {
      return {text:'CMR '+(state.active?'ON':'OFF')+'; profile '+state.profile+
        '; force '+(state.forced??'none')+'; last '+(state.last?.model??'none')+
        '. Model selector shown in IDE may not reflect per-request routing.'};
    }
    return {text:'Usage: /cmr status | auto | off | mode economy|balanced|quality | force haiku|sonnet|opus|none | stats'};
  });

  on('prompt.submit', async ($, e, next) => {
    state.promptNumber++;
    state.last=chooseModel(e.text,state.profile);
    return next(e);
  });

  on('turn.step', async function* ($, e, next) {
    // Never silently override a subagent or a session with routing turned off.
    if (!state.active || !state.last || e.agentId) return yield* next(e);
    const model=state.forced??state.last.model;
    if (state.loggedPrompt !== state.promptNumber) {
      $.ui.log('CMR -> '+model+' ('+(state.forced?'manual override':state.last.reason)+')');
      state.loggedPrompt=state.promptNumber;
    }
    const result=yield* next({...e,model});
    const usage=result?.usage;
    if (usage) {
      const actual=String(usage.model??model).toLowerCase();
      const family=['haiku','sonnet','opus'].find(name=>actual.includes(name));
      if (family) state.counts[family]++;
      state.usage.input+=Number(usage.input_tokens??0);
      state.usage.output+=Number(usage.output_tokens??0);
      state.usage.cacheRead+=Number(usage.cache_read_input_tokens??0);
      state.usage.cacheWrite+=Number(usage.cache_creation_input_tokens??0);
    }
    return result;
  });
}
