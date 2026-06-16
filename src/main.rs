use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Detek a mouche")]
    Mouche,
    #[command(description = "Eske l'abeille est contamnée par les pecticide")]
    Abeille,
    #[command(description = "On en parle plus que les gens ils peuvent manger les vers de terre")]
    VerDeTerre,
    #[command(description = "Detek")]
    Detek,
    #[command(description = "Pecticide sa contamine")]
    Pecticide,
    #[command(description = "kadav")]
    Kadav,
    #[command(description = "Ayamouche")]
    Ayamouche,
    #[command(description = "Les ordres de l'état")]
    Ordres,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Mouche => {
            bot.send_message(msg.chat.id, format!("🪰 MOUCHE DETEKTED")).await?
        }
        Command::Abeille => {
            bot.send_message(msg.chat.id, format!("🐝 ABEILLE SUR LE PECTICIDE")).await?
        }
        Command::Detek => {
            bot.send_message(msg.chat.id, format!("🚨 DETEK DETEK DETEK")).await?
        }
        Command::VerDeTerre => {
            bot.send_message(msg.chat.id, format!("😨 On n'en parle plus 🤫🤫 mais les gens 🧑‍🤝‍🧑 ils peuvent ⚡ manger les vers 🪱 de terre")).await?
        }
        Command::Ayamouche => {
            bot.send_message(msg.chat.id, format!("😆")).await?
        }
        Command::Pecticide => {
            bot.send_message(msg.chat.id, format!("☢️ PECTICIDE DETEKTÉ SUR L'ABEILLE")).await?
        }
        Command::Kadav => {
            bot.send_message(msg.chat.id, format!("💀 MOUCHE DETEKTED SUR LE KADAV")).await?
        }
        Command::Ordres => {
            bot.send_message(msg.chat.id, format!("Vous 🫵 avez des ordres 🫡 de l'Etat 🇺🇸🦅🦅 bah 🙄 oui 🙂‍↕️")).await?
        }
    };

    Ok(())
}
