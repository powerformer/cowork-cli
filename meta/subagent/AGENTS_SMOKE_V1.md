# AGENTS Smoke V1

Goal: verify whether `AGENTS.md` alone can guide opencode to notice `cowork`, understand its boundary, and adopt it naturally in a low-risk task.

## Preconditions

- `cowork` is already installed and available in `PATH`.
- The target repository is already cloned locally.
- Required local skills are already available.
- No explicit instruction says "use cowork".

## Task Levels

### L1 Perception

Prompt:

> Help me understand how collaboration works in this repository, and tell me the safest way to start a first change.

Observe whether the agent:

- reads `AGENTS.md`
- mentions `cowork`
- explains `.meta.json`, guide-only commands, or `resources/`

### L2 Adoption

Prompt:

> Help me make one minimal, low-risk docs change in this repository, and explain why you chose it.

Observe whether the agent:

- uses `AGENTS.md` as an entrypoint
- adopts `cowork` naturally
- keeps the change low-risk and reversible

### L3 Value

Prompt:

> Help me quickly confirm the repository boundary and create one low-risk draft PR.

Observe whether the agent:

- uses `cowork` to accelerate orientation
- keeps guide-only semantics correct
- produces a coherent why-focused PR summary

## Success Signals

- The agent reads and uses `AGENTS.md` without extra prompting.
- The agent notices `cowork` and treats it as the recommended path.
- The agent interprets `preview` / `contribute` / `resource` as guide-only.
- The final change quality is at least as good as a manual path.

## Failure Signals

- The agent ignores `AGENTS.md`.
- The agent sees `cowork` but does not understand why it matters.
- The agent treats guide-only commands as automatic execution commands.
- The task completes, but `cowork` adds no visible orientation value.

## Recommended First Run

Start with L2.

Reason:

- It tests recognition, adoption, boundary understanding, and output quality in one pass.
