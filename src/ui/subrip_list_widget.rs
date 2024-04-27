use crate::ui::{Drawable, SubripListItem};
use crate::{prelude::*, Subrip};

pub struct SubripListWidget {
    pub sig_subrip_loaded: Signal<Rc<RefCell<Subrip>>>,

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
            sig_subrip_loaded: Signal::new(),
            item_widgets: vec![],
        }
    }

    pub fn add(&mut self, item: Shared<Subrip>) {
        let widget = SubripListItem::new(item);
        self.item_widgets.push(Shared::new(widget));
    }
}

impl From<&Vec<Shared<Subrip>>> for SubripListWidget {
    fn from(value: &Vec<Shared<Subrip>>) -> Self {
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
        let mut pos = eui.cursor().min;
        pos.x += eui.available_width();
        pos.y += eui.available_height();
        let item_count = self.item_widgets.len();
        eui.separator();
        eui.label(format!("Total: {} subrip(s)", item_count));
        eui.separator();
        egui::ScrollArea::vertical().show(eui, |eui| {
            egui_dnd::dnd(eui, "Subrips List").show_vec(
                &mut self.item_widgets,
                |eui, item, handle, state| {
                    if state.dragged {
                        if let Some(pointer_pos) = ctx.pointer_latest_pos() {
                            if ctx.input(
                                |i| { i.pointer.primary_released() } && pointer_pos.y > pos.y,
                            ) {
                                item.borrow_mut().subrip.borrow_mut().set_loading(true);
                                self.sig_subrip_loaded.emit(&item.borrow_mut().subrip);
                            }
                        }
                    }
                    handle.ui(eui, |eui| {
                        item.borrow_mut().draw(ctx, eui);
                    });
                },
            );
        });
    }
}
