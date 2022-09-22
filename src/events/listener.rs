use crate::{Data, Error};
use poise::serenity_prelude;
use poise::serenity_prelude::{Activity, OnlineStatus};

pub async fn event_listener(
    _ctx: &serenity_prelude::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    /*
        Runs an event listener using Serenity's built-in listener to set the status and presence to online
    */
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);

            let activity = Activity::playing("with dice");
            let status = OnlineStatus::Online;

            _ctx.set_presence(Some(activity), status).await;
        }
        _ => {}
    }

    Ok(())
}
