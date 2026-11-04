import test from 'node:test';
import assert from 'node:assert/strict';
import {chooseModel} from '../hooks/policy.mjs';

const cases=[
  ['Liste arquivos do src','balanced','haiku'],
  ['Where is the README file?','balanced','haiku'],
  ['Implemente um endpoint com testes','balanced','sonnet'],
  ['Refatore a arquitetura distribuida','balanced','opus'],
  ['Investigue um deadlock em produção','economy','opus'],
  ['Refatore a arquitetura distribuida','economy','sonnet'],
  ['Crie um componente Angular','quality','opus'],
  ['Liste arquivos do src','quality','sonnet'],
  ['Use o Haiku para localizar arquivos','quality','haiku'],
  ['Explique o funcionamento de uma API','balanced','sonnet'],
];
for(const [prompt,profile,model] of cases)
  test(prompt+' ('+profile+')',()=>assert.equal(chooseModel(prompt,profile).model,model));
