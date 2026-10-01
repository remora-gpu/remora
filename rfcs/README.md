# RFCs

Substantial changes to Remora go through a lightweight RFC process so that design decisions are written down before code lands.

**Needs an RFC:** wire protocol changes, new transports, new API families, public CLI/config changes, security model changes, anything that is hard to reverse.
**Does not:** bug fixes, new functions within an already supported API family, docs, refactoring.

## Process
1. Copy `0000-template.md` to `NNNN-short-title.md` (next free number) and open a PR.
2. Discussion happens on the PR. Maintainers mark it *Final comment period* (7 days) once consensus forms.
3. Merged = accepted. Implementation may then start; the RFC links its tracking issue.
4. Rejected RFCs are closed with a summary of why.
