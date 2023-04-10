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

use crate::{
    site::*,
    widgets::AppEvents,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CollapsingHeader, RichText, FontId, Color32, Ui},
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
    pub message: String,
}

#[derive(Resource)]
pub struct Logs {
    pub log_history: Vec<Log>,
    pub last_log_time: f32,
}

impl Default for Logs {
    fn default() -> Self {
        Self {
            log_history: vec![Log{
                category: LogCategory::Status,
                message: String::from("Entered RMF site editor!")
            }],
            last_log_time: 0.0,
        }
    }
}

impl Logs {

    pub fn status(&mut self, msg: &str) {
        self.append_log(LogCategory::Status, String::from(msg));
    }

    pub fn hint(&mut self, msg: &str) {
        self.append_log(LogCategory::Hint, String::from(msg));
    }

    pub fn warn(&mut self, msg: &str) {
        self.append_log(LogCategory::Warning, String::from(msg));
    }

    pub fn err(&mut self, msg: &str) {
        self.append_log(LogCategory::Error, String::from(msg));
    }

    fn append_log(&mut self, log_category: LogCategory, msg: String) {
        println!("{}", msg);
        let new_log = Log {
            category: log_category,
            message: msg,
        };
        self.log_history.push(new_log);
    }

    pub fn get_log_history(&mut self) -> &Vec<Log> {
        &self.log_history
    }

    pub fn get_last_line(&mut self) -> Option<&Log> {
        self.log_history.last()
    }
}

fn print_log(ui: &mut egui::Ui, log: &Log) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.5;

        // Match LogCategory to color
        let category_text_color = match log.category {
            LogCategory::Hint => Color32::LIGHT_GREEN,
            LogCategory::Status => Color32::WHITE,
            LogCategory::Warning => Color32::YELLOW,
            _ => Color32::RED,
        };
        ui.label(RichText::new(log.category.to_string()).color(category_text_color));
        ui.label(log.message.to_string());
    });
}

pub struct ViewConsole<'a, 'w2, 's2> {
    events: &'a mut AppEvents<'w2, 's2>,
}

impl<'a, 'w2, 's2> ViewConsole<'a, 'w2, 's2> {
    pub fn new(events: &'a mut AppEvents<'w2, 's2>) -> Self {
        Self { events }
    }

    pub fn show(mut self, ui: &mut Ui) {
        // access via self.events.display.logs.
        //

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.5;
            // ui.label("Latest log: ");
            let latest_log = self.events.display.logs.get_last_line().unwrap();
            print_log(ui, latest_log);
        });
        ui.add_space(5.0);
        CollapsingHeader::new("RMF Site Editor Console")
            .default_open(false)
            .show(ui, |ui| {
                //
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
                        for log in self.events.display.logs.get_log_history() {
                            print_log(ui, log);
                        }
                    });

                ui.add_space(10.);
            });
    }
}
