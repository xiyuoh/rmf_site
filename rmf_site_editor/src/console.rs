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
// use bevy_log::LogSettings;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct ColorText;

pub struct LoggingConsolePlugin;

impl Plugin for LoggingConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(console_ui)
        .add_system(update_text);
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

fn console_ui(
    time: Res<Time>,
    mut egui_context: ResMut<EguiContext>,
) {
    // some logic goes here

    egui::TopBottomPanel::bottom("log_console")
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_space(5.);
            ui.heading("RMF Site Editor Console");
            ui.add_space(10.);
            
            ui.add(egui::Label::new("hello there"));
            ui.add(egui::Label::new("yo yo yo"));
        });
}

fn update_text(
    time: Res<Time>,
    mut query: Query<&mut Text, With<ColorText>>,
) {
    for mut text in &mut query {
        let seconds = time.delta_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba
        {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }

}
