import { readFileSync, existsSync } from 'node:fs';
import assert from 'node:assert/strict';
const read = (p) => JSON.parse(readFileSync(p, 'utf8'));
const marketplace = read('.claude-plugin/marketplace.json');
assert.equal(marketplace.name, 'weslley08-cmr');
assert.equal(marketplace.owner.name, 'Weslley08');
assert.ok(Array.isArray(marketplace.plugins) && marketplace.plugins.length === 1);
const plugin = marketplace.plugins[0];
assert.equal(plugin.name, 'model-router');
assert.equal(plugin.source, './plugins/model-router');
const root = 'plugins/model-router/';
assert.ok(existsSync(root + '.claude-plugin/plugin.json'));
assert.equal(read(root + '.claude-plugin/plugin.json').name, plugin.name);
for (const module of read(root + 'hooks/hooks.json').modules) {
  assert.ok(module.startsWith('./'));
  assert.ok(existsSync(root + 'hooks/' + module.slice(2)));
}
console.log('CMR marketplace JSON and plugin paths OK');
