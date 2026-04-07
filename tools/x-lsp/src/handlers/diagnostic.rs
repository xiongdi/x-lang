//! Diagnostic handler for reporting errors and warnings

use std::sync::Arc;

use anyhow::Result;
use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::{
    notification::{Notification, PublishDiagnostics},
    Diagnostic, DiagnosticSeverity, Position, PublishDiagnosticsParams, Range, Url,
};

use crate::constants::{LANGUAGE_NAME, TYPE_CHECKER_NAME};
use crate::server::LspServer;
use crate::state::{Document, WorkspaceState};
use crate::utils::span_to_range;

/// Generate diagnostics for a document
fn generate_diagnostics(doc: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let content = doc.content();

    // Check for parse errors
    if let Some(err_msg) = doc.parse_error() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 0));
        diagnostics.push(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some(LANGUAGE_NAME.to_string()),
            message: err_msg.to_string(),
            ..Default::default()
        });
    }

    // Add type checker diagnostics
    for type_err in doc.type_errors() {
        let range = if let Some(span) = &type_err.span {
            span_to_range(span, content)
        } else {
            Range::new(Position::new(0, 0), Position::new(0, 0))
        };

        diagnostics.push(Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some(TYPE_CHECKER_NAME.to_string()),
            message: type_err.message.clone(),
            ..Default::default()
        });
    }

    diagnostics
}

/// Publish diagnostics for a document
pub fn publish_diagnostics(server: &LspServer, uri: Url, doc: &Document) -> Result<()> {
    let diagnostics = generate_diagnostics(doc);

    let params = PublishDiagnosticsParams {
        uri,
        diagnostics,
        version: Some(doc.version()),
    };

    let notification =
        lsp_server::Notification::new(PublishDiagnostics::METHOD.to_string(), params);

    server.sender().send(Message::Notification(notification))?;

    Ok(())
}

/// Update and publish diagnostics for a document
pub fn update_diagnostics(
    workspace: &Arc<WorkspaceState>,
    sender: &Sender<Message>,
    uri: &Url,
) -> Result<()> {
    if let Some(doc) = workspace.get_document(uri) {
        let diagnostics = generate_diagnostics(&doc);

        let params = PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics,
            version: Some(doc.version()),
        };

        let notification =
            lsp_server::Notification::new(PublishDiagnostics::METHOD.to_string(), params);

        sender.send(Message::Notification(notification))?;
    }

    Ok(())
}

/// Register diagnostic handler with the server
pub fn register(_server: &mut LspServer) {
    // Diagnostics are automatically published on document changes, no need to register request handler
}
