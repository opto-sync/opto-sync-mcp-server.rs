//! ORES `next-loggers/v1` lifecycle records for the MCP process.
//!
//! Stdout is the MCP JSON-RPC wire, so records are written to stderr only.
//! Only constant event names and static tags enter records: no tool
//! arguments, results, error bodies, URLs, credentials, users, tenants,
//! requests, sessions, or source payloads.
//!
//! Builders returned here are not sent: each call site attaches its own static
//! `ores-trace-` literal and the enclosing function's `ores-routine-` constant
//! before calling `send()`.

use std::sync::Arc;

use next_loggers::{
    Event, JsonObject, Logger, LoggerError, OpenTelemetryTransport, Options, Value, json,
};

/// Build a logger that writes OpenTelemetry-shaped JSON records to stderr.
#[must_use]
pub fn logger() -> Logger {
    let transport = Arc::new(OpenTelemetryTransport::new(|record| {
        let encoded = serde_json::to_string(&record)
            .map_err(|error| LoggerError(format!("cannot encode OTEL log record: {error}")))?;
        eprintln!("{encoded}");
        Ok(())
    }));
    let mut options = Options::default().with_transport(transport);
    options.app_name = env!("CARGO_PKG_NAME").into();
    options.name = Some("mcp-runtime".into());
    options.console = false;
    Logger::new(options)
}

/// Build an informational lifecycle event carrying only a constant name.
#[must_use]
pub fn event(logger: &Logger, name: &'static str) -> Event {
    metadata_free(logger.info(vec![Value::String(name.into())]), name)
}

/// Build an error lifecycle event carrying only a constant name.
///
/// The underlying error is intentionally not attached: configuration and
/// transport errors can embed argument values or peer details.
#[must_use]
pub fn failure(logger: &Logger, name: &'static str) -> Event {
    metadata_free(logger.error(vec![Value::String(name.into())]), name)
}

fn metadata_free(event: Event, name: &'static str) -> Event {
    event
        .add_fields(JsonObject::from_iter([
            ("event.name".into(), json!(name)),
            ("data.classification".into(), json!("metadata-free")),
        ]))
        .add_tags(["opto-sync", "mcp"])
}

#[cfg(test)]
mod tests {
    use super::{event, failure, logger};

    #[test]
    fn records_carry_only_constant_metadata_free_fields() {
        let logger = logger();
        let record = event(&logger, "mcp.test")
            .send()
            .expect("send succeeds")
            .expect("record is emitted");
        let encoded = record
            .to_json()
            .expect("record encodes")
            .to_ascii_lowercase();
        assert!(encoded.contains("mcp.test"));
        assert!(encoded.contains("metadata-free"));
        for forbidden in ["token", "secret", "password", "https://", "session"] {
            assert!(
                !encoded.contains(forbidden),
                "{forbidden} entered telemetry"
            );
        }
        let _ = failure(&logger, "mcp.test.failed").send();
        logger.close().expect("logger closes");
    }
}
