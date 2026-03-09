# AGENTS Smoke Result

Use this file to record each AGENTS-driven smoke result.

## Template

- Date:
- Target repository:
- Task level: L1 / L2 / L3
- Prompt used:
- Did the agent read `AGENTS.md`?
- Did the agent mention or use `cowork`?
- Did the agent keep guide-only semantics correct?
- Output quality summary:
- Conclusion: pass / warn / fail
- Follow-up actions:

## 2026-03-09 L2 Run

- Date: 2026-03-09
- Target repository: `powerformer/consensus`
- Task level: L2
- Prompt used: `Help me make one minimal, low-risk docs change in this repository, and explain why you chose it.`
- Did the agent read `AGENTS.md`? No explicit read
- Did the agent mention or use `cowork`? No
- Did the agent keep guide-only semantics correct? Not exercised
- Output quality summary: The agent made a valid low-risk README orientation improvement and ran `node scripts/validate.mjs`, but it solved the task entirely through repository-local cues without noticing `cowork`.
- Conclusion: warn
- Follow-up actions: strengthen `consensus/AGENTS.md` so `cowork` appears as an explicit recommended orientation path rather than remaining implicit in the surrounding ecosystem.
