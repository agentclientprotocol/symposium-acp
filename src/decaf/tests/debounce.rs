//! Integration test for the decaf debouncing proxy.
//!
//! Creates a fast word-dumping agent that sends 20 words as individual
//! `AgentMessageChunk` notifications with no delay, runs them through
//! decaf, and verifies the client receives fewer (coalesced) notifications
//! containing all the original text.

use std::path::PathBuf;
use std::time::Duration;

use decaf::Decaf;
use futures::{SinkExt, StreamExt, channel::mpsc};
use sacp::schema::{
    AgentCapabilities, ContentBlock, ContentChunk, InitializeRequest, InitializeResponse,
    NewSessionRequest, NewSessionResponse, PromptRequest, PromptResponse, ProtocolVersion,
    SessionId, SessionNotification, SessionUpdate, StopReason, TextContent,
};
use sacp::{Agent, Client, ConnectTo, ConnectionTo, Responder};
use sacp_conductor::{ConductorImpl, ProxiesAndAgent};
use tokio::io::duplex;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

const WORDS: &[&str] = &[
    "The ",
    "quick ",
    "brown ",
    "fox ",
    "jumps ",
    "over ",
    "the ",
    "lazy ",
    "dog. ",
    "Pack ",
    "my ",
    "box ",
    "with ",
    "five ",
    "dozen ",
    "liquor ",
    "jugs. ",
    "How ",
    "vexingly ",
    "quick ",
];

// ---------------------------------------------------------------------------
// FastWordAgent — sends each word as an individual AgentMessageChunk
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct FastWordAgent;

impl ConnectTo<Client> for FastWordAgent {
    async fn connect_to(self, client: impl ConnectTo<Agent>) -> Result<(), sacp::Error> {
        Agent
            .builder()
            .name("fast-word-agent")
            .on_receive_request(
                async |init: InitializeRequest, responder: Responder<InitializeResponse>, _cx| {
                    responder.respond(
                        InitializeResponse::new(init.protocol_version)
                            .agent_capabilities(AgentCapabilities::new()),
                    )
                },
                sacp::on_receive_request!(),
            )
            .on_receive_request(
                async |_req: NewSessionRequest, responder: Responder<NewSessionResponse>, _cx| {
                    responder.respond(NewSessionResponse::new(SessionId::new("test-session-1")))
                },
                sacp::on_receive_request!(),
            )
            .on_receive_request(
                async |request: PromptRequest,
                       responder: Responder<PromptResponse>,
                       cx: ConnectionTo<Client>| {
                    let cx2 = cx.clone();
                    cx.spawn(async move {
                        let session_id = request.session_id.clone();

                        // Dump all words as fast as possible — no delay
                        for word in WORDS {
                            cx2.send_notification(SessionNotification::new(
                                session_id.clone(),
                                SessionUpdate::AgentMessageChunk(ContentChunk::new(
                                    ContentBlock::Text(TextContent::new(word.to_string())),
                                )),
                            ))?;
                        }

                        responder.respond(PromptResponse::new(StopReason::EndTurn))
                    })
                },
                sacp::on_receive_request!(),
            )
            .connect_to(client)
            .await
    }
}

// ---------------------------------------------------------------------------
// Test helper
// ---------------------------------------------------------------------------

async fn recv<T: sacp::JsonRpcResponse + Send>(
    response: sacp::SentRequest<T>,
) -> Result<T, sacp::Error> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    response.on_receiving_result(async move |result| {
        tx.send(result).map_err(|_| sacp::Error::internal_error())
    })?;
    rx.await.map_err(|_| sacp::Error::internal_error())?
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Verifies that decaf coalesces rapid word-by-word chunks into fewer
/// notifications while preserving all the text content.
#[tokio::test]
async fn test_decaf_coalesces_chunks() -> Result<(), sacp::Error> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_test_writer()
        .try_init();

    // Channel to collect notifications arriving at the client
    let (notif_tx, mut notif_rx) = mpsc::unbounded::<SessionNotification>();

    let (client_write, conductor_read) = duplex(8192);
    let (conductor_write, client_read) = duplex(8192);

    // Spawn conductor: FastWordAgent -> Decaf (100ms) -> client
    let conductor_handle = tokio::spawn(async move {
        ConductorImpl::new_agent(
            "decaf-test-conductor".to_string(),
            ProxiesAndAgent::new(FastWordAgent).proxy(Decaf::new(Duration::from_millis(100))),
            Default::default(),
        )
        .run(sacp::ByteStreams::new(
            conductor_write.compat_write(),
            conductor_read.compat(),
        ))
        .await
    });

    // Run the client, capturing every SessionNotification
    let result = tokio::time::timeout(Duration::from_secs(10), async {
        sacp::Client
            .builder()
            .name("decaf-test-client")
            .on_receive_notification(
                {
                    let mut notif_tx = notif_tx.clone();
                    async move |notification: SessionNotification,
                                _cx: sacp::ConnectionTo<Agent>| {
                        notif_tx
                            .send(notification)
                            .await
                            .map_err(|_| sacp::Error::internal_error())
                    }
                },
                sacp::on_receive_notification!(),
            )
            .connect_with(
                sacp::ByteStreams::new(client_write.compat_write(), client_read.compat()),
                async |cx| {
                    recv(cx.send_request(InitializeRequest::new(ProtocolVersion::LATEST))).await?;

                    let session =
                        recv(cx.send_request(NewSessionRequest::new(PathBuf::from("/")))).await?;

                    let _prompt_response = recv(cx.send_request(PromptRequest::new(
                        session.session_id,
                        vec![ContentBlock::Text(TextContent::new("go".to_string()))],
                    )))
                    .await?;

                    Ok(())
                },
            )
            .await
    })
    .await
    .expect("Test timed out");

    conductor_handle.abort();
    result?;

    // Collect all captured notifications
    drop(notif_tx);
    let mut notifications = Vec::new();
    while let Some(n) = notif_rx.next().await {
        notifications.push(n);
    }

    // Extract text from each notification
    let mut texts: Vec<String> = Vec::new();
    for notif in &notifications {
        if let SessionUpdate::AgentMessageChunk(ContentChunk {
            content: ContentBlock::Text(tc),
            ..
        }) = &notif.update
        {
            texts.push(tc.text.clone());
        }
    }

    let all_text: String = texts.concat();
    let expected: String = WORDS.concat();

    // All text must arrive intact
    assert_eq!(
        all_text, expected,
        "Debounced text should contain all original words"
    );

    // Debouncing should coalesce — fewer notifications than words sent.
    // The agent sends 20 words with no delay, so they should all land
    // within a single 100ms tick (or at most a few ticks).
    assert!(
        texts.len() < WORDS.len(),
        "Expected fewer notifications ({}) than words sent ({}), \
         meaning debouncing coalesced chunks. Individual chunks: {:?}",
        texts.len(),
        WORDS.len(),
        texts,
    );

    tracing::info!(
        words_sent = WORDS.len(),
        notifications_received = texts.len(),
        "Debouncing verified: {} words coalesced into {} notifications",
        WORDS.len(),
        texts.len(),
    );

    Ok(())
}
