use iced::Task;

use super::super::super::*;

impl Oxide {
    pub(super) fn handle_menu_placement_command(
        &mut self,
        msg: &MenuMessage,
    ) -> Option<Task<Message>> {
        match msg {
            MenuMessage::PlaceWire => {
                self.interaction_state.current_tool = Tool::Wire;
                Some(Task::none())
            }
            MenuMessage::PlaceBus => {
                self.interaction_state.current_tool = Tool::Bus;
                Some(Task::none())
            }
            MenuMessage::PlaceLabel => {
                self.interaction_state.current_tool = Tool::Label;
                Some(Task::none())
            }
            MenuMessage::PlaceComponent => {
                self.interaction_state.current_tool = Tool::Component;
                Some(Task::none())
            }
            MenuMessage::PlacePowerPort => Some(self.set_pending_power_port("GND", "power:GND")),
            MenuMessage::PlaceText => {
                self.interaction_state.current_tool = Tool::Text;
                Some(Task::none())
            }
            MenuMessage::PlaceNoConnect => {
                self.interaction_state.current_tool = Tool::NoConnect;
                Some(Task::none())
            }
            MenuMessage::PlaceSheetEntry => {
                self.interaction_state.current_tool = Tool::Rectangle;
                Some(Task::none())
            }
            MenuMessage::PlaceBusEntry => {
                self.interaction_state.current_tool = Tool::BusEntry;
                Some(Task::none())
            }
            _ => None,
        }
    }
}
