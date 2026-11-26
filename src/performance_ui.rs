//! 性能页路由与占位视图；系统采样接入后由该页面展示实时指标和趋势。
use crate::app::App;
use crate::paint::{Card, H};
use crate::theme::{self, W};
use eframe::egui::Color32;

impl App {
    pub(crate) fn page_performance(&mut self, c: &Card) {
        let metrics = [
            ("CPU", "—"),
            ("内存", "—"),
            ("磁盘读写", "—"),
            ("网络收发", "—"),
        ];
        let positions = [(38.0, 48.0), (218.0, 48.0), (38.0, 112.0), (218.0, 112.0)];
        for ((label, value), (x, y)) in metrics.into_iter().zip(positions) {
            c.fill(x, y, 163.0, 54.0, 10.0, Color32::from_white_alpha(9));
            c.text(
                x + 12.0,
                y + 19.0,
                label,
                11.0,
                W::Regular,
                theme::GRAY,
                H::Left,
            );
            c.text(
                x + 150.0,
                y + 38.0,
                value,
                17.0,
                W::Bold,
                theme::WHITE,
                H::Right,
            );
        }
        c.text(
            209.5,
            PERFORMANCE_STATUS_Y,
            "系统指标采样接入中",
            10.5,
            W::Regular,
            theme::GRAY,
            H::Center,
        );
    }
}

const PERFORMANCE_STATUS_Y: f32 = 188.0;
