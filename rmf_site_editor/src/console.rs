/*
 * Copyright (C) 2022 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use tracing::{event, info, instrument, span, Level};
use tracing_subscriber;

pub struct LoggingConsolePlugin;

impl Plugin for LoggingConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system(console_ui);
        // .add_system_set(
        //     SystemSet::on_update(AppState::________)
        //         .with_system()
        // );
        // app.insert_resource(LogSettings {
        //     level: bevy_utils::tracing::Level::INFO,
        //     filter: "warn,librmf_site_editor=info".to_string(),
        // });
        
    }
}

fn setup(
    //
) {
    // tracing_subscriber::fmt::init();

    // let collector = tracing_subscriber::fmt()
    //     .with_max_level(Level::TRACE)
    //     .finish();

    // let number_of_yaks = 3;

    // tracing::collect::with_default(collector, || {
    //     info!("this will be logged???");
    // });
    // info!(number_of_yaks, "preparing to shave yaks");

    // info!("yak shaving completed.");
    println!("------------------------------------- YOYOYOYOYO")
}

fn console_ui(
    // arguments go here
    mut egui_context: ResMut<EguiContext>,
) {
    event!(Level::INFO, "something happened");

    let span = span!(Level::TRACE, "my_span");
    let _guard = span.enter();


    event!(Level::INFO, printout = "testing");
    event!(Level::INFO, "something happened inside my span");


    // some logic goes here

    egui::TopBottomPanel::bottom("log_console")
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_space(5.);
            ui.heading("RMF Site Editor Console");
            ui.add_space(10.);
            
            ui.label("hello there");
            ui.label("yoyo wassup");
        });
}

fn log_generator(
    mut commands: Commands,
    mut spawner: Spawner,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn();
}

// #[instrument]
// pub fn my_function(
//     my_arg: usize
// ) {
//     event!(Level::INFO, "inside my_function!");
// }

// fn update_text(
//     diagnostics: Res<Diagnostics>,
//     mut query: Query<&mut Text, With<FpsText>>,
// ) {
//     for mut text in &mut query {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS)
//         {
//             if let Some(value) = fps.smoothed()
//             {
//                 // Update the value of the second section
//                 text.sections[1].value = format!("{value:.2}");
//             }
//         }
//     }

// }