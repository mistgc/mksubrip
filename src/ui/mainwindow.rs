use crate::app::AppState;
use crate::io::{SubripSaveHelper, SubripWriterBuilder};
use crate::prelude::*;
use crate::ui::{self, Drawable};

pub struct MainWindow {
    app_state: Shared<AppState>,

    pub sig_toggle_new_subrip_win: Signal<()>,
    pub sig_toggle_media_play: Signal<()>,

    menu_bar: Shared<ui::MenuBar>,
    new_subrip_win: Shared<ui::NewSubripWindow>,
    subrip_list_widget: Shared<ui::SubripListWidget>,
    timeline: Shared<ui::Timeline>,
    monitor: Shared<ui::Monitor>,
    control_bar: Shared<ui::ControlBar>,

    subrip_save_helper: Shared<SubripSaveHelper>,
}

impl MainWindow {
    pub fn new(app_state: Shared<AppState>) -> Self {
        let mut ret = Self {
            app_state: app_state.clone(),

            sig_toggle_new_subrip_win: Signal::new(),
            sig_toggle_media_play: Signal::new(),

            menu_bar: Shared::new(ui::MenuBar::new()),
            new_subrip_win: Shared::new(ui::NewSubripWindow::new()),
            subrip_list_widget: Shared::new(ui::SubripListWidget::new(app_state.clone())),
            timeline: Shared::new(ui::Timeline::new(app_state.clone())),
            monitor: Shared::new(ui::Monitor::new(app_state.clone())),
            control_bar: Shared::new(ui::ControlBar::new()),

            subrip_save_helper: Shared::new(SubripSaveHelper::new(app_state.clone())),
        };

        ret.init();

        ret
    }

    /// Initialize connections between Signals and Functions
    fn init(&mut self) {
        let save_dir_path_buf = self
            .subrip_save_helper
            .borrow()
            .save_dir_path()
            .to_path_buf();

        if let Some(srt_writer) = SubripWriterBuilder::generate_writer_from_format(
            save_dir_path_buf.as_path(),
            crate::subrip::SubripFormat::SRT,
        ) {
            self.subrip_save_helper.borrow_mut().add_writer(srt_writer);
        }

        self.menu_bar
            .borrow_mut()
            .sig_open_selected
            .connect_func(|path_buf| {
                info!("Selected file path is {}", path_buf.to_str().unwrap());
            });

        self.menu_bar
            .borrow_mut()
            .sig_open_selected
            .connect_method(self.monitor.clone(), ui::Monitor::set_media_path);

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
            .connect_method(self.timeline.clone(), ui::Timeline::add_block_from_subrip);

        self.monitor
            .borrow_mut()
            .sig_media_duration_s_changed
            .connect_method(self.timeline.clone(), ui::Timeline::set_media_duration_s);

        self.timeline
            .borrow_mut()
            .sig_video_seeked
            .connect_method(self.monitor.clone(), ui::Monitor::seek);

        self.sig_toggle_media_play
            .connect_method(self.monitor.clone(), ui::Monitor::play);

        self.monitor
            .borrow_mut()
            .sig_media_loaded
            .connect_method(self.timeline.clone(), ui::Timeline::set_player);

        self.monitor
            .borrow_mut()
            .sig_media_loaded
            .connect_method(self.control_bar.clone(), ui::ControlBar::set_player);

        self.control_bar
            .borrow_mut()
            .sig_btn_play_clicked
            .connect_method(self.monitor.clone(), ui::Monitor::play);

        self.menu_bar
            .borrow_mut()
            .sig_export_srt_selected
            .connect_method(
                self.subrip_save_helper.clone(),
                crate::io::SubripSaveHelper::save,
            );

        self.menu_bar
            .borrow_mut()
            .sig_translate_by_ai_selected
            .connect_method(
                self.subrip_list_widget.clone(),
                ui::SubripListWidget::translate_by_ai,
            );
    }

    /// Poll and handle input events
    fn update_input_event(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter)) {
            self.sig_toggle_new_subrip_win.emit(&());
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.sig_toggle_media_play.emit(&());
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
                self.control_bar.borrow_mut().draw(ctx, eui);
            });

        // monitor area
        egui::CentralPanel::default().show_inside(eui, |eui| {
            eui.heading("c1");
            if self.monitor.borrow().ctx.is_none() {
                self.monitor.borrow_mut().set_ctx(ctx)
            } else {
                self.monitor.borrow_mut().draw(ctx, eui);
            }
        });
    }
}
