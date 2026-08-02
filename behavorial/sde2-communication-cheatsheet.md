# Communication & Office Politics Cheat Sheet
### SDE 2 Edition — Owning Ambiguity, Not Just Tickets

---

## What actually changes at SDE 2

At junior level, the bar is: *"can I trust you to execute what I hand you?"*
At SDE 2, the bar shifts to: *"can I hand you a fuzzy problem and trust the shape of the solution you bring back?"*

This means your communication needs to shift too:
- Less "what should I do" → more "here's what I think we should do, and why"
- Less reacting to tickets → more shaping what the tickets even are
- You start being expected to disagree, push back on estimates, and represent your team to others — not just execute quietly

---

## 1. Driving Technical Decisions (not just implementing them)

You're now expected to *propose*, not just ask.

- "I've been looking at this — I think we should go with [X] over [Y] because [trade-off]. Want to sanity-check before I write it up properly?"
- "There are two viable approaches here. [A] is faster to ship but harder to extend. [B] is the opposite. Given [context/deadline], I'd lean [A] — thoughts?"
- "I want to write a short design doc for this before starting — it touches [system], want to make sure we're aligned before I sink time in."

**Key shift:** bring a recommendation with reasoning, not a menu of options with no opinion. A menu with no opinion reads as "not ready for ownership yet."

---

## 2. Pushing Back on Estimates / Scope (from PM, EM, or Lead)

- "I can commit to [smaller scope] by [date] with confidence. The full ask would need [more time/resources] — want me to break down where the time goes?"
- "Just flagging: this estimate assumes [X] is stable. If [X] changes, the timeline changes too — want to lock that down first?"
- "I don't think [date] is realistic without cutting [feature/quality bar] — which would you rather cut?"
- "Before I commit to a number — is this date hard (external dependency/launch) or soft (nice-to-have)? Changes how much I'd push to hit it."

**Underlying principle:** Never just absorb an unrealistic deadline silently — surface the trade-off and let the business decide what to cut. Silent absorption at SDE2 reads as poor judgment, not heroics.

---

## 3. Disagreeing With Peers / Seniors as a Near-Equal

You're expected to hold a technical position now, respectfully but firmly.

- "I hear the reasoning, but I still think [X] has a real risk here — [specific scenario]. Can we pressure-test it before committing?"
- "I'm not fully convinced — can we get a third opinion, or prototype both quickly to compare?"
- "I'll defer if the team wants to move forward with [X], but I want it on record that I flagged [risk] — can we revisit if [signal] shows up?"

**Escalation ladder when you disagree and can't resolve it 1:1:**
1. Discuss directly with the person first.
2. If unresolved, propose a neutral tie-breaker: data, prototype, or a third opinion.
3. Only loop in a manager/lead if it's genuinely blocking and the above failed — frame it as "need a decision," not "they're wrong."

---

## 4. Mentoring Junior Engineers (new responsibility at this level)

- Instead of giving the answer: "What have you tried so far? What's your hypothesis?"
- Instead of fixing their PR yourself: "Nit vs. blocking — this one's blocking because [reason], here's a pointer to a similar pattern in [file]."
- When they're stuck and time matters: "Let's pair for 15 min — I'll explain my thinking as we go so it's useful next time too."
- Giving critical feedback: "This works, but here's why I'd structure it differently — [reasoning], not just a style preference."

**Watch for:** junior engineers won't always tell you they're blocked. Check in proactively rather than waiting for them to escalate — that instinct is part of what gets noticed for promotion.

---

## 5. Cross-Team Negotiation (asking for things, not just help)

At SDE 2 you're often negotiating priority, not just asking a favor.

- "This is blocking [business impact] on our side — can we get this prioritized this sprint, or help us understand what it'd take to unblock ourselves?"
- "We can build a workaround on our end, but it adds [cost/tech debt] — is that worth avoiding a dependency on your team, or would you rather we wait for the real fix?"
- "What would it take for your team to prioritize this? Happy to pair or contribute code if that helps move it up."
- When another team pushes back: "Understood you're slammed — can we agree on a rough timeline so we can plan around it either way?"

**Key move:** offer to contribute, not just request. "I can write the PR if you review it" gets far more yeses than a pure ask.

---

## 6. Managing Up (Your Manager / EM)

Your manager increasingly needs *you* to summarize impact, not just status.

- "Here's what shipped this sprint, and here's the actual impact: [metric/outcome], not just 'done.'"
- "I want to take on more design ownership — is there a project coming up where I could lead the technical approach?"
- "I think I'm ready for [bigger scope/promo-relevant work] — what would you need to see from me to make that case?"
- Flagging team-level issues (not just your own): "I've noticed [pattern] is slowing the team down — might be worth raising, wanted your take first."

**Promotion-relevant habit:** proactively summarize your impact in writing (design docs, postmortems, project retros) — don't rely on your manager remembering everything you did.

---

## 7. Handling Ambiguous / Underspecified Asks

At SDE 2, you're expected to fill gaps, not wait for full specs.

- "This is underspecified in a few places — here's how I'm interpreting it: [assumptions]. Flag now if any of these are wrong."
- "I'll make a reasonable call on [smaller decision] to keep moving, but want explicit sign-off on [bigger decision] before I commit."
- "Rather than blocking on a full spec, I'll build the core flow and we can iterate on edge cases — sound OK?"

**Anti-pattern to avoid:** going silent for days trying to fully nail down every detail before starting. Bias toward stating assumptions and moving, with checkpoints.

---

## 8. Owning Incidents / Postmortems

At SDE 2 you may lead incident response or write the postmortem, not just fix the bug.

- During: "Status: [root cause found / still investigating]. Mitigation: [action]. ETA on next update: [time]."
- In postmortems — own it without spiraling: "The root cause was [X]. Here's what we're changing so this class of bug doesn't recur: [action items with owners]."
- Avoid blame language, even about your own past decisions: "We didn't have monitoring on [X]" not "I should have added monitoring."

---

## 9. Saying No at This Level (More Leverage, More Nuance)

You can push back harder now, but it must come with reasoning and alternatives — not just refusal.

- "I don't think this is the right time for [X] — here's the risk: [reason]. Can we revisit after [milestone]?"
- "I'll own this, but I want to be upfront: it'll come at the cost of [other commitment]. Are we OK with that trade-off?"
- "I'd rather not rubber-stamp this without a design review — too much blast radius if it's wrong. Can we take 30 min first?"

---

## 10. Quick Reference — SDE2 Tone Swaps

| Junior instinct | SDE2 move |
|---|---|
| "What should I do?" | "Here's what I'd do and why — sanity check me?" |
| Silently accepting a deadline | "Here's the trade-off if we hit that date — which do we cut?" |
| Waiting for a full spec | "Here's my interpretation — flag if wrong, I'll start now." |
| Fixing a junior's bug myself | "What have you tried? Let's reason through it together." |
| Reporting only "done" tasks | Reporting outcomes/impact, not just task completion |
| Avoiding conflict with peers | Disagreeing respectfully, backed by data or a proposed test |
| Waiting to be asked to lead | Proactively proposing ownership of ambiguous work |

---

### The core shift, in one line

**Junior earns trust by executing reliably. SDE 2 earns trust — and promotion signal — by making good calls under ambiguity and communicating those calls clearly enough that others don't have to double-check your judgment.**
