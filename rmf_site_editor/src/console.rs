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
use bevy_egui::{
    egui::{self, CollapsingHeader, RichText, FontId, Color32},
    EguiContext
};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum LogCategory {
    Hint,
    Status,
    Warning,
    Error,
}

impl fmt::Display for LogCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogCategory::Hint => write!(f, "[HINT] "),
            LogCategory::Status => write!(f, "[STATUS] "),
            LogCategory::Warning => write!(f, "[WARNING] "),
            LogCategory::Error => write!(f, "[ERROR] "),
        }
    }
}

#[derive(Debug, Component)]
pub struct Log {
    pub category: LogCategory,
    pub content: String,
}

// create logging console resource
#[derive(Debug, Resource)]
pub struct Logs {
    pub expand_logs: bool,
    pub log_history: Vec<Log>,
    pub last_log_time: f32,
}

impl Logs {
    pub fn add_new_log(&mut self, log_category: LogCategory, log_content: String) {
        let new_log = Log {
            category: log_category,
            content: log_content,
        };
        self.log_history.push(new_log);
    }

    pub fn get_log_history(&mut self) -> &Vec<Log> {
        &self.log_history
    }

    pub fn get_last_line(&mut self) -> Option<&Log> {
        self.log_history.last()
    }

    pub fn get_last_log_time(&mut self) -> f32 {
        self.last_log_time
    }

    pub fn update_last_log_time(&mut self, new_time: f32) {
        self.last_log_time = new_time
    }
}

fn print_log(ui: &mut egui::Ui, log: &Log, time: f32) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.5;

        // Match LogCategory to color
        let category_text_color = match log.category {
            //
            LogCategory::Hint => Color32::LIGHT_GREEN,
            LogCategory::Status => Color32::WHITE,
            LogCategory::Warning => Color32::YELLOW,
            _ => Color32::RED,
        };
        // ui.label("Latest log: ");
        ui.label(RichText::new(log.category.to_string()).color(category_text_color));
        ui.label(log.content.to_string());
        // ui.label(time.to_string());
    });
}

impl FromWorld for Logs {
    fn from_world(world: &mut World) -> Self {
        let mut initial_logs = Vec::new();
        let first_log = Log {
            category: LogCategory::Status,
            content: String::from("Loading site editor..."),
        };
        initial_logs.push(first_log);

        Logs {
            expand_logs: true,
            log_history: initial_logs,
            last_log_time: 0.,
        }
    }
}

pub struct LoggingConsolePlugin;

impl Plugin for LoggingConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Logs>()
        .add_system(console_ui);
    }
}

fn console_ui(
    // commands: Commands,
    mut logs: ResMut<Logs>,
    time: Res<Time>,
    mut egui_context: ResMut<EguiContext>,
) {

    let time_now = time.elapsed_seconds();

    egui::TopBottomPanel::bottom("log_console")
        .resizable(true)
        .min_height(30.)
        .max_height(160.)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_space(10.);
            ui.vertical(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.5;
                    ui.label("Latest log: ");
                    let latest_log = &logs.get_last_line().unwrap();
                    print_log(ui, latest_log, time_now);
                });
                ui.add_space(5.0);
                // let latest_log = "Latest log: ".to_owned() + &logs.get_last_line().unwrap().category.to_string() + &logs.get_last_line().unwrap().content;
                // let latest_log = String::from("site editor console");
                CollapsingHeader::new("RMF Site Editor Console")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            if ui.button("Log History").clicked() {
                                // do something here
                            }
                        });
                        ui.add_space(10.);

                        egui::ScrollArea::both()
                            .auto_shrink([false, false])
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                if time_now > logs.get_last_log_time() + 5.0 {
                                    // every 5 seconds we add a new log
                                    match &logs.get_last_line().unwrap().category {
                                        LogCategory::Hint => logs.add_new_log(
                                            LogCategory::Status,
                                            String::from("This is a status log.")
                                        ),
                                        LogCategory::Status => logs.add_new_log(
                                            LogCategory::Warning,
                                            String::from("This is a warning log.")
                                        ),
                                        LogCategory::Warning => logs.add_new_log(
                                            LogCategory::Error,
                                            String::from("This is an error log.")
                                        ),
                                        LogCategory::Error => logs.add_new_log(
                                            LogCategory::Hint,
                                            String::from("This is a hint log.")
                                        ),
                                    }
                                    // logs.add_new_log(
                                    //     LogCategory::Status,
                                    //     String::from("this is a new log")
                                    // );

                                    // update last log time
                                    logs.update_last_log_time(time_now);
                                }

                                // try to print logs to egui
                                for log in logs.get_log_history() {
                                    print_log(ui, log, time_now);
                                }
                        });
                        ui.add_space(10.);
                    });
            });
        });
}
