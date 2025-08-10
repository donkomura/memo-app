use clap::{Parser, Subcommand};
use memo_app::client::HttpClient;
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about = "memo-app CLI")]
struct Cli {
    #[command(subcommand)]
    commands: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Signup {
        email: String,
        password: String,
    },
    Login {
        email: String,
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
        title: String,
        content: String,
    },
    Update {
        id: i64,
        title: Option<String>,
        content: Option<String>,
    },
    Delete {
        id: i64,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let http = HttpClient::new(std::env::var("MEMO_API_BASE").unwrap_or_else(|_| "http://localhost:8080".into()));

    match cli.commands {
        Command::Signup { email, password } => {
            #[derive(Serialize)]
            struct Body<'a> { email: &'a str, password: &'a str }
            let (status, text) = http.post_json("/auth/signup", &Body { email: &email, password: &password }, None).await.expect("request failed");
            println!("{} {}", status, text);
        }
        Command::Login { email, password } => {
            #[derive(Serialize)]
            struct Body<'a> { email: &'a str, password: &'a str }
            let (status, text) = http.post_json("/auth/login", &Body { email: &email, password: &password }, None).await.expect("request failed");
            println!("{} {}", status, text);
        }
        Command::Note { command: NoteCommand::List } => {
            let (status, text) = http.get("/notes", None).await.expect("request failed");
            println!("{} {}", status, text);
        }
        Command::Note { command: NoteCommand::Create { title, content } } => {
            #[derive(Serialize)]
            struct Body<'a> { title: &'a str, content: &'a str }
            let (status, text) = http.post_json("/notes", &Body { title: &title, content: &content }, None).await.expect("request failed");
            println!("{} {}", status, text);
        }
        Command::Note { command: NoteCommand::Update { id, title, content } } => {
            #[derive(Serialize)]
            struct Body<'a> { title: Option<&'a str>, content: Option<&'a str> }
            let (status, text) = http.put_json(&format!("/notes/{}", id), &Body { title: title.as_deref(), content: content.as_deref() }, None).await.expect("request failed");
            println!("{} {}", status, text);
        }
        Command::Note { command: NoteCommand::Delete { id } } => {
            let (status, text) = http.delete(&format!("/notes/{}", id), None).await.expect("request failed");
            println!("{} {}", status, text);
        }
    }
}
