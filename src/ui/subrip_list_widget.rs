use crate::ui::{Drawable, SubripListItem};
use crate::{prelude::*, Subrip};

pub struct SubripListWidget {
    subrips: Vec<Shared<SubripListItem>>,
}

impl Default for SubripListWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl SubripListWidget {
    pub fn new() -> Self {
        Self { subrips: vec![] }
    }

    pub fn add(&mut self, item: &Subrip) {
        let widget = SubripListItem::from(item);
        self.subrips.push(Shared::new(widget));
    }
}

impl From<&Vec<Box<Subrip>>> for SubripListWidget {
    fn from(value: &Vec<Box<Subrip>>) -> Self {
        let mut subrips = vec![];
        for subrip in value.iter() {
            subrips.push(Shared::new(SubripListItem::new(
                subrip.get_content(),
                subrip.get_begin_time().format("%H:%M:%S").to_string(),
                subrip.get_end_time().format("%H:%M:%S").to_string(),
            )));
        }

        Self { subrips }
    }
}

impl Drawable for SubripListWidget {
    // FIXME:
    // Cannot change the sequence of items
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui) {
        let item_count = self.subrips.len();
        eui.separator();
        eui.label(format!("Total: {} subrip(s)", item_count));
        eui.separator();
        egui_dnd::dnd(eui, "Subrips List").show_vec(
            &mut self.subrips,
            |eui, item, handle, _state| {
                // eui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |eui| {
                //     handle.ui(eui, |eui| {
                //         item.borrow_mut().draw(ctx, eui);
                //     });
                // });
                handle.ui(eui, |eui| {
                    item.borrow_mut().draw(ctx, eui);
                });
            },
        );
    }
}
