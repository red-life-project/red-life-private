use crate::backend::constants::DESIRED_FPS;
use crate::backend::gamestate::GameState;
use crate::backend::screen::{Popup, StackCommand};
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::languages;
use crate::languages::german::{
    INFORMATIONSPOPUP_MARS, INFORMATIONSPOPUP_NASA, KOMETENEINSCHLAG, SANDSTURM, STROMAUSFALL,
};
use crate::languages::german::{MARS_INFO, NASA_INFO, WARNINGS};
use crate::machines::machine::State;
use ggez::graphics::Color;
use ggez::Context;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use tracing::info;

/// defines an event which has an impact on the game and the player
/// for example a popup or a change in the player's resources
/// events can just fade off or stay as long as the player didnt interact with them

// resources
//TODO: trigger no_energy() on machine and spawn Loch instead
pub const DEBUG_CR: Resources<i16> = Resources {
    oxygen: 10,
    energy: 10,
    life: 0,
};
pub const STROMAUSFALL_CR: Resources<i16> = Resources {
    oxygen: 0,
    energy: 10,
    life: 0,
};
pub const KOMETENEINSCHLAG_CR: Resources<i16> = Resources {
    oxygen: 10,
    energy: 0,
    life: 0,
};
pub const SANDSTURM_CR: Resources<i16> = Resources {
    oxygen: 5,
    energy: 5,
    life: 0,
};
// for info events
pub const NO_CHANGE: Resources<i16> = Resources {
    oxygen: 0,
    energy: 0,
    life: 0,
};

/// Defines an event in the game
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct Event {
    name: String,
    info_text: String,
    pub(crate) resources: Option<Resources<i16>>,
    duration: i32,
    popup_type: String,
    popup_message: String,
}
impl Event {
    /// create new event
    pub fn new(
        event: [&str; 2],
        popup_message: &str,
        popup_type: &str,
        resources: Resources<i16>,
        duration: i32,
    ) -> Self {
        // Self::send_popup(popup_message, sender, popup_type, event[0]);
        info!(
            "New event created: {}, info text: {}",
            event[0].to_string(),
            event[1].to_string()
        );
        Self {
            name: event[0].to_string(),
            info_text: event[1].to_string(),
            resources: Option::from(resources),
            duration: duration * (DESIRED_FPS as i32),
            popup_type: popup_type.to_string(),
            popup_message: popup_message.to_string(),
        }
    }

    /// if no Event is active it either chooses a random event of the Event enum or nothing every 60 seconds
    pub fn event_generator(sender: &Sender<StackCommand>) -> Option<Event> {
        let rng = fastrand::Rng::new();
        let event = rng.usize(..50);
        match event {
            0 => Some(Event::new(
                KOMETENEINSCHLAG,
                WARNINGS[0],
                "warning",
                KOMETENEINSCHLAG_CR,
                10,
            )),
            11 => Some(Event::new(
                INFORMATIONSPOPUP_NASA,
                NASA_INFO[rng.usize(..4)],
                "nasa",
                NO_CHANGE,
                10,
            )),
            22 => Some(Event::new(
                SANDSTURM,
                WARNINGS[2],
                "warning",
                SANDSTURM_CR,
                10,
            )),
            33 => Some(Event::new(
                STROMAUSFALL,
                WARNINGS[1],
                "warning",
                STROMAUSFALL_CR,
                10,
            )),
            44 => Some(Event::new(
                INFORMATIONSPOPUP_MARS,
                MARS_INFO[rng.usize(..5)],
                "mars",
                NO_CHANGE,
                10,
            )),
            _ => None,
        }
    }
    /// Sends a popup of an event to the screen
    /// # Arguments
    /// * `popup_message` - The message which should be displayed in the popup
    /// * `sender` - The sender which is used to send the popup to the screen
    /// * `popup_type` - The type of the popup, which is used to determine the color of the popup
    /// * `event_name` - The name of the event, which is used to determine what Event name should be displayed in the popup
    pub fn send_popup(
        popup_message: &str,
        sender: &Sender<StackCommand>,
        popup_type: &str,
        event_name: &str,
    ) {
        let popup = match popup_type {
            "warning" => Popup::warning(popup_message.to_string()),
            "nasa" => Popup::nasa(popup_message.to_string()),
            "mars" => Popup::mars(popup_message.to_string()),
            _ => Popup::new(Color::RED, "Error".to_string(), 10),
        };
        sender.send(StackCommand::Popup(popup)).unwrap();
        info!(
            "Event Popup sent: name: {}, Popup-Message: {}, Popup-Type: {}",
            event_name,
            popup_message.to_string(),
            popup_type
        );
    }
    /// Check if event is still active
    pub fn is_active(&self) -> bool {
        // check if time since event creation is greater than the duration of the event
        !self.duration <= 0
    }
    /// Triggers the event and activates its effect
    /// # Arguments
    /// * `restore` - If true the event will be deactivated and the resources will be restored
    /// * `gamestate` - The gamestate which is used to access the player and the machines
    /// * `sender` - The sender which is used to send the popup to the screen
    pub fn action(&self, restore: bool, gamestate: &mut GameState, sender: &Sender<StackCommand>) {
        const KOMETENEINSCHLAG_NAME: &str = KOMETENEINSCHLAG[0];
        const STROMAUSTFALL_NAME: &str = STROMAUSFALL[0];
        match self.name.as_str() {
            KOMETENEINSCHLAG_NAME => {
                Event::send_popup(&self.popup_message, sender, &self.popup_type, &self.name);
                gamestate.machines.iter_mut().for_each(|machine| {
                    if machine.name == "Loch" {
                        machine.change_state_to(&State::Idle);
                    }
                });
            }
            STROMAUSTFALL_NAME => {
                Event::send_popup(&self.popup_message, sender, &self.popup_type, &self.name);
                gamestate.machines.iter_mut().for_each(|machine| {
                    //Question: set broken or idle?
                    if machine.name == "Stromgenerator" {
                        machine.change_state_to(&State::Broken);
                    }
                });
            }
            (_) => {
                if let Some(resources) = self.resources {
                    if restore {
                        gamestate.player.resources_change =
                            gamestate.player.resources_change + resources;
                    } else {
                        gamestate.player.resources_change =
                            gamestate.player.resources_change - resources;
                    }
                }
            }
        }
        info!("Event triggered (restore: {}): {}", restore, self.name);
    }

    /// Returns the name of the event
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Deletes due events from the gamestates events vector and adds new events
    pub fn update_events(ctx: &Context, gamestate: &mut GameState) {
        if ctx.time.ticks() % 20 == 0 {
            gamestate.events.iter_mut().for_each(|event| {
                event.duration -= 20;
            });

            gamestate.events.retain(|event| {
                if event.is_active() {
                    true
                } else {
                    //TODO: restore resources
                    info!("Event {} is not active anymore", event.get_name());
                    false
                }
            });
        }
        // have a maximum of one active event
        if ctx.time.ticks() % 1000 == 0 {
            // generate new event
            // might not return an event
            let gen_event =
                Event::event_generator(&gamestate.screen_sender.as_ref().unwrap().clone());
            if let Some(event) = gen_event {
                event.action(
                    false,
                    gamestate,
                    &gamestate.screen_sender.as_ref().unwrap().clone(),
                );
                gamestate.events.push(event);
            }
        }
    }
}
