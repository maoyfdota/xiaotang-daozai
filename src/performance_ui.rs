//! 性能页指标卡与最近一分钟趋势。

use crate::app::App;
use crate::paint::{Card, H};
use crate::performance::{Reading, Sample};
use crate::theme::{self, W};
use eframe::egui::Color32;
use std::time::{Duration, Instant};

impl App {
    pub(crate) fn page_performance(&mut self, c: &Card) {
        let reading = self.performance.latest();
        let disk = reading.disk_total_rate();
        let network = reading.network_total_rate();
        let cards = [
            (
                "CPU",
                reading
                    .cpu_percent
                    .map(|value| format!("{value:.0}%"))
                    .unwrap_or_else(unavailable),
                "使用率",
            ),
            ("内存", memory_label(reading), "已用 / 总量"),
            (
                "磁盘",
                disk.map(format_rate).unwrap_or_else(unavailable),
                &format!(
                    "读 {}  写 {}",
                    option_rate(reading.disk_read_bytes_per_sec),
                    option_rate(reading.disk_write_bytes_per_sec)
                ),
            ),
            (
                "网络",
                network.map(format_rate).unwrap_or_else(unavailable),
                &format!(
                    "收 {}  发 {}",
                    option_rate(reading.network_receive_bytes_per_sec),
                    option_rate(reading.network_send_bytes_per_sec)
                ),
            ),
        ];
        let positions = [(40.0, 28.0), (219.0, 28.0), (40.0, 84.0), (219.0, 84.0)];
        for ((label, value, detail), (x, y)) in cards.into_iter().zip(positions) {
            metric_card(c, x, y, label, &value, detail);
        }

        c.text(
            43.0,
            145.0,
            "近一分钟趋势",
            10.0,
            W::Semibold,
            theme::GRAY,
            H::Left,
        );
        let bounds = TrendBounds {
            left: 99.0,
            right: 377.0,
            top: 151.0,
            bottom: 211.0,
        };
        c.fill(38.0, 150.0, 344.0, 62.0, 8.0, Color32::from_white_alpha(7));
        let samples: Vec<_> = self.performance.samples().copied().collect();
        draw_trend(c, &samples, bounds, Instant::now());
    }
}

#[derive(Clone, Copy)]
struct TrendBounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

fn metric_card(c: &Card, x: f32, y: f32, label: &str, value: &str, detail: &str) {
    c.fill(x, y, 163.0, 49.0, 9.0, Color32::from_white_alpha(9));
    c.text(
        x + 10.0,
        y + 15.0,
        label,
        10.5,
        W::Semibold,
        theme::GRAY,
        H::Left,
    );
    c.text(
        x + 153.0,
        y + 17.0,
        value,
        14.0,
        W::Bold,
        theme::WHITE,
        H::Right,
    );
    c.text(
        x + 10.0,
        y + 37.0,
        detail,
        9.0,
        W::Regular,
        Color32::from_gray(145),
        H::Left,
    );
}

fn draw_trend(c: &Card, samples: &[Sample], bounds: TrendBounds, now: Instant) {
    let row_height = (bounds.bottom - bounds.top) / 4.0;
    let series: [(&str, Color32, fn(Reading) -> Option<f64>); 4] = [
        ("CPU", Color32::from_rgb(86, 179, 255), |reading| {
            reading.cpu_percent
        }),
        (
            "内存",
            Color32::from_rgb(112, 218, 166),
            Reading::memory_percent,
        ),
        (
            "磁盘",
            Color32::from_rgb(255, 190, 92),
            Reading::disk_total_rate,
        ),
        (
            "网络",
            Color32::from_rgb(204, 146, 255),
            Reading::network_total_rate,
        ),
    ];
    let window_start = now.checked_sub(Duration::from_secs(60)).unwrap_or(now);

    for (row, (label, color, value)) in series.into_iter().enumerate() {
        let top = bounds.top + row as f32 * row_height;
        let center_y = top + row_height * 0.5;
        c.text(45.0, center_y + 3.0, label, 8.5, W::Regular, color, H::Left);
        c.seg(
            (bounds.left, center_y),
            (bounds.right, center_y),
            0.45,
            Color32::from_white_alpha(15),
        );

        let values: Vec<_> = samples
            .iter()
            .map(|sample| {
                value(sample.reading).filter(|number| number.is_finite() && *number >= 0.0)
            })
            .collect();
        let max_value = values
            .iter()
            .flatten()
            .copied()
            .fold(0.0f64, f64::max)
            .max(1.0);
        let points: Vec<_> = samples
            .iter()
            .zip(values.iter())
            .map(|(sample, value)| {
                let x_ratio = sample
                    .at
                    .checked_duration_since(window_start)
                    .map(|elapsed| (elapsed.as_secs_f64() / 60.0).clamp(0.0, 1.0) as f32)
                    .unwrap_or(0.0);
                let y_ratio = value.map(|number| (number / max_value).clamp(0.0, 1.0) as f32);
                (
                    bounds.left + (bounds.right - bounds.left) * x_ratio,
                    y_ratio,
                )
            })
            .collect();

        for pair in points.windows(2) {
            if let (Some(a), Some(b)) = (pair[0].1, pair[1].1) {
                let y1 = top + row_height - 2.0 - a * (row_height - 4.0);
                let y2 = top + row_height - 2.0 - b * (row_height - 4.0);
                c.seg((pair[0].0, y1), (pair[1].0, y2), 1.4, color);
            }
        }
        for (x, value) in points {
            if let Some(value) = value {
                let y = top + row_height - 2.0 - value * (row_height - 4.0);
                c.circle(x, y, 1.2, color);
            }
        }
    }
}

fn memory_label(reading: Reading) -> String {
    match (reading.memory_used_bytes, reading.memory_total_bytes) {
        (Some(used), Some(total)) => format!("{} / {}", format_bytes(used), format_bytes(total)),
        _ => unavailable(),
    }
}

fn option_rate(value: Option<f64>) -> String {
    value.map(format_rate).unwrap_or_else(unavailable)
}

fn unavailable() -> String {
    "不可用".to_owned()
}

fn format_rate(value: f64) -> String {
    format_bytes(value.max(0.0) as u64) + "/s"
}

fn format_bytes(value: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;
    if value >= GIB {
        format!("{:.1} GB", value as f64 / GIB as f64)
    } else if value >= MIB {
        format!("{:.1} MB", value as f64 / MIB as f64)
    } else if value >= KIB {
        format!("{:.1} KB", value as f64 / KIB as f64)
    } else {
        format!("{value} B")
    }
}
