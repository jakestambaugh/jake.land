---
title: "Technical Debt"
date: 2020-10-04T11:54:22-04:00
draft: true
---

Technical Debt has been the recurring theme of my past few weeks. At work, our team did an exercise to categorize technical debt in the bug backlog. Another group in the comapny has started publishing articles and videos internally about code health. Even in my free time I've been seeing articles in my news feed. With all of this coming at me, I've formed/found some :fire:_hot takes_:fire: on tech debt.

One of the best points that I've seen reiterated is that technical debt isn't really "debt" at all. When you go into financial debt, you know exactly how much money you owe and to whom. You can budget to pay it down month after month, and you can extrapolate how fast the interest will accumulate. Technical debt is more like pollution. Taking inventory of the "tech debt" in a system can take a lot of effort, and the goal will never really be to "pay it down". The objective when dealing with tech debt is to identify, contain and reduce it.

I learned this week that the metaphor of "debt" in software originally meant something slightly different. When [Ward Cunningham started using it](http://wiki.c2.com/?WardExplainsDebtMetaphor) the metaphor of "debt" was a way of explaining how a developer could intentionally write impermanent code in order to unblock themselves from implementing a feature of the system. In that case, the developer could build an run a system, learn through experience where it was mis-aligned with the problem domain, and then refactor and re-align it once they had this greater understanding of how to model the system.

Unfortunately, the definition that I learned (and I think the one discussed in industry most) is more like "bad code that we don't have time to fix, but maybe later". 

Another effect of treating tech debt like pollution is that the developers who own the code are also the only ones who really know where the pollution it. This also leads an interesting tension. If an organization is trying to enforce a top-down policy to "reduce technical debt by xx%", they might incentivize their teams to under-report difficult tech debt issues, opting instead to clean up some linter warnings to get the bug percentage up.

I'm finding that the structural issues are much more of a personal taste issue than the linter warnings. When a teammate says "I think our API has a lot of technical debt", what they are saying is "This API doesn't match my understanding of the problem domain". Sometimes, the whole team is feeling the same thing, in which case the API is labelled technical debt and the refactors can be planned. Alternatively, the API matches *some* developers expectations, but not all. Now its a social issue; what can we do about this?