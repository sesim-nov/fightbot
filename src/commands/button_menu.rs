use crate::{Context, Error};
use poise::serenity_prelude::{
    self as serenity, ComponentInteraction, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuOption, 
};

/// Main Menu
#[poise::command(slash_command)]
pub async fn main_menu(ctx: Context<'_>) -> Result<(), Error> {
    let buttons = vec![
        serenity::CreateButton::new("casual_match").label("Casual Match"),
        serenity::CreateButton::new("ranked_match").label("Sweatlord Match (coming soon)"),
    ];
    let components = serenity::CreateActionRow::Buttons(buttons);
    let embed = serenity::CreateEmbed::new()
        .color(serenity::Color::DARK_ORANGE)
        .field(
            "PVP Bot: Main Menu",
            "Welcome to the main menu. Do you want to start an unranked or ranked match?",
            false,
        );

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![components]);

    ctx.send(reply).await?;

    main_menu_responder(ctx).await
}

async fn main_menu_responder(ctx: Context<'_>) -> Result<(), Error> {
    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .await
    {
        if mci.data.custom_id == "casual_match" {
            draw_casual_menu(ctx, mci).await?;
            break;
        } else if mci.data.custom_id == "ranked_match" {
            mci.create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::new()
                        .content("Let's get sweaty, baby."),
                ),
            )
            .await?;
            break;
        }
    }
    Ok(())
}

async fn draw_casual_menu(ctx: Context<'_>, mci: ComponentInteraction) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .color(serenity::Color::DARK_GREEN)
        .field("Casual Menu", "Select the Team Size", false);
    let casual_menu = CreateSelectMenu::new(
        "casual_menu",
        serenity::CreateSelectMenuKind::String {
            options: vec![CreateSelectMenuOption::new("2v2", "2")],
        },
    );
    let resp_msg = CreateInteractionResponseMessage::new()
        .select_menu(casual_menu)
        .embed(embed);
    let resp = CreateInteractionResponse::UpdateMessage(resp_msg);
    mci.create_response(ctx.serenity_context(), resp).await?;
    Ok(())
}