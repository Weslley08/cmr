// Lightweight, fully local classification. No model API calls and no user-data storage.
export function normalize(input) {
  return String(input ?? '').toLowerCase().normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '').trim();
}

const dangerous = /\b(deadlock|race condition|incident|incidente|security|seguranca|vulnerabil\w*|permiss\w*|permission\w*|production|producao|credential\w*|credencia\w*|autenticacao|authorization|autorizacao|leak|vazamento|multi.tenant|migracao de banco)\b/i;
const complex = /\b(arquitetur\w*|architecture|redesenh\w*|distributed|distribuid\w*|scalability|escalabilidade|consistencia|consistency|design system|system design|root cause|causa raiz)\b/i;
const changes = /\b(implemen\w*|implement\w*|crie|criar|create|alter\w*|change|modif\w*|edit\w*|corrij\w*|fix|refator\w*|deploy|pipeline|migration|migracao|teste|tests|build|atualiz\w*|upgrade|instal\w*)\b/i;
const simple = /^(liste|listar|mostre|mostrar|onde|encontre|localize|list|show|where|find|locate|search|busque|procure)\b/i;
const scoped = /\b(arquivos?|files?|pasta|pastas|folders?|readme|branch|commits?|git status|caminho|path|diretorios?|directories)\b/i;
const broad = /\b(todo o sistema|toda a base|todos os modulos|entire|whole|codebase|projeto completo|full repository)\b/i;

export function chooseModel(prompt, profile = 'balanced') {
  const s = normalize(prompt);
  // Negated write verbs in a read-only request are constraints, not mutations:
  // "Liste os arquivos. Nao modifique nada." should remain a Haiku candidate.
  // Other positive write verbs ("implemente", "refatore") still count as changes.
  const intent = s.replace(/\b(?:nao|nunca|sem|never|do not|don't)\s+(?:modif\w*|alter\w*|edit\w*|chang\w*|writ\w*|escrev\w*|creat\w*|cri\w*)\b/g, '');
  const p = ['economy', 'balanced', 'quality'].includes(profile) ? profile : 'balanced';
  // Explicit user override beats heuristics and can be reset with /cmr force none.
  const explicit = s.match(/\b(?:use|utilize|usar|modelo|model|forcar|force|rode com|execute com)\s+(?:o\s+)?(haiku|sonnet|opus)\b/);
  if (explicit) return {model:explicit[1],tier:'manual',reason:'explicit model request'};
  if (dangerous.test(s)) return {model:'opus',tier:'critical',reason:'high-risk task'};
  if (complex.test(s) || (broad.test(s) && changes.test(s))) {
    return {model:p==='economy'?'sonnet':'opus',tier:'complex',reason:'architecture or broad change'};
  }
  if (p !== 'quality' && s.length <= 240 && simple.test(s) && scoped.test(s) && !changes.test(intent)) {
    return {model:'haiku',tier:'simple',reason:'scoped read-only task'};
  }
  if (p === 'quality' && changes.test(s)) return {model:'opus',tier:'medium',reason:'quality profile'};
  return {model:'sonnet',tier:'medium',reason:'conservative default'};
}
