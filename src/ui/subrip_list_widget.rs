use crate::app::AppState;
use crate::ui::{Drawable, SubripListItem};
use crate::{ai, prelude::*, Subrip};

pub struct SubripListWidget {
    pub sig_subrip_loaded: Signal<Shared<Subrip>>,

    app_state: Shared<AppState>,

    item_widgets: Vec<Shared<SubripListItem>>,
}

impl SubripListWidget {
    pub fn new(app_state: Shared<AppState>) -> Self {
        Self {
            sig_subrip_loaded: Signal::new(),
            app_state,
            item_widgets: vec![],
        }
    }

    pub fn add(&mut self, item: Shared<Subrip>) {
        let widget = SubripListItem::new(item);
        self.item_widgets.push(Shared::new(widget));
    }

    pub fn translate_by_ai(&mut self, _: &()) {
        let translator = ai::AiTranslator::default();
        let app_state = self.app_state.clone();
        let mut borrowed_app_state = app_state.borrow_mut();
        if let Some(path) = borrowed_app_state.file_path_opening.as_ref() {
            let subrips = translator.translate(path.as_path());
            for subrip in subrips.iter() {
                borrowed_app_state.subrips.push(subrip.clone());
                self.add(subrip.clone());
            }
        } else {
            error!("There isn't video selected...");
        }
    }

    pub fn delete_all_subrips(&mut self, _: &()) {
        let mut app_state = self.app_state.borrow_mut();
        app_state.subrips.iter_mut().for_each(|i| {
            i.borrow_mut().delete();
        });
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
