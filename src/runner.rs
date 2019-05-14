use crate::error::SynFloodError;
use crate::option::Opt;
use log::warn;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub fn run<F, O>(opt: &Opt, mut f: F, output: O) -> Result<RunStatistics, SynFloodError>
where
    F: FnMut() -> Result<(), SynFloodError>,
    O: Fn(RunStatistics) -> Result<(), SynFloodError>,
{
    let mut all_time = RunStatisticsBuilder::new();
    let mut current = all_time.clone();
    let mut last_time: Option<SystemTime> = None;
    loop {
        if current.start.elapsed().map_err(SynFloodError::SystemTime)? >= opt.output_interval {
            output(current.clone().build())?;
            current.clear();
        }
        if let Some(time) = opt.time {
            if all_time
                .start
                .elapsed()
                .map_err(SynFloodError::SystemTime)? >=
                time
            {
                break Ok(all_time.build())
            }
        }
        if let Some(number) = opt.number {
            if all_time.total() == number {
                break Ok(all_time.build())
            }
        }

        if let Some(interval) = opt.interval {
            if let Some(last) = last_time {
                let elapsed = last.elapsed().map_err(SynFloodError::SystemTime)?;
                if elapsed < interval {
                    sleep(interval - elapsed);
                }
            }
        }
        match f() {
            Ok(_) => {
                all_time.success += 1;
                current.success += 1;
            },
            Err(e) => {
                warn!("{}", e);
                all_time.failed += 1;
                current.failed += 1;
            },
        }
        if opt.interval.is_some() {
            last_time = Some(SystemTime::now());
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunStatistics {
    pub start: SystemTime,
    pub end: SystemTime,
    pub success: usize,
    pub failed: usize,
}

impl RunStatistics {
    pub fn total(&self) -> usize {
        self.success + self.failed
    }

    pub fn duration(&self) -> Result<Duration, SynFloodError> {
        self.end
            .duration_since(self.start)
            .map_err(SynFloodError::SystemTime)
    }

    pub fn packet_per_second(&self) -> Result<f64, SynFloodError> {
        self.duration()
            .map(|duration| self.total() as f64 / duration.as_secs_f64())
    }
}

#[derive(Debug, Clone)]
struct RunStatisticsBuilder {
    start: SystemTime,
    success: usize,
    failed: usize,
}

impl RunStatisticsBuilder {
    fn new() -> RunStatisticsBuilder {
        RunStatisticsBuilder {
            start: SystemTime::now(),
            success: 0,
            failed: 0,
        }
    }

    fn clear(&mut self) {
        self.start = SystemTime::now();
        self.success = 0;
        self.failed = 0;
    }

    fn build(self) -> RunStatistics {
        let RunStatisticsBuilder {
            start,
            success,
            failed,
        } = self;
        RunStatistics {
            start,
            end: SystemTime::now(),
            success,
            failed,
        }
    }

    fn total(&self) -> usize {
        self.success + self.failed
    }
}
