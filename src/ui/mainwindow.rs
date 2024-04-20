use crate::app::AppState;
use crate::prelude::*;
use crate::ui::{self, Drawable};

use std::cell::RefCell;
use std::rc::Rc;

pub struct MainWindow {
    app_state: Rc<RefCell<AppState>>,

    pub sig_toggle_new_subrip_win: Signal<()>,

    menu_bar: Shared<ui::MenuBar>,
    new_subrip_win: Shared<ui::NewSubripWindow>,
    subrip_list_widget: Shared<ui::SubripListWidget>,
    timeline: Shared<ui::TimeLine>,
}

impl MainWindow {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        let mut ret = Self {
            app_state: app_state.clone(),

            sig_toggle_new_subrip_win: Signal::new(),

            menu_bar: Shared::new(ui::MenuBar::new()),
            new_subrip_win: Shared::new(ui::NewSubripWindow::new()),
            subrip_list_widget: Shared::new(ui::SubripListWidget::new()),
            timeline: Shared::new(ui::TimeLine::new(app_state.clone())),
        };

        let subrip = Rc::new(RefCell::new(crate::Subrip::new(
            "hello world.",
            chrono::NaiveTime::from_hms_opt(0, 0, 10).unwrap(),
            chrono::TimeDelta::seconds(180),
        )));

        let block = ui::SubripBlock::new(subrip.clone());
        ret.timeline.borrow_mut().add_subrip_block(block);

        ret.init();

        ret
    }

    /// Initialize connections between Signals and Functions
    fn init(&mut self) {
        self.menu_bar
            .borrow_mut()
            .sig_open_selected
            .connect_func(|path_buf| {
                info!("Selected file path is {}", path_buf.to_str().unwrap());
            });

        self.menu_bar
            .borrow_mut()
            .sig_export_srt_selected
            .connect_func(|_| {
                debug!("Selected export srt...");
            });

        let state = self.app_state.clone();
        let subrip_list_widget = self.subrip_list_widget.clone();
        self.new_subrip_win
            .borrow_mut()
            .sig_created_subrip
            .connect_func(move |subrip| {
                state.borrow_mut().subrips.push(subrip.clone());
                subrip_list_widget.borrow_mut().add(subrip.clone());
            });

        self.sig_toggle_new_subrip_win.connect_method(
            self.new_subrip_win.clone(),
            ui::NewSubripWindow::toggle_visible,
        );

        self.subrip_list_widget
            .borrow_mut()
            .sig_subrip_loaded
            .connect_method(self.timeline.clone(), ui::TimeLine::add_block_from_subrip);
    }

    fn update_input_event(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter)) {
            self.sig_toggle_new_subrip_win.emit(&());
        }
    }
}

impl Drawable for MainWindow {
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui) {
        self.update_input_event(ctx);

        self.new_subrip_win.borrow_mut().draw(ctx, eui);

        egui::TopBottomPanel::bottom("b1")
            .resizable(true)
            .min_height(300.0)
            .show_inside(eui, |eui| {
                eui.heading("b1");
                self.timeline.borrow_mut().draw(ctx, eui);
            });

        egui::SidePanel::left("l1")
            .resizable(false)
            .min_width(30.0)
            .show_inside(eui, |eui| {
                eui.heading("l1");
                self.menu_bar.borrow_mut().draw(ctx, eui);
            });

        // subrip(caption) list area
        egui::SidePanel::right("r1")
            .resizable(true)
            .min_width(400.0)
            .show_inside(eui, |eui| {
                eui.heading("r1");
                self.subrip_list_widget.borrow_mut().draw(ctx, eui);
            });

        egui::TopBottomPanel::bottom("b2")
            .resizable(true)
            .min_height(50.0)
            .show_inside(eui, |eui| {
                eui.heading("b2");
            });

        // monitor area
        egui::CentralPanel::default().show_inside(eui, |eui| {
            eui.heading("c1");
        });
    }
}
