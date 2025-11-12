// batch-auditor/src/main.rs
// GitLoom Personal Portfolio Auditor
// Usage: cargo run

use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug)]
struct RepoAudit {
    name: String,
    path: PathBuf,
    score: u8,
    badges: Badges,
    missing: Vec<String>,
    forbidden: Vec<String>,
}

#[derive(Debug)]
struct Badges {
    dependable: bool,   // 🛡️ Has Rust/Ada/Elixir, no Python
    secure: bool,       // 🔒 Has Nickel, Justfile, no YAML
    interoperable: bool,// 🔗 Has AsciiDoc, void.ttl
    governed: bool,     // 📋 Has LICENSE, CONTRIBUTING.md
}

impl Badges {
    fn is_rhodium(&self) -> bool {
        self.dependable && self.secure && self.interoperable && self.governed
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        if self.dependable { s.push_str("🛡️ "); }
        if self.secure { s.push_str("🔒"); }
        if self.interoperable { s.push_str("🔗"); }
        if self.governed { s.push_str("📋"); }
        if self.is_rhodium() { s.push_str(" 💎"); }
        s
    }
}

fn has_file(path: &Path, pattern: &str) -> bool {
    // Handle exact filenames
    if !pattern.contains('*') {
        return path.join(pattern).exists();
    }

    // Handle glob patterns like "*.py"
    let extension = pattern.trim_start_matches("*.");
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            if let Some(ext) = entry.path().extension() {
                if ext == extension {
                    return true;
                }
            }
        }
    }
    false
}

fn audit_repo(path: &Path) -> RepoAudit {
    let name = path.file_name()
    .unwrap_or_default()
    .to_string_lossy()
    .to_string();

    // Check for dependable languages
    let has_rust = has_file(path, "Cargo.toml");
    let has_ada = has_file(path, "*.adb") || has_file(path, "*.ads");
    let has_elixir = has_file(path, "mix.exs");
    let has_python = has_file(path, "*.py");

    let dependable = (has_rust || has_ada || has_elixir) && !has_python;

    // Check for secure practices
    let has_nickel = has_file(path, "*.ncl");
    let has_justfile = has_file(path, "Justfile") || has_file(path, "justfile");
    let has_yaml = has_file(path, "*.yaml") || has_file(path, "*.yml");

    let secure = has_nickel && has_justfile && !has_yaml;

    // Check for interoperability
    let has_adoc = has_file(path, "*.adoc");
    let has_void = has_file(path, "void.ttl");

    let interoperable = has_adoc && has_void;

    // Check for governance
    let has_license = has_file(path, "LICENSE");
    let has_contributing = has_file(path, "CONTRIBUTING.md");
    let has_coc = has_file(path, "CODE_OF_CONDUCT.md");

    let governed = has_license && has_contributing;

    let badges = Badges {
        dependable,
        secure,
        interoperable,
        governed,
    };

    // Track what's missing
    let mut missing = Vec::new();
    let mut forbidden = Vec::new();

    if !has_license { missing.push("LICENSE".into()); }
    if !has_contributing { missing.push("CONTRIBUTING.md".into()); }
    if !has_justfile { missing.push("Justfile".into()); }
    if !has_void { missing.push("void.ttl".into()); }
    if !has_adoc && !has_file(path, "README.adoc") {
        missing.push("README.adoc".into());
    }
    if !has_nickel { missing.push("config.ncl".into()); }

    if has_python { forbidden.push("*.py (use Rust instead)".into()); }
    if has_yaml { forbidden.push("*.yaml/*.yml (use Nickel)".into()); }
    if has_file(path, "Makefile") { forbidden.push("Makefile (use Justfile)".into()); }

    // Calculate score (0-100)
    let badge_score = [
        badges.dependable,
        badges.secure,
        badges.interoperable,
        badges.governed,
    ].iter().filter(|&&b| b).count() * 25;

    let score = badge_score as u8;

    RepoAudit {
        name,
        path: path.to_path_buf(),
        score,
        badges,
        missing,
        forbidden,
    }
}

fn main() {
    println!("🔍 GitLoom Portfolio Audit");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // YOUR REPOS HERE - edit this list!
    let repos = vec![
        "/var/home/hyper/workspace/projects/to-git/_ou/gitloom",
        "/var/home/hyper/workspace/projects/echomesh",
        "/var/home/hyper/workspace/projects/oblibeny",
        // Add more repos here...
    ];

    let mut results = Vec::new();
    let mut total_score = 0u32;
    let mut count = 0u32;

    for repo_path in repos {
        let path = Path::new(repo_path);

        if !path.exists() {
            eprintln!("⚠️  Skipping (not found): {}", repo_path);
            continue;
        }

        let audit = audit_repo(path);

        // Display result
        let status = match audit.score {
            90..=100 => "✅",
            70..=89 => "⚠️ ",
            50..=69 => "⚠️ ",
            _ => "❌",
        };

        println!("{} {:30} {:>3}/100  {}",
                 status,
                 audit.name,
                 audit.score,
                 audit.badges.to_string()
        );

        if !audit.missing.is_empty() {
            println!("   Missing: {}", audit.missing.join(", "));
        }

        if !audit.forbidden.is_empty() {
            println!("   Forbidden: {}", audit.forbidden.join(", "));
        }

        if !audit.missing.is_empty() || !audit.forbidden.is_empty() {
            println!();  // Extra space for readability
        }

        total_score += audit.score as u32;
        count += 1;
        results.push(audit);
    }

    // Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n📊 Summary:");

    let perfect = results.iter().filter(|r| r.score == 100).count();
    let good = results.iter().filter(|r| r.score >= 70 && r.score < 100).count();
    let needs_work = results.iter().filter(|r| r.score >= 50 && r.score < 70).count();
    let critical = results.iter().filter(|r| r.score < 50).count();

    if perfect > 0 {
        println!("  💎 {} perfect (Rhodium Certified)", perfect);
    }
    if good > 0 {
        println!("  ✅ {} good (≥70)", good);
    }
    if needs_work > 0 {
        println!("  ⚠️  {} need work (50-69)", needs_work);
    }
    if critical > 0 {
        println!("  ❌ {} critical (<50)", critical);
    }

    if count > 0 {
        let avg = total_score / count;
        println!("\n  Average score: {}/100", avg);
    }

    println!("\nBadge Legend:");
    println!("  🛡️  Dependable (Rust/Ada/Elixir, no Python)");
    println!("  🔒 Secure (Nickel config, Justfile, no YAML)");
    println!("  🔗 Interoperable (AsciiDoc docs, VoID metadata)");
    println!("  📋 Governed (LICENSE, CONTRIBUTING.md)");
    println!("  💎 Rhodium Certified (all badges earned)");

    println!("\nNext steps:");
    println!("  1. Fix highest-impact issues first (missing LICENSE)");
    println!("  2. Run: gitloom fix --dry-run <repo-path> (coming soon!)");
    println!("  3. Commit fixes: gitloom fix --commit <repo-path>");
}
