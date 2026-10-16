//! `fr2027 backtest`: replay the model on past elections, score it, fit the error settings, and
//! write `research/backtest/results.json` and `report.md`.

use std::path::PathBuf;

use anyhow::Result;
use fr2027_collectors::Repo;

#[derive(clap::Args)]
pub struct Args {
    /// Override the number of simulations per replay and setting in config/backtest.yaml.
    #[arg(long)]
    simulations: Option<u32>,
    /// Write here instead of research/backtest/.
    #[arg(long)]
    out_dir: Option<PathBuf>,
}

pub const OUT_DIR: &str = "research/backtest";

pub fn run(repo: &Repo, args: &Args) -> Result<()> {
    let report = fr2027_backtest::run(repo, args.simulations, &mut |line| eprintln!("{line}"))?;
    let out_dir = args.out_dir.clone().unwrap_or_else(|| repo.root().join(OUT_DIR));
    std::fs::create_dir_all(&out_dir)?;
    let mut json = serde_json::to_vec_pretty(&report)?;
    json.push(b'\n');
    std::fs::write(out_dir.join("results.json"), json)?;
    std::fs::write(out_dir.join("report.md"), report.markdown())?;

    let line = |label: &str, s: &fr2027_backtest::score::Summary| {
        println!(
            "  {label:<22} qualify Brier {:.3} log-loss {:.3} | win Brier {:.3} log-loss {:.3} | share log score {:.1} | round 2 log score {:.2}",
            s.qualify_brier, s.qualify_log_loss, s.win_brier, s.win_log_loss, s.share_log_score, s.round2_log_score
        );
    };
    println!("wrote {}", repo.relative(&out_dir.join("report.md")));
    line("latest-poll average", &report.baseline.overall);
    line("model, current", &report.current_scores.overall);
    line("model, fitted", &report.fitted_scores.overall);
    for param in fr2027_backtest::report::Param::ALL {
        println!(
            "  {:<34} current {:<6} fitted {}",
            param.name(),
            param.get(&report.current),
            param.get(&report.fitted)
        );
    }
    Ok(())
}
