//! Security audit: run openclaw security audit --json, parse report, compute score 1–100.

use crate::run;

/// Matches openclaw security audit --json output (report part).
#[derive(serde::Deserialize)]
pub struct AuditReport {
    pub summary: AuditSummary,
    pub findings: Vec<AuditFinding>,
}

#[derive(serde::Deserialize)]
pub struct AuditSummary {
    pub critical: u32,
    pub warn: u32,
    pub info: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct AuditFinding {
    pub check_id: String,
    pub severity: String,
    pub title: String,
    pub detail: String,
    #[serde(default)]
    pub remediation: Option<String>,
}

/// When --fix is used, CLI may output { "fix": {...}, "report": {...} }.
#[derive(serde::Deserialize)]
pub struct AuditWithFix {
    pub report: AuditReport,
}

/// Result we expose to the frontend.
#[derive(serde::Serialize, Clone)]
pub struct AuditResult {
    pub score: u32,
    pub label: String,
    pub critical: u32,
    pub warn: u32,
    pub info: u32,
    pub message: String,
    pub findings: Vec<AuditFinding>,
}

/// Compute score from summary. Formula: 100 - critical*25 - warn*5 - info*1, clamped 1..=100.
pub fn compute_score(summary: &AuditSummary) -> u32 {
    let deduct = summary.critical * 25 + summary.warn * 5 + summary.info;
    let score = 100i32.saturating_sub(deduct as i32);
    score.clamp(1, 100) as u32
}

pub fn score_label(score: u32) -> &'static str {
    if score >= 80 {
        "Good"
    } else if score >= 50 {
        "Review recommended"
    } else {
        "Action needed"
    }
}

/// Run `openclaw security audit --json` or `--deep`.
/// Returns parsed result or error message. Uses cmd /c on Windows so openclaw.cmd works.
pub fn run_audit(deep: bool) -> Result<AuditResult, String> {
    let args: &[&str] = if deep {
        &["security", "audit", "--json", "--deep"]
    } else {
        &["security", "audit", "--json"]
    };
    let output = run::output("openclaw", args).map_err(|e| format!("Failed to run openclaw: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // CLI may output either { "report": {...} } (with fix) or the report object directly.
    let report: AuditReport = if let Ok(with_fix) = serde_json::from_str::<AuditWithFix>(&stdout) {
        with_fix.report
    } else if let Ok(r) = serde_json::from_str::<AuditReport>(&stdout) {
        r
    } else {
        return Err(format!(
            "Could not parse audit JSON. stdout: {} stderr: {}",
            stdout.trim(),
            stderr.trim()
        ));
    };

    let score = compute_score(&report.summary);
    let label = score_label(score).to_string();
    let message = format!(
        "{} critical, {} warn, {} info",
        report.summary.critical,
        report.summary.warn,
        report.summary.info
    );
    Ok(AuditResult {
        score,
        label,
        critical: report.summary.critical,
        warn: report.summary.warn,
        info: report.summary.info,
        message,
        findings: report.findings,
    })
}

/// Run `openclaw security audit --fix`. Returns (success, message). Uses cmd /c on Windows.
pub fn run_fix() -> (bool, String) {
    let output = run::output("openclaw", &["security", "audit", "--fix"]);
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let stderr = String::from_utf8_lossy(&o.stderr);
            if o.status.success() {
                (true, stdout.trim().to_string())
            } else {
                (false, format!("{} {}", stderr.trim(), stdout.trim()))
            }
        }
        Err(e) => (false, e.to_string()),
    }
}
