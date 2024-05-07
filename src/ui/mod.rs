pub mod control_bar;
pub mod mainwindow;
pub mod menu_bar;
pub mod monitor;
pub mod new_subrip_window;
pub mod subrip_list_item;
pub mod subrip_list_widget;
pub mod timeline;

pub use control_bar::ControlBar;
pub use mainwindow::MainWindow;
pub use menu_bar::MenuBar;
pub use monitor::Monitor;
pub use new_subrip_window::NewSubripWindow;
pub use subrip_list_item::SubripListItem;
pub use subrip_list_widget::SubripListWidget;
pub use timeline::subrip_block::SubripBlock;
pub use timeline::Timeline;

use eframe::egui;

pub trait Drawable {
    fn draw(&mut self, ctx: &egui::Context, eui: &mut egui::Ui);
}
