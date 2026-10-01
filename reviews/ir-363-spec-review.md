---
id: "SR-760"
title: "IR-363 spec review: FR-014 and FR-017-AC-17 grammar-warning repair (CG)"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@6ef817b5499092bb031e36871bd5ae909f35059c; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/kani/functional/FR-017-kani-execution-evidence.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

# SR-760: IR-363 spec review

## Summary

Ticket: IR-363 (CG half). PR: agent-ix/quire-contract-codegen#224, head 6ef817b, base
origin/main 224ca6e (the merge base is the current main). Method: spec-review. The
integrity, EARS and id checks are folded into this file.

The PR changes two spec files and four lines. It changes no source, test or matrix.

## Method

- I ran `make spec` on origin/main 224ca6e. It exited 0 with no errors and three grammar
  warnings: FR-017 line 159 `[ac:vague-response]` on `process`, and FR-014 line 278
  `[ears:unclassifiable]` and `[ears:missing-subject]`. So the AP-001 and MP-001 frontmatter
  errors the ticket lists are already gone on main, as the PR body says.
- I ran `make spec` on head 6ef817b. It exited 0 with no errors and no warnings.
- FR-014: I compared the file at main and at head with whitespace collapsed. The text is
  identical, so the change is a rewrap only.
- FR-017: I compared the AC, CON and other id lists at main and at head. They are identical,
  so no requirement was removed or renumbered.
- FR-017-AC-17: I read the FR-017 statement (lines 86-88), `src/kani_execution.rs`
  (`run_launcher_with_timeout`, which spawns with `.process_group(0)`, and
  `kill_process_tree`, which calls `kill_process_group` on the child's pid), the test
  `a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child` (Trace:
  FR-017-AC-17, TC-027), the TC-027 doc and `spec/kani/matrix/tests.md`. The matrix and the
  trace cite AC-17 by id only, so the rewording does not affect them.
- I tried candidate wordings for AC-17 against the checker in scratch copies, not in the
  repo. Any wording containing the word `process` still warns, including "every process in
  the launcher's group" and "process-group". The checker reads the noun `process` as a vague
  verb, which is a false positive in quire's grammar check, not a defect in this spec.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | In the new FR-017-AC-17 wording, "it" in "the whole group it leads" grammatically refers to "A run". A run leads no group; the launcher does. The FR statement at lines 86-88 makes the intent clear (the launcher leads its own process group, and that group is killed), and the test checks exactly that. So the AC stays testable, but it reads less precisely than before. Suggested wording, which passes the checker with no warning: "A run that times out has the whole group the launcher leads killed, a real grandchild included." | spec/kani/functional/FR-017-kani-execution-evidence.md:159 |

### FND-001 detail

Before: "A run that times out has its whole process group killed, a real grandchild
included." After: "A run that times out has the whole group it leads killed, a real
grandchild included." Both versions have the same loose antecedent ("its" or "it" referring to
the run). The old wording also named a *process* group. The behaviour under test is that the
launcher runs as leader of its own process group and the whole group is killed on timeout.
The grandchild assertion in the test is what tells a group kill apart from a child-only kill.
Naming the launcher fixes the antecedent. I measured that "the whole group the launcher leads"
and "the launcher's whole group" both clear the checker.

## Verdict

The change does what it says. Head passes `make spec` with exit 0 and 0 warnings, against 3
warnings on main. No id was removed or changed. FR-014's meaning is unchanged because the edit
is a pure rewrap. FR-017-AC-17 still specifies the tested behaviour once it is read with the
FR statement. The one finding is a low wording nit. Mergeable as is; applying the suggested
AC-17 wording would be a small improvement.

Not a finding against this PR: quire's `ac:vague-response` check flags the noun `process` as a
verb. That is a checker false positive worth raising with quire.
