//! Pong Tutorial 1

use amethyst::{
    prelude::*,
    renderer::{//RenderFlat2D plugin is used to render entities with a 'SpriteRender'  componet.
        plugins::{RenderFlat2D, RenderToWindow}, //RenderToWindows-scaffolding for opening a windows and drawing on it
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};


pub struct Pong;

impl SimpleState for Pong{

}

fn main() -> amethyst::Result<()>{
    amethyst::start_logger(Default::default()); //log error messages
    let app_root = application_root_dir()?; // app directory
    let display_config_path = app_root.join("config").join("display.ron"); // the config file we just created

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindows plugin provides all the scaffolding for opening a windows and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]), // the window background colour 
            )
            //RenderFlat2D plugin is used to render entities with a 'SpriteRender'  componet.
            .with_plugin(RenderFlat2D::default()),
        )?;// new instance of GameBuilder

    let assets_dir = app_root.join("assests");
    //creating the overarching Amethyst's root object: Application. It binds the OS event loop, 
    //state machines, timers and other core components in a central place.
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run(); //starts the game loop, the game will run untill SimpleState returns Trans::Quit

    
    Ok(())
}