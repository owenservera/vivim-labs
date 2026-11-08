/**
 * @hq:context Generated slice of the NL conformance corpus — deterministic template expansion, source-verified.
 * @hq:owner vivim-labs
 * @hq:task ENGINEERING-EVIDENCE-FACTORY/W3-01
 *
 * GENERATED FILE. Do not hand-edit.
 * Regenerate with: bun corpora/nl/generate.mjs
 *
 * Every case here was re-checked against the real TierFlow source at
 * generation time and carries generated: true. Generated cases are NOT
 * independently reviewed labels and must never be counted as verified.
 */

export const GENERATION_META = Object.freeze({
  seed: 20261106,
  target: 240,
  generated: 84,
  deduplicated: 453,
  rejected: 0,
  handLabelled: 91,
  tierflowSha: "ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7",
  byCategory: {
  "synonym-paraphrase": 30,
  "intent-classification": 17,
  "ambiguity": 15,
  "correction": 6,
  "multilingual": 6,
  "refusal": 7,
  "chained-command": 3
},
});

export const GENERATED_CASES = [
  {
    "id": "nl-gen-syn-0001",
    "category": "synonym-paraphrase",
    "utterance": "Use the dusk appearance.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-007: theme noun variant (seed nl-syn-007; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0002",
    "category": "intent-classification",
    "utterance": "Could you ui theme reset?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-007: generic intent phrasing (seed nl-int-007; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0003",
    "category": "ambiguity",
    "utterance": "Can you organise my panels?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-006: vague but safe request (seed nl-amb-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cor-0004",
    "category": "correction",
    "utterance": "Hold on, put it back where it was.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "unverifiable-in-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cor-004: deictic correction, no prior-state channel in the IR (seed nl-cor-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-mul-0005",
    "category": "multilingual",
    "utterance": "Mova o painel de tarefas activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-mul-005: pt phrasing (seed nl-mul-005; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0006",
    "category": "ambiguity",
    "utterance": "Can you tweak the layout?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-006: vague but safe request (seed nl-amb-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0007",
    "category": "ambiguity",
    "utterance": "I want you to tweak the layout.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-006: vague but safe request (seed nl-amb-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-ref-0008",
    "category": "refusal",
    "utterance": "without asking, change theme",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "adversarial-mutation",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-ref-015: unsafe or compound phrasing (seed nl-ref-015; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0009",
    "category": "ambiguity",
    "utterance": "I want you to tidy up my workspace.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-004: vague but safe request (seed nl-amb-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-mul-0010",
    "category": "multilingual",
    "utterance": "Mueve mi panel de tareas activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-mul-001: es phrasing (seed nl-mul-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0011",
    "category": "intent-classification",
    "utterance": "Move the tasks at the top activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-004: move intent, noun variant (seed nl-int-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-ref-0012",
    "category": "refusal",
    "utterance": "send to provider change theme",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "adversarial-mutation",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-ref-009: unsafe or compound phrasing (seed nl-ref-009; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0013",
    "category": "ambiguity",
    "utterance": "Can you tidy up my workspace?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-001: vague but safe request (seed nl-amb-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-mul-0014",
    "category": "multilingual",
    "utterance": "Verplaats het taakpaneel activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-mul-007: nl phrasing (seed nl-mul-007; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0015",
    "category": "ambiguity",
    "utterance": "I want you to make the panels nicer.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-004: vague but safe request (seed nl-amb-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cha-0016",
    "category": "chained-command",
    "utterance": "Move my tasks above activity, then put the task widget at the top.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "independent-review",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cha-001: chained intents separated by \", then\" (seed nl-cha-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0017",
    "category": "ambiguity",
    "utterance": "Can you make the panels nicer?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-006: vague but safe request (seed nl-amb-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-ref-0018",
    "category": "refusal",
    "utterance": "sudo change theme",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "adversarial-mutation",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-ref-016: unsafe or compound phrasing (seed nl-ref-016; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0019",
    "category": "synonym-paraphrase",
    "utterance": "Please move the task panel at the top the activity panel.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-004: politeness + preposition variant (seed nl-syn-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-ref-0020",
    "category": "refusal",
    "utterance": "!use change theme",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "adversarial-mutation",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-ref-006: unsafe or compound phrasing (seed nl-ref-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-ref-0021",
    "category": "refusal",
    "utterance": "powershell -Command change theme",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "adversarial-mutation",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-ref-012: unsafe or compound phrasing (seed nl-ref-012; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-mul-0022",
    "category": "multilingual",
    "utterance": "Sposta il pannello delle attivita activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-mul-001: it phrasing (seed nl-mul-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0023",
    "category": "intent-classification",
    "utterance": "Move the task widget above activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-001: move intent, noun variant (seed nl-int-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cha-0024",
    "category": "chained-command",
    "utterance": "Move my tasks above activity; put the task widget at the top.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cha-003: chained intents separated by \";\" (seed nl-cha-003; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0025",
    "category": "synonym-paraphrase",
    "utterance": "Please do the ui layout preview action now.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-012: generic phrasing (seed nl-syn-012; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0026",
    "category": "synonym-paraphrase",
    "utterance": "Move my task widget at the top activity.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-002: panel noun variant (seed nl-syn-002; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0027",
    "category": "synonym-paraphrase",
    "utterance": "Move my task panel at the top activity.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-004: panel noun variant (seed nl-syn-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-ref-0028",
    "category": "refusal",
    "utterance": "!run change theme",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "adversarial-mutation",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-ref-003: unsafe or compound phrasing (seed nl-ref-003; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-mul-0029",
    "category": "multilingual",
    "utterance": "Deplace le panneau de taches activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-mul-004: fr phrasing (seed nl-mul-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0030",
    "category": "synonym-paraphrase",
    "utterance": "Please move the task panel above the activity panel.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-006: politeness + preposition variant (seed nl-syn-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-mul-0031",
    "category": "multilingual",
    "utterance": "Setze mein Task-Panel activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-mul-007: de phrasing (seed nl-mul-007; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0032",
    "category": "synonym-paraphrase",
    "utterance": "Move my task widget above activity.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-005: panel noun variant (seed nl-syn-005; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0033",
    "category": "intent-classification",
    "utterance": "Move the task panel at the top activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-004: move intent, noun variant (seed nl-int-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0034",
    "category": "intent-classification",
    "utterance": "Could you workspace panel show?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-003: generic intent phrasing (seed nl-int-003; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cor-0035",
    "category": "correction",
    "utterance": "No wait, move it back.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "unverifiable-in-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cor-002: deictic correction, no prior-state channel in the IR (seed nl-cor-002; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0036",
    "category": "synonym-paraphrase",
    "utterance": "Use the ember appearance.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-010: theme noun variant (seed nl-syn-010; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0037",
    "category": "intent-classification",
    "utterance": "Show me my appearance.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-006: show intent, must not collapse into set (seed nl-int-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0038",
    "category": "ambiguity",
    "utterance": "I want you to clean up the layout.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-001: vague but safe request (seed nl-amb-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0039",
    "category": "ambiguity",
    "utterance": "I want you to rearrange my dashboard.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-001: vague but safe request (seed nl-amb-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0040",
    "category": "synonym-paraphrase",
    "utterance": "Use the dusk theme.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-008: theme noun variant (seed nl-syn-008; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cor-0041",
    "category": "correction",
    "utterance": "Actually, put it back where it was.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "unverifiable-in-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cor-003: deictic correction, no prior-state channel in the IR (seed nl-cor-003; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0042",
    "category": "synonym-paraphrase",
    "utterance": "My appearance should be ember.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-010: first person phrasing (seed nl-syn-010; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cor-0043",
    "category": "correction",
    "utterance": "Hold on, move it back.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "unverifiable-in-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cor-004: deictic correction, no prior-state channel in the IR (seed nl-cor-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0044",
    "category": "ambiguity",
    "utterance": "Can you clean up the layout?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-007: vague but safe request (seed nl-amb-007; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0045",
    "category": "intent-classification",
    "utterance": "Reorder the tasks so it comes first",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-001: reorder intent, must not collapse into move (seed nl-int-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0046",
    "category": "ambiguity",
    "utterance": "Can you improve the dashboard?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-001: vague but safe request (seed nl-amb-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-ref-0047",
    "category": "refusal",
    "utterance": "instead of that, change theme",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "adversarial-mutation",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-ref-001: unsafe or compound phrasing (seed nl-ref-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cor-0048",
    "category": "correction",
    "utterance": "Sorry, put it back where it was.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "unverifiable-in-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cor-001: deictic correction, no prior-state channel in the IR (seed nl-cor-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0049",
    "category": "ambiguity",
    "utterance": "I want you to improve the dashboard.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-003: vague but safe request (seed nl-amb-003; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0050",
    "category": "ambiguity",
    "utterance": "I want you to sort my dashboard.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-001: vague but safe request (seed nl-amb-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0051",
    "category": "ambiguity",
    "utterance": "Can you sort my dashboard?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-007: vague but safe request (seed nl-amb-007; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0052",
    "category": "synonym-paraphrase",
    "utterance": "Please move the tasks at the top the activity panel.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-002: politeness + preposition variant (seed nl-syn-002; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0053",
    "category": "synonym-paraphrase",
    "utterance": "Please move the tasks above the activity panel.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-003: politeness + preposition variant (seed nl-syn-003; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0054",
    "category": "intent-classification",
    "utterance": "Set my appearance to dusk.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-006: set intent, noun variant (seed nl-int-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0055",
    "category": "synonym-paraphrase",
    "utterance": "My appearance should be lab.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-009: first person phrasing (seed nl-syn-009; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0056",
    "category": "synonym-paraphrase",
    "utterance": "My appearance should be dusk.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-008: first person phrasing (seed nl-syn-008; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cor-0057",
    "category": "correction",
    "utterance": "Sorry, move it back.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "unverifiable-in-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cor-002: deictic correction, no prior-state channel in the IR (seed nl-cor-002; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0058",
    "category": "synonym-paraphrase",
    "utterance": "Please move the task widget above the activity panel.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-006: politeness + preposition variant (seed nl-syn-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0059",
    "category": "synonym-paraphrase",
    "utterance": "Please move the task widget at the top the activity panel.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-004: politeness + preposition variant (seed nl-syn-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-amb-0060",
    "category": "ambiguity",
    "utterance": "I want you to organise my panels.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-amb-003: vague but safe request (seed nl-amb-003; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0061",
    "category": "synonym-paraphrase",
    "utterance": "Move my tasks at the top activity.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-002: panel noun variant (seed nl-syn-002; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0062",
    "category": "synonym-paraphrase",
    "utterance": "My colour scheme should be dusk.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-008: first person phrasing (seed nl-syn-008; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0063",
    "category": "synonym-paraphrase",
    "utterance": "Move my task panel above activity.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-006: panel noun variant (seed nl-syn-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0064",
    "category": "intent-classification",
    "utterance": "Reorder the task widget so it comes first",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-004: reorder intent, must not collapse into move (seed nl-int-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0065",
    "category": "synonym-paraphrase",
    "utterance": "Use the ember colour scheme.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-010: theme noun variant (seed nl-syn-010; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0066",
    "category": "intent-classification",
    "utterance": "Reorder the task panel so it comes first",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-004: reorder intent, must not collapse into move (seed nl-int-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-cha-0067",
    "category": "chained-command",
    "utterance": "Move my tasks above activity after that put the task widget at the top.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "refusal",
      "reasonCode": "UNSAFE_OR_COMPOUND_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-cha-005: chained intents separated by \"after that\" (seed nl-cha-005; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0068",
    "category": "intent-classification",
    "utterance": "Move the task widget at the top activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-004: move intent, noun variant (seed nl-int-004; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0069",
    "category": "intent-classification",
    "utterance": "Show me my colour scheme.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-006: show intent, must not collapse into set (seed nl-int-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0070",
    "category": "synonym-paraphrase",
    "utterance": "My colour scheme should be lab.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-009: first person phrasing (seed nl-syn-009; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0071",
    "category": "synonym-paraphrase",
    "utterance": "Use the lab appearance.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-009: theme noun variant (seed nl-syn-009; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0072",
    "category": "synonym-paraphrase",
    "utterance": "Please do the ui theme reset action now.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-011: generic phrasing (seed nl-syn-011; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0073",
    "category": "intent-classification",
    "utterance": "Could you ui theme show?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-005: generic intent phrasing (seed nl-int-005; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0074",
    "category": "synonym-paraphrase",
    "utterance": "My theme should be ember.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-010: first person phrasing (seed nl-syn-010; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0075",
    "category": "synonym-paraphrase",
    "utterance": "My colour scheme should be ember.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-010: first person phrasing (seed nl-syn-010; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0076",
    "category": "intent-classification",
    "utterance": "Set my colour scheme to dusk.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-006: set intent, noun variant (seed nl-int-006; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0077",
    "category": "synonym-paraphrase",
    "utterance": "My theme should be dusk.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-007: first person phrasing (seed nl-syn-007; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0078",
    "category": "synonym-paraphrase",
    "utterance": "Use the lab colour scheme.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-009: theme noun variant (seed nl-syn-009; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0079",
    "category": "synonym-paraphrase",
    "utterance": "Use the ember theme.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-010: theme noun variant (seed nl-syn-010; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0080",
    "category": "synonym-paraphrase",
    "utterance": "Use the dusk colour scheme.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-008: theme noun variant (seed nl-syn-008; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0081",
    "category": "intent-classification",
    "utterance": "Move the tasks above activity",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-001: move intent, noun variant (seed nl-int-001; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0082",
    "category": "intent-classification",
    "utterance": "Could you workspace panel reorder?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-002: generic intent phrasing (seed nl-int-002; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-int-0083",
    "category": "intent-classification",
    "utterance": "Could you ui layout preview?",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-int-008: generic intent phrasing (seed nl-int-008; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  },
  {
    "id": "nl-gen-syn-0084",
    "category": "synonym-paraphrase",
    "utterance": "My theme should be lab.",
    "surface": "text",
    "labelSource": "generated-from-template:corpora/nl/generate.mjs",
    "labelMethod": "derived-from-source",
    "expected": {
      "outcome": "clarification",
      "reasonCode": "UNKNOWN_TEXT",
      "command": null
    },
    "requirement": null,
    "contextOverrides": null,
    "generated": true,
    "notes": "templated from nl-syn-009: first person phrasing (seed nl-syn-009; verified against TierFlow ec832f8be079ba4d3fb1de0f9594ba9606b5f7c7)."
  }
];

export default GENERATED_CASES;
