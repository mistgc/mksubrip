use crate::ui::{Drawable, SubripListItem};
use crate::{prelude::*, Subrip};

pub struct SubripListWidget {
    item_widgets: Vec<Shared<SubripListItem>>,
}

impl Default for SubripListWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl SubripListWidget {
    pub fn new() -> Self {
        Self {
            item_widgets: vec![],
        }
    }

    pub fn add(&mut self, item: Rc<RefCell<Subrip>>) {
        let widget = SubripListItem::new(item);
        self.item_widgets.push(Shared::new(widget));
    }
}

impl From<&Vec<Rc<RefCell<Subrip>>>> for SubripListWidget {
    fn from(value: &Vec<Rc<RefCell<Subrip>>>) -> Self {
        let mut ret = Self::new();
        for i in value.iter() {
            ret.add(i.clone());
        }

        ret
    }
}

impl Drawable for SubripListWidget {
    // FIXME:
    // Cannot change the sequence of items
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui) {
        let item_count = self.item_widgets.len();
        eui.separator();
        eui.label(format!("Total: {} subrip(s)", item_count));
        eui.separator();
        egui::ScrollArea::vertical().show(eui, |eui| {
            egui_dnd::dnd(eui, "Subrips List").show_vec(
                &mut self.item_widgets,
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
        });
    }
}