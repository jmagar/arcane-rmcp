---
type: Playbook
title: OpenWiki Update Workflow
description: Documents the repository's OpenWiki automation workflow, including scheduler, generation command, provider settings, and required pull request output paths.
tags:
  - openwiki
  - workflow
  - automation
  - documentation
---

# OpenWiki Update Workflow

## Purpose

The OpenWiki workflow in this repository performs periodic and on-demand documentation regeneration for code-mode documentation. It is the primary mechanism for keeping `openwiki/` current when source or operations files change.

## Workflow behavior

The workflow is defined in [`.github/workflows/openwiki-update.yml`](../../.github/workflows/openwiki-update.yml) and:

- runs on `workflow_dispatch` and a daily UTC schedule (`cron: 0 8 * * *`),
- connects to the private API over Tailscale and verifies that API before generation,
- installs Node.js 22 and OpenWiki 0.2.0,
- executes `openwiki --update --print`, and
- uses the OpenAI-compatible provider at `http://100.120.242.29:8317/v1` with the `gpt-5.3-codex-spark` model.

## Environment and tracing

The job uses these relevant secrets and environment variables:

- `TS_OAUTH_CLIENT_ID` and `TS_OAUTH_SECRET` connect the runner to Tailscale,
- `OPENAI_COMPATIBLE_API_KEY` authenticates to the configured API,
- `OPENAI_COMPATIBLE_BASE_URL=http://100.120.242.29:8317/v1`, and
- `OPENWIKI_PROVIDER=openai-compatible` with `OPENWIKI_MODEL_ID=gpt-5.3-codex-spark`.

Before running OpenWiki, the workflow calls the API's `/models` endpoint and fails if credentials are absent or the endpoint does not return HTTP 200.

## Pull request automation

The workflow uses `peter-evans/create-pull-request` to collect the generated `openwiki` directory on the `openwiki/update` branch. Other repository files are not included by its `add-paths` setting.

## Source-level context

The workflow file is the source of truth for its schedule, provider, command, network setup, and pull request paths. Regenerate these pages after changing that workflow.

## Cross-links

- The top-level contributor entrypoint is [OpenWiki Quickstart](../quickstart.md).
