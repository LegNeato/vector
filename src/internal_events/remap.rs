use super::InternalEvent;
use metrics::counter;

#[derive(Debug)]
pub struct RemapFailedMapping {
    /// If set to true, the remap transform has dropped the event after a failed
    /// mapping. This internal event will reflect that in its messaging.
    pub event_dropped: bool,
    pub error: String,
}

impl InternalEvent for RemapFailedMapping {
    fn emit_logs(&self) {
        let message = if self.event_dropped {
            "Mapping failed with event; discarding event."
        } else {
            "Mapping failed with event."
        };

        warn!(
            message,
            error = ?self.error,
            rate_limit_secs = 30
        )
    }

    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1, "error_type" => "failed_mapping");
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RemapConditionExecutionFailed;

impl InternalEvent for RemapConditionExecutionFailed {
    fn emit_logs(&self) {
        warn!(
            message = "Remap condition execution failed.",
            rate_limit_secs = 120
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RemapConditionNonBooleanReturned;

impl InternalEvent for RemapConditionNonBooleanReturned {
    fn emit_logs(&self) {
        warn!(
            message = "Remap condition non-boolean value returned.",
            rate_limit_secs = 120
        )
    }
}
