# Communication & Office Politics Cheat Sheet
### For working with Team Leads / Staff Engineers (Junior/Novice Engineer Edition)

---

## 1. Escalating a Problem

**Formula:** Problem → What you tried → Your best guess/recommendation → Specific ask

- "I'm seeing [X error/behavior]. I tried [A] and [B], neither fixed it. My guess is [Y] — could you sanity check that, or point me somewhere?"
- "Before I dig further — is this a known issue, or should I keep debugging?"
- "I want to flag this early: [problem]. Not blocked yet, but might be by [day] if we don't sort out [X]."
- "Quick gut check — am I overcomplicating this, or is [approach] actually the right call here?"

**Avoid:**
- "It's not working, what do I do?" (no context = you look like you didn't try)
- Silently struggling for 2 days before saying anything

---

## 2. Asking for Time / Help

- "Got 15 min today or tomorrow? I've got 3 quick things to run by you."
- "No rush at all — whenever you have a sec, could you glance at [X]?"
- "Sorry to interrupt — quick question, is now an OK time or should I come back later?"
- "I'll async this in case you're heads-down: [question + context in one message]"

**Rule of thumb:** Batch questions. Don't ping 5 times in an hour — it fragments their focus and makes you seem less prepared.

---

## 3. Code Review — Giving Feedback (as a junior, to peers/seniors)

- "Nit: [minor thing]" — signals non-blocking.
- "Is there a reason we're not caching this here? Might be missing context."
- "Curious about this approach — was [alternative] considered? Not pushing back, just trying to learn the reasoning."
- "This might just be a style preference, but would [X] be clearer here?"

## 3b. Code Review — Receiving Feedback

- "Good catch — fixing now."
- "I see the concern. I went with [X] because [reason] — open to switching if you think [Y] is better in practice."
- "Can you say more about why this pattern is preferred here? Want to actually understand it, not just change it."
- If you disagree and have data: "I benchmarked this — [A] was Xms faster than [B] in our case. Still open to being wrong, just wanted to share the numbers."

---

## 4. Disagreeing With a Staff Engineer / Lead

**Calibrate confidence to your certainty:**

Low confidence → ask, don't assert:
- "Genuinely asking — why not [X] instead? Might be missing context."

Medium confidence → flag, don't declare:
- "I think there might be an issue with this approach — [specific reason]. Let me know if I'm missing something."

High confidence (you have evidence) → present the evidence, not the verdict:
- "I ran into [specific error/log/benchmark] with this — here's what I found: [data]. Wanted to flag before we ship it."

**Never say:** "That's wrong" / "That doesn't make sense" — without something concrete backing it up.

---

## 5. Declining / Pushing Back on Extra Work

**Never a flat "no." Always show the trade-off and let them decide:**

- "I can take this on, but it'll push [current task] back by ~2 days — which is the priority?"
- "Happy to help — should I context-switch now, or finish [current ticket] first?"
- "I don't think I'm the right owner for this — [teammate] knows that area better, want me to loop them in?"
- "I can get you a rough version by Wednesday, but the full thing won't be ready till Friday — does that work?"
- "That's outside what I've been working on — can you help me understand where it should slot in relative to [current priority]?"

**Why this works:** You're not refusing, you're making the invisible cost visible. Silently absorbing extra work just teaches people you have infinite bandwidth — which becomes the new normal, and it's on you to correct that.

---

## 6. Closing the Loop (an underrated trust-builder)

After someone helps you — always follow up. This single habit compounds trust faster than almost anything else.

- "That fixed it — turned out to be [root cause]. Thanks!"
- "Following up on this — went with your suggestion, works great now."
- "Wanted to close the loop: shipped this yesterday, no issues so far."

---

## 7. Delivering Bad News

Always be the first source of bad news about your own work — never let them hear it from someone else.

- "Wanted to flag before standup: [issue]. Here's what I'm doing about it: [plan]. Might need [X] from you."
- "Heads up, I broke [X] in staging — already reverted, investigating root cause now, will update by EOD."
- "This is going to miss the deadline. Here's why: [reason]. Here's the earliest realistic date: [date]."

**Framing tip:** Bad news + a plan lands very differently than bad news alone.

---

## 8. Asking Other Teams for Help

State the ask, the why, and the deadline in the first two lines — people triage by scanning.

- "Hey — need 10 min of your time on [specific thing]. Context: [1 sentence]. Trying to unblock [X] by [day], no rush if you're slammed, just let me know timing."
- "Quick question for your team: does [system] support [X]? Trying to figure out if I need to build this myself or if it already exists."

**Come with a proposed solution when possible:**
- "I think we should do [X] because [Y] — open to other approaches if you see issues."

This gets far more traction than an open-ended "what should we do?"

---

## 9. 1:1s and Status Updates

**With your Team Lead** — keep it execution-focused:
- "Here's where I'm at: [ticket status]. Blocked on [X]. On track for [Y]."

**With your Manager** — keep it impact/growth-focused, not just task-listing:
- "Finished [X], and I think it moved the needle on [broader thing]. I want to get better at [skill] — any suggestions on how to practice that?"

**General 1:1 hygiene:**
- Send a 1-line agenda beforehand if it's not a routine sync.
- Bring 1 growth-oriented question occasionally, not just status.

---

## 10. Junior-Specific Traps to Avoid

| Trap | Why it hurts | Do instead |
|---|---|---|
| Asking questions Google/docs would answer in 2 min | Signals low effort | Spend 5–10 min trying first, then ask with context |
| Silently overloading yourself | Becomes the expected norm | Flag trade-offs out loud (see #5) |
| Arguing to "win" instead of to understand | Reads as ego, not curiosity | Ask questions before asserting positions |
| Disappearing after getting help | Erodes trust over time | Always close the loop (see #6) |
| Waiting till the deadline to flag a delay | Removes everyone's ability to react | Flag blockers on day 1–2, not day 5 |
| Bare "no" to a request | Reads as unhelpful even when correct | Always pair a decline with a trade-off or alternative |

---

## 11. Quick Reference — Tone Swaps

| Instead of... | Say... |
|---|---|
| "That's wrong." | "I have a concern about this — [reason]." |
| "I don't know." | "Let me check and get back to you by [time]." |
| "Can you help me?" | "I'm seeing [specific issue] — tried [X, Y] — any ideas?" |
| "No, I can't do that." | "I can, but it means [trade-off] — which do you want prioritized?" |
| "Why did you do it this way?" (in review) | "Is there a reason we went with [X] here?" |
| Matching an aggressive tone in chat | Replying calmer than they were — always |

---

### The one habit that beats all of these

**Visibility + calm, specific communication when blocked or in disagreement.**
Most of what reads as "politics" is really just: does this person make it easy to trust and work with them? Everything on this sheet is in service of that.
