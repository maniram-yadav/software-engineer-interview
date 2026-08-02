# Communication & Office Politics Cheat Sheet
### SDE 3 / Senior Engineer Edition — Influence Without Authority

---

## What actually changes at SDE 3

At SDE 2, the bar is: *"can I hand you ambiguity and trust your judgment on it?"*
At SDE 3, the bar shifts to: *"can I trust you to set direction for other people, including people who don't report to you and don't have to listen to you?"*

This is the level where technical skill stops being the bottleneck and **influence, narrative, and political navigation** become the actual job. You'll spend more time convincing people than coding. That's not a bug — it's the role.

---

## 1. Influencing Without Authority

This is the central skill at this level. You often can't order anyone to do anything — you have to make the right thing the easy thing to agree to.

- "I want to make the case for [X]. Can I get 20 min to walk you through the trade-offs before we lock the roadmap?"
- "I'm not asking you to change course today — just flagging a risk I think we should have eyes on before Q[X] planning."
- "If we don't solve [X] now, here's what it costs us in 6 months: [concrete cost]. If we do, here's the near-term cost: [concrete cost]. I think it's worth it — here's why."
- Building a coalition before the room: talk to 2–3 key people 1:1 *before* the group meeting. If they're already nodding, the meeting is a formality, not a fight.

**Key mindset:** you win by making your reasoning legible and your trade-offs honest — not by being the loudest or most senior voice in the room.

---

## 2. Setting Technical Direction / Architecture

- "I want to propose we standardize on [X] across teams. Here's the cost of not doing it: [duplicated effort / inconsistent reliability / etc]. Open to pushback."
- "This is a one-way door — hard to reverse once we commit. Worth an extra week of design review before we start."
- "This is a two-way door — let's just try it and adjust; not worth over-designing."
- Framing a design doc for buy-in, not just documentation: state the problem and constraints *before* the solution, so readers arrive at your answer with you instead of being told it.

**Anti-pattern:** presenting the final architecture without showing the alternatives you rejected and why. People trust decisions more when they can see the discarded options.

---

## 3. Disagreeing With Other Senior/Staff Engineers or EMs

At this level, disagreements are higher-stakes and more political — they can shape roadmaps, not just PRs.

- "I want to flag a strong disagreement here, not just a preference — here's my reasoning: [specific, data-backed]."
- "I'll support the team's decision either way, but I want it documented that I raised [concern], so we can course-correct fast if it shows up."
- When two senior people disagree and it's stuck: "Let's timebox this — pick a decision-maker, set a revisit date, and move. Debating in circles costs us more than picking wrong and correcting."
- Disagreeing with your own manager: "I see the business reasoning, but from an engineering-risk lens, I'd push back on [X] — can I lay out the specific risk before we finalize?"

**Rule:** disagree loudly in the room, commit fully once decided ("disagree and commit"). Undermining a decision after the fact — even one you opposed — destroys trust fast at this level.

---

## 4. Managing Multiple Stakeholders With Conflicting Priorities

- "I'm hearing [Team A] wants [X] and [Team B] wants [Y] — those conflict directly. Someone needs to make this call; here's my recommendation and why."
- "I can't optimize for all three of [speed, reliability, scope] at once — which is the actual priority this quarter?"
- To a PM pushing for scope: "I can hit that date if we cut [Y], or hit full scope if we move the date — I won't silently do both by cutting corners on quality."
- Translating engineering trade-offs into business language: "This isn't just tech debt — it's a growing risk of [outage type], which would cost us [business impact]."

---

## 5. Representing Engineering to Non-Engineering Stakeholders

- To Product/Design: "I want to push back on this requirement — not because it's hard, but because [X] will create [specific downstream problem]. Can we solve the underlying need differently?"
- To Leadership (skip-levels, exec reviews): lead with business impact, then technical detail only if asked. "This reduced latency 40%, which translates to [X] in conversion." Not "we migrated to a new caching layer."
- When leadership pushes an unrealistic timeline: "I want us to hit that too. Here's the real constraint, and here's what would need to be true to hit it — more headcount, reduced scope, or a later date. Your call which lever."

**Key skill:** translate technical reality into business consequences without dumbing it down or being condescending. This is what actually gets you heard by people who don't have your context.

---

## 6. Mentoring at Scale (Beyond 1:1 Mentoring)

- Writing things down once instead of repeating advice 1:1 — internal design docs, RFC templates, onboarding guides become your leverage.
- "Instead of me reviewing every PR in this area, let's set up a lightweight guideline doc so the team can self-serve." (You scale by removing yourself as a bottleneck.)
- Sponsoring others' visibility, not just their skill: "I think [engineer] should present this at the next tech review — they did the hard part, they should get the credit."
- Giving feedback to peers, not just juniors: "Something I noticed in that incident — want my honest take, or just venting space right now?" (Always ask before giving unsolicited peer feedback.)

---

## 7. Navigating Org Politics / Reorgs / Ambiguous Ownership

- When ownership is unclear between teams: "Rather than debate who owns this abstractly, can we agree on outcomes each team is responsible for? Ownership follows from that."
- When a reorg threatens your project's momentum: "I want to make sure [project] doesn't lose context in the transition — can I write a short handoff doc regardless of how ownership shakes out?"
- Protecting your team from scope creep from other orgs: "Happy to collaborate, but I want to be explicit this isn't within our charter — flagging so it doesn't quietly become our responsibility by default."
- When politics genuinely gets ugly (credit-stealing, blame-shifting): stay factual and written. "For the record, here's the timeline of decisions: [dates, docs, PRs]." Don't respond emotionally in the moment — let the paper trail speak.

---

## 8. Driving Postmortems / Incidents at Scale

- Framing root cause without blame, even in front of leadership: "The system allowed this class of failure — here's the structural fix, not just the patch."
- Pushing back on a rushed postmortem: "I don't think we've found the real root cause yet — recommend we take 2 more days before publishing, rather than ship an incomplete story."
- Presenting incident impact to execs: lead with blast radius and resolution, then prevention plan — skip the blow-by-blow unless asked.

---

## 9. Promotion / Visibility Politics at This Level

- Document impact in terms of *leverage*, not just output: "This design decision saved [X team-months] across 3 teams," not "I wrote a service."
- Actively seek out cross-team or cross-org projects — visibility beyond your immediate team is usually a hard requirement past this level.
- When asking your manager for promo support: "Here's the evidence I think supports [level] — technical decisions I drove, people I influenced outside my team, mentorship impact. What gaps do you see?"
- Give credit publicly and often — at this level, how you talk about others' work in public forums (design reviews, all-hands, Slack) is itself being watched as a leadership signal.

---

## 10. Quick Reference — SDE3 Tone Swaps

| SDE2 instinct | SDE3 move |
|---|---|
| "Here's what I'd do — sanity check me?" | "Here's what I'd do, here's why, here's what I rejected and why." |
| Convincing your own team | Convincing teams you have no authority over |
| Writing a design doc for approval | Writing a design doc to build a coalition before the meeting even happens |
| Disagreeing, then quietly resenting the decision | Disagree loudly, then commit fully and say so out loud |
| Reporting your own impact | Reporting impact in terms of leverage across teams/people |
| Avoiding politics | Navigating politics deliberately, in writing, without playing dirty |
| Mentoring 1:1 | Scaling yourself via docs, standards, and sponsoring others' visibility |

---

### The core shift, in one line

**SDE2 earns trust by making good calls under ambiguity. SDE3 earns trust — and influence — by making *other people* confident enough in your judgment and honesty that they follow your direction without being told to.**
