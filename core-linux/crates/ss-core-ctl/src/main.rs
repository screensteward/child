mod client;

use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about = "ScreenSteward admin CLI")]
struct Cli {
    /// Socket path (default /run/screensteward.sock, or `$SS_SOCKET_PATH`).
    #[arg(long)]
    socket: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// System / core status.
    Status,
    /// Login with password (prompt if absent).
    Login {
        #[arg(long)]
        password: Option<String>,
    },
    /// Logout the current connection.
    Logout,
    /// List all active policies for the current child.
    PolicyList,
    /// Raw JSON dump of family.get.
    FamilyDump,
    /// List pending extension requests.
    ExtensionsPending,
    /// Approve an extension ticket.
    ExtensionsApprove {
        ticket_id: uuid::Uuid,
        #[arg(long, default_value_t = 30)]
        duration_minutes: u32,
    },
    /// Manually unfreeze a cgroup scope (debug, Phase 1 stub).
    Unfreeze { app_id: String },
    /// Change the parent password (prompt).
    PasswordChange,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let sock = cli.socket.unwrap_or_else(|| {
        std::env::var("SS_SOCKET_PATH")
            .map_or_else(|_| "/run/screensteward.sock".into(), Into::into)
    });

    // Unfreeze is a Phase 1 stub — no socket connection needed.
    if let Cmd::Unfreeze { ref app_id } = cli.cmd {
        eprintln!(
            "manual unfreeze not exposed in Phase 1 — restart the daemon or edit the policy. (app_id={app_id})"
        );
        return Ok(());
    }

    let mut c = client::CtlClient::connect(&sock).await?;

    match cli.cmd {
        Cmd::Status => {
            login_prompt(&mut c).await?;
            let v = c.call("system.getCoreStatus", json!({})).await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
        }
        Cmd::Login { password } => {
            let p = match password {
                Some(p) => p,
                None => rpassword::prompt_password("password: ")?,
            };
            let v = c.call("auth.login", json!({ "password": p })).await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
        }
        Cmd::Logout => {
            let v = c.call("auth.logout", json!({})).await?;
            println!("{v}");
        }
        Cmd::PolicyList => {
            let v = c.call("policy.listActive", json!({})).await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
        }
        Cmd::FamilyDump => {
            login_prompt(&mut c).await?;
            let v = c.call("family.get", json!({})).await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
        }
        Cmd::ExtensionsPending => {
            login_prompt(&mut c).await?;
            let v = c.call("extension.listPending", json!({})).await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
        }
        Cmd::ExtensionsApprove {
            ticket_id,
            duration_minutes,
        } => {
            login_prompt(&mut c).await?;
            let v = c
                .call(
                    "extension.approve",
                    json!({
                        "ticket_id": ticket_id,
                        "duration_minutes": duration_minutes,
                    }),
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
        }
        Cmd::Unfreeze { .. } => {
            // Already handled above before socket connection.
            unreachable!()
        }
        Cmd::PasswordChange => {
            let old = rpassword::prompt_password("current password: ")?;
            c.call("auth.login", json!({ "password": old.clone() }))
                .await?;
            let new = rpassword::prompt_password("new password: ")?;
            let confirm = rpassword::prompt_password("confirm: ")?;
            if new != confirm {
                anyhow::bail!("passwords do not match");
            }
            let v = c
                .call("auth.changePassword", json!({ "old": old, "new": new }))
                .await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
        }
    }
    Ok(())
}

async fn login_prompt(c: &mut client::CtlClient) -> anyhow::Result<()> {
    let p = rpassword::prompt_password("password: ")?;
    c.call("auth.login", serde_json::json!({ "password": p }))
        .await?;
    Ok(())
}
