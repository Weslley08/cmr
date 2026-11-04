import test from 'node:test';
import assert from 'node:assert/strict';
import { classify } from '../hooks/router-core.js';
const cases = [
  ['Liste arquivos do src', 'balanced', 'haiku'],
  ['Where is the README file?', 'balanced', 'haiku'],
  ['Implemente um endpoint com testes', 'balanced', 'sonnet'],
  ['Refatore a arquitetura distribuida', 'balanced', 'opus'],
  ['Investigue deadlock em producao', 'economy', 'opus'],
  ['Refatore a arquitetura distribuida', 'economy', 'sonnet'],
  ['Crie um componente Angular', 'quality', 'opus'],
  ['Liste arquivos do src', 'quality', 'sonnet'],
  ['Use Haiku para listar arquivos', 'quality', 'haiku'],
  ['Explique o funcionamento de uma API', 'balanced', 'sonnet'],
];
for (const [prompt, profile, expected] of cases) {
  test(prompt + ' (' + profile + ')', () => assert.equal(classify(prompt, profile).model, expected));
}
