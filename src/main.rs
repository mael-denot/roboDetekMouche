use std::{collections::HashMap, sync::Arc};

use teloxide::{prelude::*, types::*, utils::command::BotCommands};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    let state = Arc::new(Mutex::new(HashMap::<(i64, i64), i64>::new()));

    tokio::spawn({
        let state = Arc::clone(&state);
        async move {
            loop {
                sleep(Duration::from_secs(24 * 60 * 60)).await;
                let mut state = state.lock().await;
                for count in state.values_mut() {
                    if *count > 0 {
                        *count -= 1;
                    }
                }
                log::info!("douche temps");
            }
        }
    });

    Command::repl(bot, move |bot, msg, cmd| {
        let state = Arc::clone(&state);
        async move { answer(bot, msg, cmd, state).await }
    })
    .await;
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
    #[command(description = "c'est bien la propreté")]
    Douche,
    #[command(description = "help me !!")]
    Help,
    #[command(description = "philospoher")]
    OnPourraitPhilosopher,
    #[command(description = "J'ai été fou")]
    Fou,
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    state: Arc<Mutex<HashMap<(i64, i64), i64>>>,
) -> ResponseResult<()> {
    match cmd {
        Command::Mouche => {
            bot.send_message(msg.chat.id, "🪰 MOUCHE DETEKTED".to_string()).await?
        }
        Command::Abeille => {
            bot.send_message(msg.chat.id, "🐝 ABEILLE SUR LE PEKTICIDE".to_string()).await?
        }
        Command::Detek => {
            bot.send_message(msg.chat.id, "🚨 DETEK DETEK DETEK".to_string()).await?
        }
        Command::VerDeTerre => {
            bot.send_message(msg.chat.id, "😨 On n'en parle plus 🤫🤫 mais les gens 🧑‍🤝‍🧑 ils peuvent ⚡ manger les vers 🪱 de terre".to_string()).await?
        }
        Command::Ayamouche => {
            bot.send_message(msg.chat.id, "😆".to_string()).await?
        }
        Command::Pecticide => {
            bot.send_message(msg.chat.id, "☢️ PECTICIDE DETEKTÉ SUR L'ABEILLE".to_string()).await?
        }
        Command::Kadav => {
            bot.send_message(msg.chat.id, "💀 MOUCHE DETEKTED SUR LE KADAV".to_string()).await?
        }
        Command::OrdresDeLEtat => {
            bot.send_message(msg.chat.id, "Vous 🫵 avez des ordres 🫡 de l'Etat 🇺🇸🦅🦅 bah 🙄 oui 🙂‍↕️".to_string()).await?
        }
        Command::Mind => {
            let audio = InputFile::file("media/MindBoggling.m4a");
            bot.send_audio(msg.chat.id, audio).await?
        }
        Command::Robot => {
            let audio = InputFile::file("media/robotDetekteur.mp3");
            bot.send_audio(msg.chat.id, audio).await?
        }
        Command::Douche => {
            let chat_id = msg.chat.id.0;
            let user_id = msg.from().map(|user| user.id.0).unwrap_or(chat_id);
            let mut state = state.lock().await;
            let key = (chat_id, user_id);
            let new_count = state
                .entry(key)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            bot.send_message(msg.chat.id, format!("💦 Douche count: {new_count}")).await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, "No. You go get help".to_string()).await?
        }
        Command::OnPourraitPhilosopher => {
            bot.send_message(msg.chat.id, "on pourrait philosopher pendant très longtemps sur komment nous arrivent euhh certaines remarkes et euh notamment j'aurais une konception de la vie à voir, notamment et donc euh euh euhm komment dire, sur le, alors oui j’avais diskuté en fait euh, alors étant libriste euh libriste en ce moment, euh donc euh j’ai posé une kestion euhm euh euh euhm à une sane euhm, oui vive, vive henria, j’ai fait une remarke ils ont kréé un robo libre hein au libre, on dit didroam c’est pas de droit k’il faut dire effectivement, et donc euhh, j’ai eu komme réponse d’un chercheur euh, vive d’assos et donc euhh j’ai eu komme réponse euhh de l’élu euh dialektik k’on pourrait euh, ki lui, ki lui a demandé de venir euh là eskop le pavé voilà et donc euhh euhh, la kestion c’est par rapport à euh Louis XVI pouvez vous me parler de Turgot".to_string()).await?
        }
        Command::Glycine => {
            bot.send_message(msg.chat.id, "si les nuages étaient solides
quelqu'un ferait du cross-country là-bas
et si j'étais liquide
quelqu'un se laverait les mains

la règle trente-quatre existe partout
pas seulement dans le sphère
la nature crie les règles du jeu
on peut règler monsieur oui bien sûr

il y a aussi des règles en écriture
s'il y a un mot il y aura quelqu'un pour l'écrire
le rendre solide et moi propre et nature
contente de son rire".to_string()).await?
        }
        Command::Fou => {
            bot.send_message(msg.chat.id, format!("Fou? J'ai été fou une fois... Ils m'ont mis dans une pièce. Il y avait un jonas et un nathan dans la pièce. Ils m'ont parlé de mouches jusqu'à ce que je devienne fou. Fou? J'ai été /fou une fois...")).await?
        }
    };

    Ok(())
}

