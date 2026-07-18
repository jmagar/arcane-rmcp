"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const repoRoot = path.resolve(__dirname, "..", "..", "..");

test("release-please paths cover and transform every published version", () => {
  const config = JSON.parse(fs.readFileSync(path.join(repoRoot, "release-please-config.json"), "utf8"));
  const server = JSON.parse(fs.readFileSync(path.join(repoRoot, "server.json"), "utf8"));
  const paths = config.packages["."]["extra-files"]
    .filter((entry) => entry && entry.type === "json" && entry.path === "server.json")
    .map((entry) => entry.jsonpath);
  const next = "9.9.9";

  for (const jsonpath of paths) {
    if (jsonpath === "$.version") {
      server.version = next;
    } else if (jsonpath === "$.packages[?(@.identifier == 'arcane-rmcp')].version") {
      const target = server.packages.find((entry) => entry.identifier === "arcane-rmcp");
      assert.ok(target, "semantic package selector must resolve a live target");
      target.version = next;
    } else if (jsonpath.endsWith(".distribution.npm")) {
      server._meta["io.modelcontextprotocol.registry/publisher-provided"].distribution.npm = `arcane-rmcp@${next}`;
    } else if (jsonpath.endsWith(".buildInfo.version")) {
      server._meta["io.modelcontextprotocol.registry/publisher-provided"].buildInfo.version = next;
    } else {
      assert.fail(`unrecognized release metadata path: ${jsonpath}`);
    }
  }

  const publisher = server._meta["io.modelcontextprotocol.registry/publisher-provided"];
  assert.equal(server.version, next);
  assert.equal(server.packages.find((entry) => entry.identifier === "arcane-rmcp").version, next);
  assert.equal(publisher.distribution.npm, `arcane-rmcp@${next}`);
  assert.equal(publisher.buildInfo.version, next);
});
