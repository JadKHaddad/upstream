use std::{fmt::Display, path::Path, time::Duration};

use anyhow::Context;
use futures::channel::mpsc::UnboundedSender;

pub enum Watcher {
    Poll(notify::PollWatcher),
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Debounce(
        notify_debouncer_full::Debouncer<notify::INotifyWatcher, notify_debouncer_full::NoCache>,
    ),
    #[cfg(target_os = "windows")]
    Debounce(
        notify_debouncer_full::Debouncer<
            notify::ReadDirectoryChangesWatcher,
            notify_debouncer_full::FileIdMap,
        >,
    ),
}

#[derive(Debug)]
pub enum WatchEvent {
    Poll(notify::Event),
    Debounce(Vec<notify_debouncer_full::DebouncedEvent>),
}

#[derive(Debug)]
pub enum WatchError {
    Poll(notify::Error),
    Debounce(Vec<notify::Error>),
}

impl Display for WatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Poll(err) => write!(f, "Poll watch error: {}", err),
            Self::Debounce(errs) => {
                for err in errs {
                    writeln!(f, "Debounce watch error: {}", err)?;
                }
                Ok(())
            }
        }
    }
}

impl core::error::Error for WatchError {}

impl WatchEvent {
    const fn _is_change_event(event: &notify::Event) -> bool {
        matches!(
            event.kind,
            notify::EventKind::Modify(_)
                | notify::EventKind::Create(_)
                | notify::EventKind::Remove(_)
        )
    }

    fn is_change_event(&self) -> bool {
        match self {
            Self::Poll(ev) => Self::_is_change_event(ev),
            Self::Debounce(events) => events.iter().any(|ev| Self::_is_change_event(ev)),
        }
    }
}

impl Watcher {
    pub fn poll(
        duration: Duration,
        tx: UnboundedSender<Result<WatchEvent, WatchError>>,
    ) -> anyhow::Result<Self> {
        let watcher = notify::PollWatcher::new(
            move |res: Result<notify::Event, notify::Error>| match res
                .map(WatchEvent::Poll)
                .map_err(WatchError::Poll)
            {
                Ok(ev) if !ev.is_change_event() => {}
                res => {
                    _ = tx.unbounded_send(res);
                }
            },
            notify::Config::default().with_poll_interval(duration),
        )?;

        Ok(Self::Poll(watcher))
    }

    pub fn debounce(
        duration: Duration,
        tx: UnboundedSender<Result<WatchEvent, WatchError>>,
    ) -> anyhow::Result<Self> {
        let watcher = notify_debouncer_full::new_debouncer(
            duration,
            None,
            move |res: Result<Vec<notify_debouncer_full::DebouncedEvent>, Vec<notify::Error>>| {
                match res.map(WatchEvent::Debounce).map_err(WatchError::Debounce) {
                    Ok(ev) if !ev.is_change_event() => {}
                    res => {
                        _ = tx.unbounded_send(res);
                    }
                }
            },
        )?;

        Ok(Self::Debounce(watcher))
    }

    fn watch(&mut self, path: &Path) -> notify::Result<()> {
        use notify::Watcher;

        match self {
            Self::Poll(watcher) => watcher.watch(path, notify::RecursiveMode::Recursive),
            Self::Debounce(watcher) => watcher.watch(path, notify::RecursiveMode::Recursive),
        }
    }

    pub fn watch_many<I>(&mut self, paths: I) -> anyhow::Result<()>
    where
        I: IntoIterator,
        I::Item: AsRef<Path>,
    {
        for path in paths {
            self.watch(path.as_ref())
                .context(format!("Failed to watch path: {:?}", path.as_ref()))?;
        }

        Ok(())
    }
}
