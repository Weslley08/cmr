// Deterministic local policy: no API calls, no credentials and no project reads.
export function normalize(value) {
  return String(value ?? '').toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '').trim();
}
const critical = /\b(secur\w*|seguranca|vulnerabil\w*|exploit|credencia\w*|authz|oauth|autorizacao|permission|permiss\w*|producao|production|incidente|incident|vazamento|data loss|deadlock|race condition|concorrencia|migracao de banco|database migration|multi.?tenant|isolamento de tenant)\b/;
const complex = /\b(arquitetur\w*|architecture|redesenh\w*|distribuid\w*|distributed|escalabilidade|scalability|consistencia|consistency|root cause|causa raiz|investiga\w*|system design)\b/;
const write = /\b(implemen\w*|crie|criar|construa|desenvolv\w*|altere|alterar|modifique|corrig\w*|fix|refator\w*|test\w*|deploy|build|pipeline|endpoint|feature|adicione|remova|configur\w*|update|upgrade|write|create|implement|change|modify|edit|repair)\b/;
const simple = /^(liste|listar|mostre|mostrar|qual|quais|onde|encontre|localize|busque|procure|resuma|explique|list|show|find|locate|search|summarize|what|where|which)\b/;
const scope = /\b(arquivos?|files?|pastas?|diretorio\w*|folders?|comandos?|commands?|path|caminho|branch|commit|status|nome\w*|names?|logs?|readme)\b/;
const broad = /\b(todos? os (?:projetos|modulos|arquivos)|entire|whole|todo o (?:repositorio|sistema|projeto)|completo|completa|exaustiv\w*|deep|profund\w*)\b/;
const explicit = /\b(?:use|usar|utilize|modelo|model|force|forcar|rode com|execute com)\s+(?:o\s+)?(haiku|sonnet|opus)\b/;
export function classify(prompt, profile = 'balanced') {
  const text = normalize(prompt);
  const chosen = text.match(explicit);
  if (chosen) return { model: chosen[1], reason: 'Explicit model in prompt', tier: 'manual' };
  if (critical.test(text)) return { model: 'opus', reason: 'Critical/sensitive change', tier: 'critical' };
  if (complex.test(text) || (broad.test(text) && write.test(text))) {
    return { model: profile === 'economy' ? 'sonnet' : 'opus', reason: 'Complex architecture or broad change', tier: 'high' };
  }
  if (simple.test(text) && !write.test(text) && !broad.test(text) && text.length <= 250 && (scope.test(text) || text.length < 120) && profile !== 'quality') {
    return { model: 'haiku', reason: 'Scoped read-only task', tier: 'low' };
  }
  if (write.test(text) && profile === 'quality') return { model: 'opus', reason: 'Quality profile: implementation', tier: 'medium' };
  return { model: 'sonnet', reason: 'Conservative default', tier: 'medium' };
}
export const profileName = (p) => ({ economy: 'economy', balanced: 'balanced', quality: 'quality' }[p] ?? p);
