import test from 'node:test';
import assert from 'node:assert/strict';
import {chooseModel} from '../hooks/policy.mjs';

const cases=[
  ['Liste arquivos do src','balanced','haiku'],
  ['Liste os arquivos do diretório src deste projeto. Não modifique nada.','balanced','haiku'],
  ['List files from src. Do not modify anything.','balanced','haiku'],
  ['Liste arquivos do src, mas implemente tambem um endpoint','balanced','sonnet'],
  ['Liste arquivos e não modifique nada, mas implemente um endpoint','balanced','sonnet'],
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
