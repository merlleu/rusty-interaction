#[macro_use] extern crate rusty_interaction;

use rusty_interaction::handler::InteractionHandler;
use rusty_interaction::types::interaction::*;
// Import for using components
use rusty_interaction::types::components::*;


use std::time::Duration;
use async_std::task;

use rusty_interaction::actix::Arbiter;

const PUB_KEY: &str = "YOUR_PUBLIC_KEY";
const APP_ID: u64 = 0; 


// Use the component_handler macro.
#[component_handler]
async fn edit_button(ctx: Context) -> InteractionResponse{
    return ctx.respond().message("HAHA").finish();
}

// We defer in this instance, because we don't want to edit anything
#[component_handler]
#[defer]
async fn delete_button(ctx: Context) -> InteractionResponse{
    ctx.delete_original().await;

    // Since we've deleted the original message, it's safe to use respond().none()
    return ctx.respond().none();
}
#[slash_command]
async fn test(ctx: Context) -> InteractionResponse{

    // Let's build our message!
    let resp = ctx.respond()
            // Set message content
            .content("Not edited")
            // add a component action row using it's builder
            .add_component_row(
                &ComponentRowBuilder::default()
                    // Add buttons using it's builder
                    .add_button(
                        ComponentButtonBuilder::default()
                                        .label("Edit")
                                        .custom_id("HEHE")
                                        .style(ComponentButtonStyle::Primary)
                                        .finish()
                    )
                    .add_button(
                        ComponentButtonBuilder::default()
                                        .label("Delete")
                                        .custom_id("DELETE")
                                        .style(ComponentButtonStyle::Danger)
                                        .finish()
                    )
                .finish()
            )
            .finish();

    return resp;
}



// The lib uses actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    
    let mut handle = InteractionHandler::new(APP_ID, PUB_KEY, None);
    
    
    handle.add_global_command("summon", test);
 
    // Here we attach our custom ids we defined with our buttons to the handler
    handle.add_component_handle("EDIT_BUTTON_PRIMARY", edit_button);
    handle.add_component_handle("DELETE_BUTTON", delete_button);


    return handle.run(10443).await;
    
}

