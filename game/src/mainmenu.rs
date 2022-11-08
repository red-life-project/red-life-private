use crate::gamestate::GameState;
use crate::screen::{Screen, StackCommand};
use crate::utils::get_scale;
use crate::RLResult;
use ggez::event::MouseButton;
use ggez::{graphics, Context};
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Exit,
    NewGame,
    Start,
}

#[derive(Debug)]
struct Button<Message: Clone> {
    text: String,
    img: graphics::Image,
    message: Message,
    sender: Sender<Message>,
}
#[derive(Debug)]
pub struct MainMenu<Message: Clone> {
    buttons: Vec<Button<Message>>,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
}

impl<Message: Clone> Default for MainMenu<Message> {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            buttons: vec![],
            receiver,
            sender,
        }
    }
}

impl Screen for MainMenu<Message> {
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        // TODO: Replace with if buttons are clicked
        if ctx.mouse.button_pressed(MouseButton::Left) {
            return Ok(StackCommand::Push(Box::new(
                GameState::load_game_state().unwrap_or_default(),
            )));
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../assets/mainmenu.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        canvas.finish(ctx)?;
        Ok(())
    }
}
