use serde::{Deserialize, Serialize};
/// Defines an item in the inventory of the player
/// Contains the name of the item, information about the item and the image
// Defines an Item in the inventory of the player
#[derive(Clone, Default, Eq, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Item {
    name: String,
    info_text: String,
    //image should be a texture, didnt work yet
    img: String,
    amount: i16,
}
impl Item {
    fn new(name: String, info_text: String, img: String, amount: i16) -> Self {
        Self {
            name,
            info_text,
            img,
            amount,
        }
    }
    pub fn change_amount(&mut self, amount: i16) {
        self.amount += amount;
    }
}
