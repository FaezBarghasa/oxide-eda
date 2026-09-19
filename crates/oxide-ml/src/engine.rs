use tract_onnx::prelude::*;

use crate::embed::{MlError, ModelKind};

pub type TractTypedPlan =
    tract_onnx::tract_core::plan::SimplePlan<TypedFact, Box<dyn TypedOp>>;

/// Wrapper around a runnable tract ONNX model plan
pub struct InferenceEngine {
    plan: Arc<TractTypedPlan>,
    kind: ModelKind,
}

impl InferenceEngine {
    /// Load an ONNX model from raw byte slice and optimize graph for inference
    pub fn from_bytes(kind: ModelKind, model_bytes: &[u8]) -> Result<Self, MlError> {
        let size_mb = model_bytes.len().div_ceil(1024 * 1024);
        if size_mb > kind.max_size_mb() {
            return Err(MlError::ModelTooLarge {
                kind,
                size_mb,
                max_mb: kind.max_size_mb(),
            });
        }

        let model = tract_onnx::onnx()
            .model_for_read(&mut &model_bytes[..])
            .map_err(|e| MlError::ModelLoadFailed(e.to_string()))?;

        let optimized = model
            .into_optimized()
            .map_err(|e| MlError::OptimizationFailed(e.to_string()))?;

        let plan = optimized
            .into_runnable()
            .map_err(|e| MlError::PlanFailed(e.to_string()))?;

        Ok(Self { plan, kind })
    }

    pub fn kind(&self) -> ModelKind {
        self.kind
    }

    /// Run inference with input tensors
    pub fn run(&self, inputs: TVec<Tensor>) -> Result<TVec<Arc<Tensor>>, MlError> {
        let tvalues: TVec<TValue> = inputs.into_iter().map(TValue::from).collect();
        let results = self
            .plan
            .run(tvalues)
            .map_err(|e| MlError::InferenceFailed(e.to_string()))?;

        Ok(results.into_iter().map(|tv| tv.into_arc_tensor()).collect())
    }
}
