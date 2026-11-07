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
  // Free-form prompt mentions do not override /cmr force.
  ['Use o Haiku para localizar arquivos','quality','sonnet'],
  ['Explique o funcionamento de uma API','balanced','sonnet'],
  ['Não force Haiku, Sonnet ou Opus.','balanced','sonnet'],
  ['Nao use Haiku. Use Balanced.','balanced','sonnet'],
  ['Não force Haiku, Sonnet ou Opus. Investigue arquitetura e segurança.','balanced','opus'],
  ['Investigue a arquitetura e a segurança. Não force Haiku, Sonnet ou Opus.','economy','opus'],
  ['Liste arquivos do src. Não force Haiku, Sonnet ou Opus.','balanced','haiku'],
  ['Use o Opus para localizar arquivos','economy','sonnet'],
  ['Não force Haiku, Sonnet ou Opus. Mantenha CMR Balanced. Implemente a arquitetura e os testes da galeria 3x2.','balanced','opus'],
];
for(const [prompt,profile,model] of cases)
  test(prompt+' ('+profile+')',()=>assert.equal(chooseModel(prompt,profile).model,model));
