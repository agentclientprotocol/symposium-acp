//! Debouncing proxy for ACP.
//!
//! Agents often send `AgentMessageChunk` notifications word-by-word,
//! creating a flood of tiny messages. Decaf coalesces these chunks,
//! forwarding a single combined chunk every N milliseconds instead.
//!
//! # Usage
//!
//! ```no_run
//! # use decaf::Decaf;
//! # use sacp::{Proxy, ConnectTo};
//! # use std::time::Duration;
//! # async fn example(transport: impl ConnectTo<Proxy> + 'static) -> Result<(), sacp::Error> {
//! Decaf::new(Duration::from_millis(100))
//!     .run(transport)
//!     .await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use sacp::schema::{
    ContentBlock, ContentChunk, PromptRequest, SessionId, SessionNotification, SessionUpdate,
};
use sacp::util::MatchDispatch;
use sacp::{Agent, Client, Conductor, ConnectTo, Dispatch, Proxy};
use tokio::sync::Mutex;

/// A debouncing proxy that coalesces `AgentMessageChunk` notifications.
///
/// Instead of forwarding every individual chunk, Decaf buffers text
/// and flushes it at a configurable interval.
pub struct Decaf {
    interval: Duration,
}

struct BufferedSession {
    /// Accumulated text chunks.
    text: String,

    /// The most recent notification, used as a template when flushing
    /// (preserves session_id, meta, annotations, etc).
    template: SessionNotification,
}

type State = Arc<Mutex<HashMap<SessionId, BufferedSession>>>;

impl Decaf {
    pub fn new(interval: Duration) -> Self {
        Decaf { interval }
    }

    pub async fn run(self, transport: impl ConnectTo<Proxy> + 'static) -> Result<(), sacp::Error> {
        let state: State = Arc::new(Mutex::new(HashMap::new()));
        let interval = self.interval;

        Proxy
            .builder()
            .name("decaf")
            .on_receive_dispatch_from(
                Agent,
                {
                    let state = state.clone();
                    async move |dispatch: Dispatch, cx| {
                        MatchDispatch::new(dispatch)
                            .if_notification(async |notification: SessionNotification| {
                                let is_text_chunk = matches!(
                                    &notification.update,
                                    SessionUpdate::AgentMessageChunk(ContentChunk {
                                        content: ContentBlock::Text(_),
                                        ..
                                    })
                                );

                                if is_text_chunk {
                                    // Buffer the text chunk
                                    let mut sessions = state.lock().await;
                                    let text = match &notification.update {
                                        SessionUpdate::AgentMessageChunk(ContentChunk {
                                            content: ContentBlock::Text(tc),
                                            ..
                                        }) => tc.text.clone(),
                                        _ => unreachable!(),
                                    };

                                    match sessions.get_mut(&notification.session_id) {
                                        Some(buffered) => {
                                            buffered.text.push_str(&text);
                                            buffered.template = notification;
                                        }
                                        None => {
                                            sessions.insert(
                                                notification.session_id.clone(),
                                                BufferedSession {
                                                    text,
                                                    template: notification,
                                                },
                                            );
                                        }
                                    }
                                } else {
                                    // Non-chunk message: flush buffer first, then forward
                                    flush_session(&state, &notification.session_id, &cx).await?;
                                    cx.send_notification_to(Client, notification)?;
                                }

                                Ok(())
                            })
                            .await
                            .if_response_to::<PromptRequest, _>(async |result, router| {
                                // Flush any remaining buffered text before
                                // the prompt response reaches the client.
                                flush_all(&state, &cx).await?;
                                router.respond_with_result(result)
                            })
                            .await
                            .done()
                    }
                },
                sacp::on_receive_dispatch!(),
            )
            .with_spawned({
                let state = state.clone();
                move |cx| async move {
                    let mut ticker = tokio::time::interval(interval);
                    loop {
                        ticker.tick().await;
                        flush_all(&state, &cx).await?;
                    }
                }
            })
            .connect_to(transport)
            .await
    }
}

impl ConnectTo<Conductor> for Decaf {
    async fn connect_to(self, transport: impl ConnectTo<Proxy>) -> Result<(), sacp::Error> {
        self.run(transport).await
    }
}

/// Flush a single session's buffer, sending a coalesced chunk to the client.
async fn flush_session(
    state: &State,
    session_id: &SessionId,
    cx: &sacp::ConnectionTo<Conductor>,
) -> Result<(), sacp::Error> {
    let flushed = {
        let mut sessions = state.lock().await;
        match sessions.get_mut(session_id) {
            Some(buffered) if !buffered.text.is_empty() => {
                let text = std::mem::take(&mut buffered.text);
                let mut notification = buffered.template.clone();

                // Replace the text content with the coalesced text
                if let SessionUpdate::AgentMessageChunk(ContentChunk {
                    content: ContentBlock::Text(tc),
                    ..
                }) = &mut notification.update
                {
                    tc.text = text;
                }

                Some(notification)
            }
            _ => None,
        }
    };

    if let Some(notification) = flushed {
        cx.send_notification_to(Client, notification)?;
    }

    Ok(())
}

/// Flush all sessions that have buffered data.
async fn flush_all(state: &State, cx: &sacp::ConnectionTo<Conductor>) -> Result<(), sacp::Error> {
    // Collect session IDs that need flushing while holding the lock briefly
    let session_ids: Vec<SessionId> = {
        let sessions = state.lock().await;
        sessions
            .iter()
            .filter(|(_, b)| !b.text.is_empty())
            .map(|(id, _)| id.clone())
            .collect()
    };

    for session_id in session_ids {
        flush_session(state, &session_id, cx).await?;
    }

    Ok(())
}
