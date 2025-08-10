use clap::{Parser, Subcommand};
use memo_app::client::HttpClient;
use memo_app::domain::model::Note;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Config {
    api_base: String,
    token: Option<String>,
}

fn load_cfg() -> Config {
    confy::load("memoctl", None).unwrap_or_else(|_| Config {
        api_base: std::env::var("MEMO_API_BASE").unwrap_or_else(|_| "http://localhost:8080".into()),
        token: None,
    })
}

fn store_cfg(cfg: &Config) {
    let _ = confy::store("memoctl", None, cfg);
}

#[derive(Parser)]
#[command(author, version, about = "memo-app CLI")]
struct Cli {
    #[command(subcommand)]
    commands: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Signup {
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    Login {
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    Note {
        #[command(subcommand)]
        command: NoteCommand,
    },
}

#[derive(Subcommand, Debug)]
enum NoteCommand {
    List,
    Create {
        #[arg(short, long)]
        title: String,
        content: String,
    },
    Update {
        #[arg(short, long)]
        id: i64,
        #[arg(short, long)]
        title: Option<String>,
        content: Option<String>,
    },
    Delete {
        #[arg(short, long)]
        id: i64,
    },
}

#[actix_rt::main]
async fn main() {
    let cli = Cli::parse();
    let mut cfg = load_cfg();
    let http =
        HttpClient::new(std::env::var("MEMO_API_BASE").unwrap_or_else(|_| cfg.api_base.clone()));

    match cli.commands {
        Command::Signup { email, password } => {
            #[derive(Serialize)]
            struct Body<'a> {
                email: &'a str,
                password: &'a str,
            }
            let (status, text) = http
                .post_json(
                    "/auth/signup",
                    &Body {
                        email: &email,
                        password: &password,
                    },
                    None,
                )
                .await
                .expect("request failed");
            println!("{} {}", status, text);
        }
        Command::Login { email, password } => {
            #[derive(Serialize)]
            struct Body<'a> {
                email: &'a str,
                password: &'a str,
            }
            let (status, text) = http
                .post_json(
                    "/auth/login",
                    &Body {
                        email: &email,
                        password: &password,
                    },
                    None,
                )
                .await
                .expect("request failed");
            if status == 200
                && let Ok(v) = serde_json::from_str::<serde_json::Value>(&text)
                && let Some(token) = v.get("token").and_then(|t| t.as_str())
            {
                cfg.token = Some(token.to_string());
                store_cfg(&cfg);
                println!("Logged in. Token saved.");
                return;
            }
            println!("{} {}", status, text);
        }
        Command::Note {
            command: NoteCommand::List,
        } => {
            let notes: Vec<Note> = http
                .get_json("/notes", cfg.token.as_deref())
                .await
                .expect("request failed");
            println!(
                "{}",
                serde_json::to_string_pretty(&notes).unwrap_or_default()
            );
        }
        Command::Note {
            command: NoteCommand::Create { title, content },
        } => {
            #[derive(Serialize)]
            struct Body<'a> {
                title: &'a str,
                content: &'a str,
            }
            let note: Note = http
                .post_json_typed(
                    "/notes",
                    &Body {
                        title: &title,
                        content: &content,
                    },
                    cfg.token.as_deref(),
                )
                .await
                .expect("request failed");
            println!(
                "{}",
                serde_json::to_string_pretty(&note).unwrap_or_default()
            );
        }
        Command::Note {
            command: NoteCommand::Update { id, title, content },
        } => {
            #[derive(Serialize)]
            struct Body<'a> {
                title: Option<&'a str>,
                content: Option<&'a str>,
            }
            let note: Note = http
                .put_json_typed(
                    &format!("/notes/{}", id),
                    &Body {
                        title: title.as_deref(),
                        content: content.as_deref(),
                    },
                    cfg.token.as_deref(),
                )
                .await
                .expect("request failed");
            println!(
                "{}",
                serde_json::to_string_pretty(&note).unwrap_or_default()
            );
        }
        Command::Note {
            command: NoteCommand::Delete { id },
        } => {
            let (status, text) = http
                .delete(&format!("/notes/{}", id), cfg.token.as_deref())
                .await
                .expect("request failed");
            println!("{} {}", status, text);
        }
    }
}
