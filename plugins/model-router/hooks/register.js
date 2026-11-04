import { classify, profileName } from './router-core.js';

// Per-session local state. No project/config writes; RTK and user security hooks stay untouched.
export function register(on) {
  const state = {
    enabled: false, profile: 'balanced', forced: null, selected: null,
    promptNumber: 0, notified: -1,
    requests: { haiku: 0, sonnet: 0, opus: 0 },
    input: 0, output: 0, cacheRead: 0, cacheWrite: 0,
  };
  on('session.start', async ($, e, next) => {
    await $.command.register({
      name: 'cmr',
      description: 'CMR: status | auto | off | mode | force | stats',
      argumentHint: '[status|auto|off|mode economy|balanced|quality|force haiku|sonnet|opus|none|stats]',
    });
    return next(e);
  });
  on('command.run', { command: 'cmr' }, async ($, e) => {
    const [action = 'status', value] = String(e.args ?? '').trim().toLowerCase().split(/\s+/);
    if (action === 'auto') { state.enabled = true; return { text: 'CMR enabled (' + profileName(state.profile) + '). Use /cmr off to disable.' }; }
    if (action === 'off') { state.enabled = false; return { text: 'CMR disabled. Claude Code controls the model.' }; }
    if (action === 'mode' && ['economy', 'balanced', 'quality'].includes(value)) {
      state.profile = value; state.selected = null;
      return { text: 'CMR profile: ' + profileName(value) + '. Auto: ' + (state.enabled ? 'on' : 'off') };
    }
    if (action === 'force' && ['haiku', 'sonnet', 'opus', 'none'].includes(value)) {
      state.forced = value === 'none' ? null : value;
      return { text: 'CMR force: ' + (state.forced ?? 'none') };
    }
    if (action === 'stats') {
      const r = state.requests;
      return { text: 'CMR routed calls: Haiku ' + r.haiku + ', Sonnet ' + r.sonnet + ', Opus ' + r.opus + '; input ' + state.input + ', output ' + state.output + ', cache read ' + state.cacheRead + ', cache write ' + state.cacheWrite + '. Approximate observed usage; not subscription savings.' };
    }
    if (action !== 'status') return { text: 'Usage: /cmr status | auto | off | mode economy|balanced|quality | force haiku|sonnet|opus|none | stats' };
    return { text: 'CMR ' + (state.enabled ? 'ON' : 'OFF') + '; profile ' + profileName(state.profile) + '; force ' + (state.forced ?? 'none') + '; selection ' + (state.selected?.model ?? 'pending') };
  });
  on('prompt.submit', async ($, e, next) => {
    state.promptNumber++;
    state.selected = classify(e.text, state.profile);
    state.notified = -1;
    return next(e);
  });
  on('turn.step', async function* ($, e, next) {
    if (!state.enabled || !state.selected || e.agentId) return yield* next(e);
    const model = state.forced ?? state.selected.model;
    if (state.notified !== state.promptNumber) {
      $.ui.log('CMR -> ' + model + ' (' + (state.forced ? 'manual override' : state.selected.reason) + ')');
      state.notified = state.promptNumber;
    }
    const result = yield* next({ ...e, model });
    if (result?.usage) {
      const actual = String(result.usage.model ?? model).toLowerCase();
      const family = ['haiku', 'sonnet', 'opus'].find((name) => actual.includes(name));
      if (family) state.requests[family]++;
      state.input += Number(result.usage.input_tokens ?? 0);
      state.output += Number(result.usage.output_tokens ?? 0);
      state.cacheRead += Number(result.usage.cache_read_input_tokens ?? 0);
      state.cacheWrite += Number(result.usage.cache_creation_input_tokens ?? 0);
    }
    return result;
  });
}
