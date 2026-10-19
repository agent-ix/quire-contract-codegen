# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""quire-contract-codegen's documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. The Kani outcomes are the
variants of KaniRunOutcome in src/kani/classify.rs and the replay verdicts are the
variants of ReplayVerdict in src/replay/function.rs.
"""

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

# One role per kind of thing, fixed across the Quire repositories.
ROLES = [
    ("code", "authored source", "box"),
    ("logic", "compiler stage", "box"),
    ("model", "checked package / IR", "box"),
    ("evidence", "outcome", "box"),
    ("ink", "input", "box"),
]


# ---- figures -----------------------------------------------------------------


def hero():
    """README hero: one checked package fans out into four kinds of artifact."""
    f = Figure(
        "hero",
        980,
        350,
        "One checked contract package goes into quire-contract-codegen and fans out into four "
        "generated outputs: Rust oracles, property tests, Kani proofs and evidence records. A "
        "counterexample from a Kani run is replayed natively through QSL, and the result is "
        "either a reproduced violation or an evidence failure.",
    )
    top, step = 0, 84
    mid = top + 1.5 * step + 28
    f.box(0, mid - 28, 180, 56, "model", "checked package", "from quire-contract-ir")
    f.box(220, mid - 28, 190, 56, "logic", "quire-contract-codegen", "deterministic")
    f.line([(180, mid), (217, mid)], "model")
    f.line([(410, mid), (425, mid)], "logic", arrow=False)
    f.line([(425, top + 28), (425, top + 3 * step + 28)], "logic", arrow=False)
    outs = [
        ("model", "Rust oracles", "the contract as plain code"),
        ("model", "property tests", "proptest strategies"),
        ("model", "Kani proofs", "bounded obligations"),
        ("evidence", "evidence", "claim map · proof graph"),
    ]
    for i, (role, title, sub) in enumerate(outs):
        y = top + i * step
        f.line([(425, y + 28), (437, y + 28)], "logic")
        f.box(440, y, 210, 56, role, title, sub)
    y = top + 2 * step
    f.line([(650, y + 28), (682, y + 28)], "model")
    f.box(685, y, 130, 56, "ink", "Kani run", "finds a witness")
    f.line([(815, y + 28), (842, y + 28)], "ink")
    f.box(845, y, 135, 56, "logic", "qsl-replay", "same witness")
    f.line([(912, y + 56), (912, y + step - 3)], "logic")
    f.box(800, y + step, 180, 56, "evidence", "reproduced", "or evidence failure")
    f.legend(0, 344, [ROLES[2], ROLES[1], ROLES[3], ROLES[4]])
    return f


def outcomes():
    """How one Kani run can end, and what happens to a counterexample."""
    rows = [
        ("verified", "every property held", "logic"),
        ("falsified", "a property failed; witness returned", "evidence"),
        ("cover unsatisfied", "vacuous: a cover was never reached", "evidence"),
        ("cancelled", "the caller stopped the run", "evidence"),
        ("inconclusive", "the run established nothing", "evidence"),
    ]
    f = Figure(
        "outcomes",
        980,
        350,
        "A Kani run ends as verified, falsified, cover unsatisfied, cancelled or inconclusive. "
        "Only a falsified run returns a counterexample, which codegen replays through QSL: the "
        "result is a reproduced violation, or an evidence failure when the counterexample is "
        "not valid evidence.",
    )
    step = 60
    f.box(0, 124, 150, 56, "ink", "a Kani run", "one harness")
    f.line([(150, 152), (190, 152)], "ink", arrow=False)
    f.line([(190, 25), (190, 25 + 4 * step)], "ink", arrow=False)
    for i, (name, why, role) in enumerate(rows):
        y = i * step
        f.line([(190, y + 25), (217, y + 25)], role)
        f.box(220, y, 330, 50, role, name, why)
    f.line([(550, step + 25), (597, step + 25)], "evidence", weight=2.4)
    f.box(600, step, 150, 50, "logic", "qsl-replay", "native, in QSL")
    f.line([(750, step + 25), (765, step + 25)], "logic", arrow=False)
    f.line([(765, step - 5), (765, step + 55)], "logic", arrow=False)
    f.line([(765, step - 5), (777, step - 5)], "logic")
    f.line([(765, step + 55), (777, step + 55)], "logic")
    f.box(780, step - 30, 200, 50, "evidence", "reproduced", "the violation is real")
    f.box(780, step + 30, 200, 50, "evidence", "evidence failure", "not valid evidence")
    f.legend(0, 344, [ROLES[1], ROLES[3], ROLES[4]])
    return f


REPOS = [
    (1, 0, "quire-driver", "the quire command", True),
    (2, 0, "quire-integration", "tests the whole chain", True),
    (3, 0, "quire-verification", "plans verification", True),
    (0, 1, "quire-specification", "the standard", True),
    (1, 1, "quire-spec-language", "compiler · evaluator", False),
    (2, 1, "quire-contract-ir", "contract model", False),
    (3, 1, "quire-contract-codegen", "code · tests · proofs", False),
    (4, 1, "quire-contract-runtime", "runs generated checks", False),
    (0, 2, "quire-observation", "qualified observations", False),
    (1, 2, "quire-protocol", "protocol conformance", True),
    (2, 2, "quire-analyze", "SMT consistency", False),
    (0, 3, "quire-mltl", "MLTL over observations", False),
]
FEEDS = [
    ("quire-specification", "quire-spec-language"),
    ("quire-spec-language", "quire-contract-ir"),
    ("quire-contract-ir", "quire-contract-codegen"),
    ("quire-contract-codegen", "quire-contract-runtime"),
    ("quire-spec-language", "quire-protocol"),
    ("quire-contract-ir", "quire-analyze"),
    ("quire-observation", "quire-protocol"),
    ("quire-observation", "quire-mltl"),
]
SHARED = ["quire-exact", "quire-semantic-value", "quire-walk", "quire-canonical",
          "quire-verification-contracts"]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories: the main chain, its branches, and what runs and shares it."""
    f = Figure(
        "ecosystem",
        980,
        476,
        "The Quire repositories. The main chain runs from the standard, quire-specification, to "
        "the compiler, quire-spec-language, then quire-contract-ir, quire-contract-codegen and "
        "quire-contract-runtime. quire-spec-language also feeds quire-protocol, and "
        "quire-contract-ir feeds quire-analyze. quire-observation feeds quire-protocol and "
        "quire-mltl. Above the chain, quire-driver runs it, quire-integration tests it and "
        "quire-verification plans verification. Below it, shared libraries used throughout: "
        "quire-exact, quire-semantic-value, quire-walk, quire-canonical and "
        "quire-verification-contracts.",
    )
    w, h, step = 176, 50, 201
    rows = {0: 20, 1: 120, 2: 210, 3: 290}
    where = {}
    for col, row, name, sub, private in REPOS:
        x, y = col * step, rows[row]
        where[name] = (x, y)
        role = "logic" if name == here else "ink"
        f.box(x, y, w, h, role, name, sub, dashed=private)
    for a, b in FEEDS:
        (ax, ay), (bx, by) = where[a], where[b]
        if ay == by:
            f.line([(ax + w, ay + h / 2), (bx - 3, by + h / 2)], "model")
        else:
            f.line([(ax + w / 2, ay + h), (bx + w / 2, by - 3)], "model")
    f.text(0, 12, "ORCHESTRATION", "muted", "mono", SMALL)
    f.text(0, 112, "THE CHAIN", "muted", "mono", SMALL)
    f.text(0, 382, "SHARED LIBRARIES · USED THROUGHOUT", "muted", "mono", SMALL)
    gap = 10
    widths = [FONTS["title"].width(name, TITLE) + 24 for name in SHARED]
    spare = (980 - sum(widths) - gap * (len(SHARED) - 1)) / len(SHARED)
    x = 0
    for name, w0 in zip(SHARED, widths):
        w0 = round(w0 + spare)
        f.box(x, 390, w0, h, "logic" if name == here else "ink", name)
        x += w0 + gap
    f.legend(0, 472, [("logic", "this repository", "box"), ("ink", "private", "dash"),
                      ("model", "feeds", "fill")])
    return f


def codegen_ecosystem():
    return ecosystem(here="quire-contract-codegen")


FIGURES = [hero, outcomes, codegen_ecosystem]
