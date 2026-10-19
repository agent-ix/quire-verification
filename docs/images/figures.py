# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""quire-verification's documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. The flow follows
src/planner.rs, src/catalog.rs and src/evidence.rs; the outcome vocabularies are
read from contracts/e02-draft-2.schema.json.
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


def hero():
    """README hero: requirement to technique to capability to plan, then results."""
    f = Figure(
        "hero",
        980,
        330,
        "Quoin's method catalog and its advice for a requirement supply the techniques that "
        "apply. quire-verification combines them with tool capabilities and a selection policy "
        "into a verification plan whose tasks are ready, blocked or unsupported. A native "
        "producer runs the ready tasks, and the technique results come back to be assessed for "
        "scope and freshness. Quoin keeps the evidence and a person decides whether it is enough.",
    )
    y = 40
    f.box(0, y, 160, 56, "code", "requirement", "property shape")
    f.box(210, y, 160, 56, "ink", "Quoin advice", "methods that fit")
    f.box(420, y, 160, 56, "model", "technique", "what a method means")
    f.box(630, y, 160, 56, "model", "capability", "a tool that runs it")
    f.box(820, y, 160, 56, "model", "plan", "tasks · gaps")
    for a, b in ((160, 207), (370, 417), (580, 627), (790, 817)):
        f.line([(a, y + 28), (b, y + 28)], "logic")
    f.text(210, 24, "QUOIN CHOOSES", "muted", "mono", SMALL)
    f.text(630, 24, "QUIRE-VERIFICATION PLANS", "muted", "mono", SMALL)
    f.box(630, 150, 160, 56, "ink", "policy", "authored priority")
    f.line([(790, 178), (805, 178), (805, y + 44), (817, y + 44)], "ink")
    f.box(820, 150, 160, 56, "logic", "native run", "bounded request")
    f.line([(900, y + 56), (900, 147)], "logic")
    f.box(820, 250, 160, 56, "evidence", "result", "scope · freshness")
    f.line([(900, 206), (900, 247)], "evidence")
    f.box(420, 250, 180, 56, "logic", "assess", "no sufficiency call")
    f.line([(820, 278), (603, 278)], "evidence")
    f.box(0, 250, 200, 56, "evidence", "Quoin + a person", "keep evidence · decide")
    f.line([(420, 278), (203, 278)], "evidence")
    f.legend(0, 326, [ROLES[0], ROLES[1], ROLES[2], ROLES[3], ROLES[4]])
    return f


def task_states():
    """A task's applicability and readiness are separate answers."""
    f = Figure(
        "task-states",
        980,
        250,
        "Every plan task carries two independent answers. Applicability is applicable, "
        "not-applicable or unknown; readiness is ready, blocked or unsupported. Only a task "
        "that is applicable, ready and qualified, within budget, gets a native runner request.",
    )
    f.text(0, 14, "APPLICABILITY · DOES THE METHOD FIT", "muted", "mono", SMALL)
    f.text(520, 14, "READINESS · CAN IT RUN NOW", "muted", "mono", SMALL)
    for i, (name, why, role) in enumerate([
        ("applicable", "a rule matched", "logic"),
        ("unknown", "no rule matched", "evidence"),
        ("not-applicable", "explicitly excluded", "evidence"),
    ]):
        y = 28 + i * 56
        f.box(0, y, 200, 48, role, name, why)
    for i, (name, why, role) in enumerate([
        ("ready", "prerequisites met", "logic"),
        ("blocked", "missing generator, harness, budget", "evidence"),
        ("unsupported", "no qualified capability", "evidence"),
    ]):
        y = 28 + i * 56
        f.box(520, y, 300, 48, role, name, why)
    f.box(260, 84, 200, 48, "model", "task", "runner request or null")
    f.line([(200, 52), (230, 52), (230, 108), (257, 108)], "logic")
    f.line([(460, 108), (490, 108), (490, 52), (517, 52)], "logic")
    f.legend(0, 246, [("logic", "allows a run", "box"), ("evidence", "holds it back", "box"), ("model", "plan task", "box")])
    return f


REPOS = [
    ("ORCHESTRATION", [("quire-verification", True), ("quire-driver", True), ("quire-integration", True)]),
    ("BACKENDS", [("quire-contract-codegen", False), ("quire-contract-runtime", False),
                  ("quire-analyze", False), ("quire-protocol", True), ("quire-mltl", False)]),
    ("LANGUAGE", [("quire-specification", True), ("quire-spec-language", False), ("quire-contract-ir", False)]),
    ("FOUNDATIONS", [("quire-exact", False), ("quire-semantic-value", False), ("quire-walk", False),
                     ("quire-canonical", False), ("quire-verification-contracts", False),
                     ("quire-observation", False)]),
]


def ecosystem(here="quire-verification"):
    """The Quire repositories in bands; each band builds on the ones below."""
    f = Figure(
        "ecosystem",
        980,
        364,
        "The Quire repositories in four bands. Foundations: quire-exact, quire-semantic-value, "
        "quire-walk, quire-canonical, quire-verification-contracts, quire-observation. Language: "
        "quire-specification, quire-spec-language, quire-contract-ir. Backends: "
        "quire-contract-codegen, quire-contract-runtime, quire-analyze, quire-protocol, "
        "quire-mltl. Orchestration: quire-verification, quire-driver, quire-integration.",
    )
    y = 0
    gap = 10
    for band, repos in REPOS:
        f.text(0, y + 12, band, "muted", "mono", SMALL)
        widths = [FONTS["title"].width(name, TITLE) + 24 for name, _ in repos]
        spare = (980 - sum(widths) - gap * (len(repos) - 1)) / len(repos)
        x = 0
        for (name, private), w in zip(repos, widths):
            w = round(w + spare)
            role = "logic" if name == here else "ink"
            f.box(x, y + 20, w, 50, role, name, "private" if private else "public", dashed=private)
            x += w + gap
        y += 88
    f.legend(0, 360, [("logic", "this repository", "box"), ("ink", "private", "dash")])
    return f

FIGURES = [hero, task_states, ecosystem]
