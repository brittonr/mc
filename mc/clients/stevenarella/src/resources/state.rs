use std::io;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};

use crate::ui;

const PROGRESS_EMPTY: f64 = 0.0;
const PROGRESS_COMPLETE: f64 = 1.0;

pub type SharedManager = Arc<RwLock<Manager>>;
pub(crate) type SharedProgress = Arc<Mutex<Progress>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceBoundaryError {
    ProgressPoisoned,
}

pub trait Pack: Sync + Send {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>>;
}

pub struct Manager {
    pub(crate) packs: Vec<Box<dyn Pack>>,
    pub(crate) version: usize,
    pub(crate) vanilla_progress: SharedProgress,
}

pub struct ManagerUI {
    pub(crate) progress_ui: Vec<ProgressUI>,
    pub(crate) num_tasks: isize,
    pub(crate) vanilla_chan: Option<std::sync::mpsc::Receiver<bool>>,
    pub(crate) vanilla_assets_chan: Option<std::sync::mpsc::Receiver<bool>>,
}

pub(crate) struct ProgressUI {
    pub(crate) task_name: String,
    pub(crate) task_file: String,
    pub(crate) position: f64,
    pub(crate) closing: bool,
    pub(crate) progress: f64,
    pub(crate) background: ui::ImageRef,
    pub(crate) progress_bar: ui::ImageRef,
}

pub(crate) struct Progress {
    pub(crate) tasks: Vec<Task>,
}

pub(crate) struct Task {
    pub(crate) task_name: String,
    pub(crate) task_file: String,
    pub(crate) total: u64,
    pub(crate) progress: u64,
}

pub(crate) fn new_progress() -> SharedProgress {
    Arc::new(Mutex::new(Progress { tasks: Vec::new() }))
}

pub(crate) fn task_progress_ratio(progress: u64, total: u64) -> f64 {
    if total == 0 {
        return PROGRESS_COMPLETE;
    }
    ((progress as f64) / (total as f64)).clamp(PROGRESS_EMPTY, PROGRESS_COMPLETE)
}

pub(crate) fn lock_progress(
    progress: &SharedProgress,
) -> Result<MutexGuard<'_, Progress>, ResourceBoundaryError> {
    progress
        .lock()
        .map_err(|_| ResourceBoundaryError::ProgressPoisoned)
}

impl Manager {
    /// Returns the 'version' of the manager. The version is
    /// increase everytime a pack is added or removed.
    pub fn version(&self) -> usize {
        self.version
    }

    pub(crate) fn add_pack(&mut self, pck: Box<dyn Pack>) {
        self.packs.push(pck);
        self.version += 1;
    }

    pub(crate) fn add_task(progress: &SharedProgress, name: &str, file: &str, length: u64) {
        let _ = Self::try_add_task(progress, name, file, length);
    }

    pub(crate) fn try_add_task(
        progress: &SharedProgress,
        name: &str,
        file: &str,
        length: u64,
    ) -> Result<(), ResourceBoundaryError> {
        let mut info = lock_progress(progress)?;
        info.tasks.push(Task {
            task_name: name.into(),
            task_file: file.into(),
            total: length,
            progress: 0,
        });
        Ok(())
    }

    pub(crate) fn add_task_progress(progress: &SharedProgress, name: &str, file: &str, prog: u64) {
        let _ = Self::try_add_task_progress(progress, name, file, prog);
    }

    pub(crate) fn try_add_task_progress(
        progress: &SharedProgress,
        name: &str,
        file: &str,
        prog: u64,
    ) -> Result<(), ResourceBoundaryError> {
        let mut progress = lock_progress(progress)?;
        for task in progress
            .tasks
            .iter_mut()
            .filter(|task| task.task_file == file)
            .filter(|task| task.task_name == name)
        {
            task.progress += prog;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TASK_NAME: &str = "test task";
    const TEST_TASK_FILE: &str = "test file";
    const TEST_TASK_TOTAL: u64 = 10;
    const TEST_TASK_PROGRESS: u64 = 4;
    const TEST_TASK_RATIO: f64 = 0.4;

    #[test]
    fn progress_updates_record_task_progress() {
        let progress = new_progress();

        Manager::try_add_task(&progress, TEST_TASK_NAME, TEST_TASK_FILE, TEST_TASK_TOTAL).unwrap();
        Manager::try_add_task_progress(
            &progress,
            TEST_TASK_NAME,
            TEST_TASK_FILE,
            TEST_TASK_PROGRESS,
        )
        .unwrap();
        let progress = lock_progress(&progress).unwrap();

        assert_eq!(progress.tasks.len(), 1);
        assert_eq!(progress.tasks[0].progress, TEST_TASK_PROGRESS);
        assert_eq!(
            task_progress_ratio(progress.tasks[0].progress, progress.tasks[0].total),
            TEST_TASK_RATIO
        );
    }

    #[test]
    fn progress_updates_fail_closed_after_poison() {
        let progress = new_progress();
        let poison_target = progress.clone();
        let _ = std::panic::catch_unwind(move || {
            let _guard = poison_target.lock().unwrap();
            panic!("poison progress mutex");
        });

        assert_eq!(
            Manager::try_add_task(&progress, TEST_TASK_NAME, TEST_TASK_FILE, TEST_TASK_TOTAL),
            Err(ResourceBoundaryError::ProgressPoisoned)
        );
        assert_eq!(
            Manager::try_add_task_progress(
                &progress,
                TEST_TASK_NAME,
                TEST_TASK_FILE,
                TEST_TASK_PROGRESS,
            ),
            Err(ResourceBoundaryError::ProgressPoisoned)
        );
    }

    #[test]
    fn zero_length_progress_reports_complete_without_nan() {
        assert_eq!(
            task_progress_ratio(TEST_TASK_PROGRESS, 0),
            PROGRESS_COMPLETE
        );
    }
}
