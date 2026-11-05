//! Section plans for the five security document types.

/// One section of a document: a title and a one-line instruction for the model.
#[derive(Debug, Clone, Copy)]
pub struct Section {
    pub title: &'static str,
    pub instruction: &'static str,
}

/// A document type with its fixed section plan.
#[derive(Debug, Clone, Copy)]
pub struct DocType {
    pub key: &'static str,
    pub title: &'static str,
    pub sections: &'static [Section],
}

const fn s(title: &'static str, instruction: &'static str) -> Section {
    Section { title, instruction }
}

pub const DOC_TYPES: &[DocType] = &[
    DocType {
        key: "threat-model",
        title: "Threat Model",
        sections: &[
            s("Scope", "State what system, components and data flows this threat model covers and what it excludes."),
            s("Assets", "List the assets worth protecting, such as data, credentials and services, and why each matters."),
            s("Trust Boundaries", "Describe the trust boundaries, the actors on each side and where data crosses them."),
            s("Threats (STRIDE)", "List threats grouped by Spoofing, Tampering, Repudiation, Information disclosure, Denial of service and Elevation of privilege."),
            s("Mitigations", "Give a concrete mitigation for each threat and name the control type."),
            s("Residual Risk", "Explain which risks remain after mitigation, their likelihood and impact, and who should accept them."),
        ],
    },
    DocType {
        key: "secure-coding-guideline",
        title: "Secure Coding Guideline",
        sections: &[
            s("Purpose and Audience", "State why this guideline exists, which code it covers and who must follow it."),
            s("Input Validation and Output Encoding", "Explain how to validate untrusted input and encode output to prevent injection and XSS."),
            s("Authentication and Session Management", "Describe secure rules for passwords, tokens, sessions and multi-factor authentication."),
            s("Access Control", "Describe how to enforce authorization, least privilege and deny-by-default checks on the server side."),
            s("Secrets and Cryptography", "Give rules for storing secrets and using vetted cryptographic libraries and algorithms."),
            s("Error Handling and Logging", "Explain how to handle errors safely and log security events without leaking sensitive data."),
            s("Dependencies and Review", "Describe how to manage third-party dependencies, run security checks and perform code review."),
        ],
    },
    DocType {
        key: "incident-response-runbook",
        title: "Incident Response Runbook",
        sections: &[
            s("Purpose and Scope", "State which incident types this runbook covers and when to use it."),
            s("Roles and Contacts", "Define the incident roles, their responsibilities and how to reach them."),
            s("Detection and Triage", "Explain how to confirm an incident, assess severity and decide on escalation."),
            s("Containment", "List short-term and long-term containment steps that limit the damage."),
            s("Eradication and Recovery", "Describe how to remove the threat, restore systems and verify they are clean."),
            s("Communication", "Explain who to notify internally and externally, and when."),
            s("Post-Incident Review", "Describe how to run a lessons-learned review and track follow-up actions."),
        ],
    },
    DocType {
        key: "hardening-checklist",
        title: "Hardening Checklist",
        sections: &[
            s("Scope and Baseline", "State which systems this checklist covers and which baseline or standard it follows."),
            s("Accounts and Access", "List checks for accounts, privileges, passwords and remote access."),
            s("Network and Firewall", "List checks for open ports, network services, segmentation and firewall rules."),
            s("Services and Software", "List checks for removing unneeded services and keeping software patched."),
            s("Logging and Monitoring", "List checks for audit logging, log retention and alerting."),
            s("Data Protection", "List checks for encryption at rest and in transit, and for backups."),
        ],
    },
    DocType {
        key: "vulnerability-remediation-report",
        title: "Vulnerability Remediation Report",
        sections: &[
            s("Summary", "Summarize the vulnerability, the affected systems and the overall risk in a few sentences."),
            s("Affected Systems", "List the affected components and versions, and how the issue was found."),
            s("Impact and Severity", "Explain the impact, exploitability and severity rating, without inventing CVE IDs."),
            s("Root Cause", "Explain the underlying cause of the vulnerability."),
            s("Remediation Steps", "Give ordered remediation steps with the owner of each step."),
            s("Verification", "Describe how to confirm that the fix works and that no regression exists."),
            s("Timeline and Follow-up", "Give the discovery, fix and verification timeline and any follow-up actions."),
        ],
    },
];

/// Find a document type by its exact key.
pub fn find(key: &str) -> Option<&'static DocType> {
    DOC_TYPES.iter().find(|d| d.key == key)
}

/// Make a file-name slug: lowercase ASCII letters and digits, other runs become one '-'.
pub fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut pending_dash = false;
    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(c.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    if out.is_empty() {
        "untitled".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_types_in_order() {
        let keys: Vec<_> = DOC_TYPES.iter().map(|d| d.key).collect();
        assert_eq!(
            keys,
            [
                "threat-model",
                "secure-coding-guideline",
                "incident-response-runbook",
                "hardening-checklist",
                "vulnerability-remediation-report"
            ]
        );
    }

    #[test]
    fn keys_are_unique() {
        for (i, a) in DOC_TYPES.iter().enumerate() {
            for b in &DOC_TYPES[i + 1..] {
                assert_ne!(a.key, b.key);
            }
        }
    }

    #[test]
    fn plans_are_complete() {
        for d in DOC_TYPES {
            assert!(!d.title.is_empty());
            assert!(!d.sections.is_empty(), "{} has no sections", d.key);
            for sec in d.sections {
                assert!(!sec.title.is_empty());
                assert!(!sec.instruction.is_empty());
            }
        }
    }

    #[test]
    fn threat_model_sections() {
        let titles: Vec<_> = find("threat-model")
            .unwrap()
            .sections
            .iter()
            .map(|s| s.title)
            .collect();
        assert_eq!(
            titles,
            [
                "Scope",
                "Assets",
                "Trust Boundaries",
                "Threats (STRIDE)",
                "Mitigations",
                "Residual Risk"
            ]
        );
    }

    #[test]
    fn find_hit_and_miss() {
        assert_eq!(find("hardening-checklist").unwrap().key, "hardening-checklist");
        assert!(find("nope").is_none());
        assert!(find("Threat-Model").is_none());
        assert!(find("").is_none());
    }

    #[test]
    fn slugify_cases() {
        assert_eq!(slugify("payment API"), "payment-api");
        assert_eq!(slugify("  A/B  test!! "), "a-b-test");
        assert_eq!(slugify(""), "untitled");
        assert_eq!(slugify("!!!"), "untitled");
        assert_eq!(slugify("café Ünï 2"), "caf-n-2");
        assert_eq!(slugify("日本語"), "untitled");
    }
}
