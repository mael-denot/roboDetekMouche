use teloxide::{prelude::*, utils::command::BotCommands, types::*};

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
    OrdresDeLEtat,
    #[command(description = "This is mind boggeling")]
    Mind,
    #[command(description = "Eskil a été mis au point un robo ki detek")]
    Robot,
    #[command(description = "glycine")]
    Glycine,
    #[command(description = "help me !!")]
    Help,
    #[command(description = "philospoher")]
    OnPourraitPhilosopher,
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
        Command::OrdresDeLEtat => {
            bot.send_message(msg.chat.id, format!("Vous 🫵 avez des ordres 🫡 de l'Etat 🇺🇸🦅🦅 bah 🙄 oui 🙂‍↕️")).await?
        }
        Command::Mind => {
            let audio = InputFile::file("media/MindBoggling.m4a");
            bot.send_audio(msg.chat.id, audio).await?
        }
        Command::Robot => {
            let audio = InputFile::file("media/robotDetekteur.mp3");
            bot.send_audio(msg.chat.id, audio).await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, format!("No. You go get help")).await?
        }
        Command::OnPourraitPhilosopher => {
            bot.send_message(msg.chat.id, format!("on pourrait philosopher pendant très longtemps sur komment nous arrivent euhh certaines remarkes et euh notamment j'aurais une konception de la vie à voir, notamment et donc euh euh euhm komment dire, sur le, alors oui j’avais diskuté en fait euh, alors étant libriste euh libriste en ce moment, euh donc euh j’ai posé une kestion euhm euh euh euhm à une sane euhm, oui vive, vive henria, j’ai fait une remarke ils ont kréé un robo libre hein au libre, on dit didroam c’est pas de droit k’il faut dire effectivement, et donc euhh, j’ai eu komme réponse d’un chercheur euh, vive d’assos et donc euhh j’ai eu komme réponse euhh de l’élu euh dialektik k’on pourrait euh, ki lui, ki lui a demandé de venir euh là eskop le pavé voilà et donc euhh euhh, la kestion c’est par rapport à euh Louis XVI pouvez vous me parler de Turgot")).await?
        }
        Command::Glycine => {
            bot.send_message(msg.chat.id, format!("si les nuages étaient solides\nquelqu'un ferait du cross-country là-bas\net si j'étais liquide\nquelqu'un se laverait les mains\n\nla règle trente-quatre existe partout\npas seulement dans le sphère\nla nature crie les règles du jeu\non peut règler monsieur oui bien sûr\nil y a aussi des règles en écriture\ns'il y a un mot il y aura quelqu'un pour l'écrire\nle rendre solide et moi propre et nature\ncontente de son rire")).await?
        }
    };

    Ok(())
}
