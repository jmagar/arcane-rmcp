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

The workflow is defined in [`.github/workflows/openwiki-update.yml`](../.github/workflows/openwiki-update.yml) and:

- runs on `workflow_dispatch` and a daily UTC schedule (`cron: 0 8 * * *`),
- installs Node.js 22 and globally installs OpenWiki,
- executes `openwiki code --update --print`, and
- uses OpenRouter with `OPENWIKI_PROVIDER=openrouter` and `OPENWIKI_MODEL_ID=z-ai/glm-5.2`.

## Environment and tracing

The job sets these relevant environment variables:

- `OPENROUTER_API_KEY` (required runtime secret for the configured provider),
- optional LangSmith tracing values: `LANGSMITH_API_KEY`, `LANGCHAIN_PROJECT`, `LANGCHAIN_TRACING_V2=true`.

## Pull request automation

The workflow uses `peter-evans/create-pull-request` to collect generated updates and includes these paths in `add-paths`:

- `openwiki`
- `AGENTS.md`
- `CLAUDE.md`
- `.github/workflows/openwiki-update.yml`

This allows documentation regeneration plus the two contributor guide files and workflow itself to land together in the update PR.

## Source-level context

See the edits that introduced this behavior:

- `.github/workflows/openwiki-update.yml`
- `AGENTS.md`
- `CLAUDE.md`

## Cross-links

- The top-level contributor entrypoint is [OpenWiki Quickstart](../quickstart.md).
- The contributor-facing OpenWiki policy in these files points people to this workflow page.
