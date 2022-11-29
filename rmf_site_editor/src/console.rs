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

pub struct LoggingConsolePlugin;

impl Plugin for LoggingConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(console_ui);
        // .add_system_set(
        //     SystemSet::on_update(AppState::________)
        //         .with_system()
        // );
        
    }
}

fn console_ui(
    // arguments go here
    mut egui_context: ResMut<EguiContext>,
) {
    // some logic goes here

    egui::TopBottomPanel::bottom("log_console")
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_space(5.);
            ui.heading("RMF Site Editor Console");
            ui.add_space(10.);
            let r = egui::ScrollArea::both()
                .auto_shrink([false, false]);
        });
}