use std::{path::Path, time::Duration};

use anyhow::Context;
use futures::{Stream, StreamExt};

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
    ) -> anyhow::Result<(Self, impl Stream<Item = Result<WatchEvent, WatchError>>)> {
        let (tx, rx) = futures::channel::mpsc::unbounded();

        let watcher = notify::PollWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                _ = tx.unbounded_send(res.map(WatchEvent::Poll).map_err(WatchError::Poll));
            },
            notify::Config::default().with_poll_interval(duration),
        )?;

        let rx = rx.filter(|event| {
            futures::future::ready(matches!(event, Ok(ev) if ev.is_change_event()))
        });

        Ok((Self::Poll(watcher), rx))
    }

    pub fn debounce(
        duration: Duration,
    ) -> anyhow::Result<(Self, impl Stream<Item = Result<WatchEvent, WatchError>>)> {
        let (tx, rx) = futures::channel::mpsc::unbounded();

        let watcher = notify_debouncer_full::new_debouncer(
            duration,
            None,
            move |res: Result<Vec<notify_debouncer_full::DebouncedEvent>, Vec<notify::Error>>| {
                _ = tx.unbounded_send(res.map(WatchEvent::Debounce).map_err(WatchError::Debounce));
            },
        )?;

        let rx = rx.filter(|event| {
            futures::future::ready(matches!(event, Ok(ev) if ev.is_change_event()))
        });

        Ok((Self::Debounce(watcher), rx))
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
