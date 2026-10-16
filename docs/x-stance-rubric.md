# X stance rubric

**Version: `stance-1`.** Routine 1 of [grok-bot.md](grok-bot.md) fetches this page at every run,
labels each post it reads with it, and records the version above as `rubric_version` in the drop.
A change to a definition, a rule or an example gets a new version (`stance-2`, ...), so that drops
labelled under different rules can be told apart.

Each post is labelled for one candidate: the candidate whose search returned it. A post found in
two candidates' searches is labelled once for each, independently.

## Labels

Two questions, in order.

**1. Relevance: is this post about this candidate?** The candidate is the person in
`config/candidates.yaml`, not a namesake, a relative, a place or a common word.

- `off_topic`: the name refers to someone or something else, or the post only lists names
  (spam, hashtag stuffing, a trending list with no content). Stop here.
- Otherwise the post is relevant: go to question 2.

**2. Stance: what is the author's own attitude toward this candidate?**

- `support`: the author favours the candidate: praise, defence, a call to vote for or join them,
  approval of their proposal, pleasure at their success.
- `oppose`: the author is against the candidate: criticism, mockery, insult, a call not to vote for
  them, approval of an attack on them, pleasure at their failure.
- `neutral`: about the candidate, with no stance the author takes: news, poll results, agenda,
  quotes reported without comment, questions, analysis that takes no side, or a stance that cannot
  be told.

Stance is the author's attitude toward the candidate, not the tone of the post and not whether the
news is good or bad for them. An angry post can be `support`; a cheerful one can be `oppose`.

## Decision rules

1. **Namesakes.** "Le Pen" alone means Marine Le Pen only when the context says so (her campaign,
   the RN today, her trial). Jean-Marie Le Pen, Marion Maréchal or "les Le Pen" as a family are
   `off_topic` for `le_pen` unless Marine Le Pen is clearly part of what is said. "Hollande" the
   country, or another person named Roussel or Guedj: `off_topic`. Short surnames need the most
   care. A first
   name and an office (député, maire, président du parti) settle most cases; when nothing does,
   `off_topic`.
2. **Reporting is not endorsing.** A headline, a poll figure, a quote or a schedule passed on
   without the author's own comment is `neutral`, from a media account or anyone else. A comment
   added to it decides the label.
3. **Quote-posts and replies.** Label the author's own words. They take the quoted post's stance
   only when they endorse it ("Exactement.", "Tout est dit") and the opposite stance when they
   reject it. A quote with no added text is `neutral`.
4. **Sarcasm and irony.** Label what the author means, not the literal words. If you cannot tell
   whether a post is ironic, it is `neutral`.
5. **Supporters attacking the media or opponents.** A post defending the candidate against a
   journalist, an outlet, a poll or a rival is `support`, however hostile its tone. A post attacking
   another candidate is `support` for this one only if it says this one is better; otherwise it is
   `neutral` for this candidate.
6. **Mixed posts.** Weigh the post as a whole. If praise and criticism balance, or neither clearly
   wins, it is `neutral`.
7. **Hashtags and slogans.** A campaign hashtag or slogan used by the author (#AvecX, "X
   président") is `support`; a hostile one (#ToutSaufX) is `oppose`. A hashtag alone, with no other
   content about the candidate, follows rule 1's spam case: `off_topic`.
8. **Predictions and analysis.** "X will win", "X is rising in the polls" is `neutral` unless the
   author shows they want it or fear it.
9. **What you cannot read.** Label from the text you can read. A post whose only content is an
   image or a link you cannot open, beside the candidate's name, is `neutral`.
10. **One label per post.** Never skip a drawn post: every one gets exactly one of the four labels,
    so that `support + oppose + neutral + off_topic = posts_drawn`.

## Examples

These examples are **constructed for this rubric**. They are not real posts and no real person
wrote them.

| #   | Candidate    | Post (constructed)                                                                     | Label       | Why                                         |
| --- | ------------ | -------------------------------------------------------------------------------------- | ----------- | ------------------------------------------- |
| 1   | `melenchon`  | « Meeting incroyable ce soir, Mélenchon a parlé pour nous tous. »                      | `support`   | Praise                                      |
| 2   | `melenchon`  | « Mélenchon en meeting à Clermont-Ferrand jeudi à 19 h. »                              | `neutral`   | Agenda, no comment (rule 2)                 |
| 3   | `bardella`   | « Encore un plateau où on coupe Bardella toutes les dix secondes. Honteux. »           | `support`   | Defends him against the media (rule 5)      |
| 4   | `bardella`   | « Bardella n'a rien à dire sur l'économie, comme d'habitude. »                         | `oppose`    | Criticism                                   |
| 5   | `le_pen`     | « Jean-Marie Le Pen aurait eu cent ans cette année. »                                  | `off_topic` | Another Le Pen (rule 1)                     |
| 6   | `le_pen`     | « Le Pen à 33 % dans le dernier sondage Ifop. »                                        | `neutral`   | A poll result passed on (rule 2)            |
| 7   | `le_pen`     | « Le Pen à 33 % ? Enfin ! Le peuple se réveille. »                                     | `support`   | The author welcomes it                      |
| 8   | `hollande`   | « Match nul entre la France et la Hollande hier soir. »                                | `off_topic` | The country (rule 1)                        |
| 9   | `hollande`   | « Bravo Hollande, encore une idée brillante. On a hâte de la voir échouer. »           | `oppose`    | Sarcasm (rule 4)                            |
| 10  | `glucksmann` | Quote-post of a criticism of Glucksmann, adding « Tout est dit. »                      | `oppose`    | Endorses the quoted criticism (rule 3)      |
| 11  | `glucksmann` | Quote-post of the same criticism, adding « Article malhonnête, il n'a jamais dit ça. » | `support`   | Rejects the quoted criticism (rule 3)       |
| 12  | `glucksmann` | Quote-post of a news article about Glucksmann, with no added text.                     | `neutral`   | Quote without comment (rule 3)              |
| 13  | `attal`      | « Bonne mesure sur l'école, mais Attal reste flou sur le financement. »                | `neutral`   | Praise and criticism balance (rule 6)       |
| 14  | `attal`      | « Quelques bonnes idées, mais Attal n'est clairement pas prêt pour l'Élysée. »         | `oppose`    | Criticism wins (rule 6)                     |
| 15  | `retailleau` | « Retailleau ou Philippe, qui sera le candidat de la droite ? »                        | `neutral`   | A question                                  |
| 16  | `retailleau` | « Édouard Philippe déçoit encore. Retailleau, lui, tient ses positions. »              | `support`   | Says he is better than a rival (rule 5)     |
| 17  | `philippe`   | « Édouard Philippe déçoit encore. Retailleau, lui, tient ses positions. »              | `oppose`    | The same post, labelled for the other name  |
| 18  | `zemmour`    | « #ToutSaufZemmour »                                                                   | `oppose`    | Hostile hashtag used by the author (rule 7) |
| 19  | `roussel`    | « Les frères Roussel ouvrent leur nouvelle boulangerie à Lille. »                      | `off_topic` | Another Roussel (rule 1)                    |
| 20  | `ruffin`     | « Ruffin va finir par se présenter, c'est évident. »                                   | `neutral`   | A prediction with no stance (rule 8)        |
| 21  | `tondelier`  | « Tondelier Bardella Mélenchon Le Pen Attal #presidentielle2027 #politique »           | `off_topic` | A list of names, no content (rule 1)        |
| 22  | `tondelier`  | « Je vote Tondelier en avril, et vous ? »                                              | `support`   | Intention to vote for her                   |
