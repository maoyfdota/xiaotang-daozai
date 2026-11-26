//! 活跃性能页使用的系统读数、差值计算与短期历史。

use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[cfg(target_os = "macos")]
mod macos;

const HISTORY_WINDOW: Duration = Duration::from_secs(60);
const MAX_SAMPLES: usize = 60;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct CpuCounters {
    pub busy: u64,
    pub total: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ByteCounters {
    pub read: u64,
    pub write: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct MemoryCounters {
    pub used: u64,
    pub total: u64,
}

/// 原生接口的单次累计读数。每个接口独立可用，失败不会被转换成零。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RawSample {
    pub cpu: Option<CpuCounters>,
    pub memory: Option<MemoryCounters>,
    pub disk: Option<ByteCounters>,
    pub network: Option<ByteCounters>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Reading {
    pub cpu_percent: Option<f64>,
    pub memory_used_bytes: Option<u64>,
    pub memory_total_bytes: Option<u64>,
    pub disk_read_bytes_per_sec: Option<f64>,
    pub disk_write_bytes_per_sec: Option<f64>,
    pub network_receive_bytes_per_sec: Option<f64>,
    pub network_send_bytes_per_sec: Option<f64>,
}

impl Reading {
    pub(crate) fn memory_percent(self) -> Option<f64> {
        let used = self.memory_used_bytes?;
        let total = self.memory_total_bytes?;
        (total > 0 && used <= total).then_some(used as f64 * 100.0 / total as f64)
    }

    pub(crate) fn disk_total_rate(self) -> Option<f64> {
        Some(self.disk_read_bytes_per_sec? + self.disk_write_bytes_per_sec?)
    }

    pub(crate) fn network_total_rate(self) -> Option<f64> {
        Some(self.network_receive_bytes_per_sec? + self.network_send_bytes_per_sec?)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Sample {
    pub at: Instant,
    pub reading: Reading,
}

#[derive(Debug, Default)]
pub(crate) struct Sampler {
    last_at: Option<Instant>,
    last_raw: RawSample,
    samples: VecDeque<Sample>,
}

impl Sampler {
    pub(crate) fn due(&self, now: Instant) -> bool {
        self.last_at
            .and_then(|at| now.checked_duration_since(at))
            .is_none_or(|elapsed| elapsed >= Duration::from_secs(1))
    }

    pub(crate) fn ingest(&mut self, now: Instant, raw: RawSample) -> Reading {
        let elapsed = self.last_at.and_then(|at| now.checked_duration_since(at));
        let reading = Reading {
            cpu_percent: elapsed
                .filter(|elapsed| !elapsed.is_zero())
                .and_then(|_| cpu_percent(self.last_raw.cpu, raw.cpu)),
            memory_used_bytes: raw
                .memory
                .filter(|memory| memory.total > 0 && memory.used <= memory.total)
                .map(|memory| memory.used),
            memory_total_bytes: raw
                .memory
                .filter(|memory| memory.total > 0 && memory.used <= memory.total)
                .map(|memory| memory.total),
            disk_read_bytes_per_sec: elapsed
                .and_then(|elapsed| rate(self.last_raw.disk?.read, raw.disk?.read, elapsed)),
            disk_write_bytes_per_sec: elapsed
                .and_then(|elapsed| rate(self.last_raw.disk?.write, raw.disk?.write, elapsed)),
            network_receive_bytes_per_sec: elapsed
                .and_then(|elapsed| rate(self.last_raw.network?.read, raw.network?.read, elapsed)),
            network_send_bytes_per_sec: elapsed.and_then(|elapsed| {
                rate(self.last_raw.network?.write, raw.network?.write, elapsed)
            }),
        };

        self.last_at = Some(now);
        self.last_raw = raw;
        self.samples.push_back(Sample { at: now, reading });
        while self.samples.front().is_some_and(|sample| {
            now.checked_duration_since(sample.at)
                .is_none_or(|age| age > HISTORY_WINDOW)
        }) {
            self.samples.pop_front();
        }
        while self.samples.len() > MAX_SAMPLES {
            self.samples.pop_front();
        }
        reading
    }

    pub(crate) fn latest(&self) -> Reading {
        self.samples
            .back()
            .map(|sample| sample.reading)
            .unwrap_or_default()
    }

    pub(crate) fn samples(&self) -> impl Iterator<Item = &Sample> {
        self.samples.iter()
    }
}

fn cpu_percent(previous: Option<CpuCounters>, current: Option<CpuCounters>) -> Option<f64> {
    let previous = previous?;
    let current = current?;
    let busy = current.busy.checked_sub(previous.busy)?;
    let total = current.total.checked_sub(previous.total)?;
    if total == 0 || busy > total {
        return None;
    }
    Some((busy as f64 * 100.0 / total as f64).clamp(0.0, 100.0))
}

fn rate(previous: u64, current: u64, elapsed: Duration) -> Option<f64> {
    let seconds = elapsed.as_secs_f64();
    if seconds <= 0.0 {
        return None;
    }
    Some(current.checked_sub(previous)? as f64 / seconds)
}

#[cfg(target_os = "macos")]
pub(crate) fn collect() -> RawSample {
    macos::collect()
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn collect() -> RawSample {
    RawSample::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(
        cpu: u64,
        disk_read: u64,
        disk_write: u64,
        network_in: u64,
        network_out: u64,
    ) -> RawSample {
        RawSample {
            cpu: Some(CpuCounters {
                busy: cpu,
                total: cpu + 100,
            }),
            memory: Some(MemoryCounters {
                used: 4_000,
                total: 8_000,
            }),
            disk: Some(ByteCounters {
                read: disk_read,
                write: disk_write,
            }),
            network: Some(ByteCounters {
                read: network_in,
                write: network_out,
            }),
        }
    }

    #[test]
    fn cumulative_counter_deltas_use_the_actual_sampling_interval() {
        let start = Instant::now();
        let mut sampler = Sampler::default();
        let mut baseline = raw(10, 100, 200, 1_000, 2_000);
        baseline.cpu = Some(CpuCounters {
            busy: 100,
            total: 200,
        });
        assert!(sampler
            .ingest(start, baseline)
            .disk_read_bytes_per_sec
            .is_none());

        let mut current = raw(70, 300, 600, 2_000, 6_000);
        current.cpu = Some(CpuCounters {
            busy: 160,
            total: 360,
        });
        let reading = sampler.ingest(start + Duration::from_secs(2), current);
        assert_eq!(reading.cpu_percent, Some(37.5));
        assert_eq!(reading.disk_read_bytes_per_sec, Some(100.0));
        assert_eq!(reading.disk_write_bytes_per_sec, Some(200.0));
        assert_eq!(reading.network_receive_bytes_per_sec, Some(500.0));
        assert_eq!(reading.network_send_bytes_per_sec, Some(2_000.0));
    }

    #[test]
    fn first_samples_missing_sources_and_counter_resets_remain_unavailable() {
        let start = Instant::now();
        let mut sampler = Sampler::default();
        let first = sampler.ingest(start, raw(10, 100, 200, 1_000, 2_000));
        assert_eq!(first.cpu_percent, None);
        assert_eq!(first.disk_read_bytes_per_sec, None);
        assert_eq!(first.network_receive_bytes_per_sec, None);

        let missing = sampler.ingest(start + Duration::from_secs(1), RawSample::default());
        assert_eq!(missing.cpu_percent, None);
        assert_eq!(missing.memory_used_bytes, None);
        assert_eq!(missing.disk_read_bytes_per_sec, None);

        let after_missing = sampler.ingest(
            start + Duration::from_secs(2),
            raw(20, 150, 250, 1_100, 2_100),
        );
        assert_eq!(
            after_missing.disk_read_bytes_per_sec, None,
            "缺少中间样本后需要重新建立基线"
        );

        let reset = sampler.ingest(start + Duration::from_secs(3), raw(2, 10, 10, 10, 10));
        assert_eq!(reset.cpu_percent, None);
        assert_eq!(reset.disk_read_bytes_per_sec, None);
        assert_eq!(reset.network_receive_bytes_per_sec, None);
    }

    #[test]
    fn invalid_cpu_deltas_and_memory_values_are_not_fabricated() {
        let start = Instant::now();
        let mut sampler = Sampler::default();
        let first = RawSample {
            cpu: Some(CpuCounters { busy: 4, total: 4 }),
            memory: Some(MemoryCounters { used: 9, total: 8 }),
            ..Default::default()
        };
        sampler.ingest(start, first);
        let next = RawSample {
            cpu: Some(CpuCounters { busy: 4, total: 4 }),
            memory: Some(MemoryCounters { used: 1, total: 0 }),
            ..Default::default()
        };
        let reading = sampler.ingest(start + Duration::from_secs(1), next);
        assert_eq!(reading.cpu_percent, None, "没有 CPU 总增量无法计算利用率");
        assert_eq!(reading.memory_used_bytes, None);
        assert_eq!(reading.memory_total_bytes, None);
    }

    #[test]
    fn cpu_and_rate_values_need_a_positive_sampling_interval() {
        let start = Instant::now();
        let mut sampler = Sampler::default();
        sampler.ingest(start, raw(10, 100, 200, 1_000, 2_000));

        let reading = sampler.ingest(start, raw(20, 200, 300, 2_000, 3_000));
        assert_eq!(reading.cpu_percent, None);
        assert_eq!(reading.disk_read_bytes_per_sec, None);
        assert_eq!(reading.network_receive_bytes_per_sec, None);
    }

    #[test]
    fn trend_keeps_unavailable_gaps_and_only_retains_about_one_minute() {
        let start = Instant::now();
        let mut sampler = Sampler::default();
        sampler.ingest(start, raw(10, 100, 200, 1_000, 2_000));
        sampler.ingest(start + Duration::from_secs(1), RawSample::default());
        sampler.ingest(
            start + Duration::from_secs(2),
            raw(20, 200, 300, 2_000, 3_000),
        );
        let samples: Vec<_> = sampler.samples().copied().collect();
        assert_eq!(samples.len(), 3);
        assert!(
            samples[1].reading.cpu_percent.is_none(),
            "缺失值在趋势中保留为断点"
        );

        for second in 3..=65 {
            sampler.ingest(
                start + Duration::from_secs(second),
                raw(
                    second,
                    second * 100,
                    second * 200,
                    second * 500,
                    second * 700,
                ),
            );
        }
        let samples: Vec<_> = sampler.samples().copied().collect();
        assert!(samples.len() <= MAX_SAMPLES);
        assert!(samples.last().unwrap().at.duration_since(samples[0].at) <= HISTORY_WINDOW);
    }
}
