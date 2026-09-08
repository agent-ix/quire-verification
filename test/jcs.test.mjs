// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import test from "node:test";

import { jcsCanonicalize, jcsEqual, jcsSha256 } from "../src/index.mjs";

test("uses RFC 8785 number serialization and property ordering", () => {
  assert.equal(
    jcsCanonicalize({
      z: 0,
      numbers: [333333333.33333329, 1e30, 4.5, 2e-3, 1e-27],
      a: 1,
    }),
    '{"a":1,"numbers":[333333333.3333333,1e+30,4.5,0.002,1e-27],"z":0}',
  );
});

test("JCS identity ignores object insertion order and preserves array order", () => {
  assert.equal(jcsEqual({ b: 2, a: 1 }, { a: 1, b: 2 }), true);
  assert.equal(jcsEqual([1, 2], [2, 1]), false);
  assert.equal(jcsSha256({ b: 2, a: 1 }), jcsSha256({ a: 1, b: 2 }));
  assert.match(jcsSha256({ a: 1 }), /^sha256-jcs:[0-9a-f]{64}$/);
});

test("rejects values outside the RFC 8785 JSON domain", () => {
  for (const value of [NaN, Infinity, undefined, { value: undefined }, new Date()]) {
    assert.throws(() => jcsCanonicalize(value), /non-|JSON/);
  }
  const sparse = [];
  sparse.length = 1;
  assert.throws(() => jcsCanonicalize(sparse), /sparse/);
  const cyclic = {};
  cyclic.self = cyclic;
  assert.throws(() => jcsCanonicalize(cyclic), /cycle/);
  assert.throws(() => jcsCanonicalize("\ud800"), /ill-formed Unicode/);
});
