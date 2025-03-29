use crate::ezfn::Ezfn;
use crate::ui::User Interface;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ezfn = Ezfn::new();
    let items_to_unlock = vec![
        UserInterface::get_input("Enter the ID of the item to unlock:"),
    ];
    let response = ezfn.unlock_items(items_to_unlock).await?;
    if response.success {
        UserInterface::display_message("Items unlocked successfully.");
    } else {
        UserInterface::display_message("Failed to unlock items.");
    }
    Ok(())
}