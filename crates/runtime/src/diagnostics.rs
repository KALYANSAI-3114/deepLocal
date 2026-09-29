use deeplocal_core::{ModelLoadDiagnostic, ModelLoadFailureKind};

pub fn diagnose_model_load_error(raw_error: &str) -> ModelLoadDiagnostic {
    let lower = raw_error.to_lowercase();

    // 1. Missing or unreadable file
    if lower.contains("no such file")
        || lower.contains("file not found")
        || lower.contains("cannot be read")
        || lower.contains("permission denied")
        || lower.contains("failed to open file")
        || lower.contains("cannot open")
    {
        return ModelLoadDiagnostic {
            category: ModelLoadFailureKind::FileNotFoundOrUnreadable,
            title: "Model File Missing or Unreadable".to_string(),
            explanation: "The requested model file does not exist at the specified path or the system lacks read permissions.".to_string(),
            recovery_step: "Verify the model file path, check system file permissions, or re-link the file in settings.".to_string(),
            technical_details: raw_error.to_string(),
        };
    }

    // 2. Memory / Out of Memory (RAM / VRAM)
    if lower.contains("out of memory")
        || lower.contains("oom")
        || lower.contains("insufficient memory")
        || lower.contains("cuda out of memory")
        || lower.contains("failed to allocate")
        || lower.contains("cannot allocate memory")
        || lower.contains("not enough memory")
    {
        return ModelLoadDiagnostic {
            category: ModelLoadFailureKind::InsufficientMemory,
            title: "Insufficient Memory".to_string(),
            explanation: "Available system RAM or GPU VRAM appears insufficient for the requested model and context options.".to_string(),
            recovery_step: "Reduce context length, decrease GPU offload layers, or select a smaller quantization size.".to_string(),
            technical_details: raw_error.to_string(),
        };
    }

    // 3. GPU Backend or device unavailable
    if lower.contains("gpu backend")
        || lower.contains("cuda driver")
        || lower.contains("metal not supported")
        || lower.contains("vulkan device")
        || lower.contains("no suitable gpu")
        || lower.contains("device is unavailable")
        || lower.contains("rocm not found")
    {
        return ModelLoadDiagnostic {
            category: ModelLoadFailureKind::GpuBackendUnavailable,
            title: "GPU Backend Unavailable".to_string(),
            explanation: "The requested GPU accelerator or backend runtime could not be initialized.".to_string(),
            recovery_step: "Switch the inference backend to CPU or check your GPU driver and runtime configuration.".to_string(),
            technical_details: raw_error.to_string(),
        };
    }

    // 4. Unsupported model architecture
    if lower.contains("unsupported architecture")
        || lower.contains("unknown model architecture")
        || lower.contains("unsupported tensor type")
    {
        return ModelLoadDiagnostic {
            category: ModelLoadFailureKind::UnsupportedArchitecture,
            title: "Unsupported Model Architecture".to_string(),
            explanation: "The model architecture or tensor structure is not supported by the active engine version.".to_string(),
            recovery_step: "Verify the GGUF model format version or update the inference runtime to a compatible build.".to_string(),
            technical_details: raw_error.to_string(),
        };
    }

    // 5. Incompatible or missing runtime
    if lower.contains("runtime missing")
        || lower.contains("incompatible runtime")
        || lower.contains("backend not registered")
        || lower.contains("binary not found")
        || lower.contains("shared library")
    {
        return ModelLoadDiagnostic {
            category: ModelLoadFailureKind::RuntimeIncompatibleOrMissing,
            title: "Runtime Incompatible or Missing".to_string(),
            explanation: "The specified inference runtime executable or shared library could not be located or executed.".to_string(),
            recovery_step: "Choose another installed runtime backend or inspect prerequisite system dependencies.".to_string(),
            technical_details: raw_error.to_string(),
        };
    }

    // Fallback: Unknown (requirement: preserve technical details, do not speculate)
    ModelLoadDiagnostic {
        category: ModelLoadFailureKind::Unknown,
        title: "Model Load Failure".to_string(),
        explanation: "The model failed to load due to an undetermined error.".to_string(),
        recovery_step: "Review the technical details below and consult the troubleshooting guide."
            .to_string(),
        technical_details: raw_error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnose_file_not_found() {
        let err = "Failed to open file: /models/test.gguf: No such file or directory";
        let diag = diagnose_model_load_error(err);
        assert_eq!(
            diag.category,
            ModelLoadFailureKind::FileNotFoundOrUnreadable
        );
        assert_eq!(diag.technical_details, err);
    }

    #[test]
    fn test_diagnose_insufficient_memory() {
        let err = "llama_model_load: out of memory allocating context buffer";
        let diag = diagnose_model_load_error(err);
        assert_eq!(diag.category, ModelLoadFailureKind::InsufficientMemory);
    }

    #[test]
    fn test_diagnose_gpu_unavailable() {
        let err = "CUDA error: no suitable gpu device is unavailable";
        let diag = diagnose_model_load_error(err);
        assert_eq!(diag.category, ModelLoadFailureKind::GpuBackendUnavailable);
    }

    #[test]
    fn test_diagnose_unsupported_arch() {
        let err = "unsupported architecture: llama-moe-v99";
        let diag = diagnose_model_load_error(err);
        assert_eq!(diag.category, ModelLoadFailureKind::UnsupportedArchitecture);
    }

    #[test]
    fn test_diagnose_runtime_missing() {
        let err = "backend not registered: llama.cpp";
        let diag = diagnose_model_load_error(err);
        assert_eq!(
            diag.category,
            ModelLoadFailureKind::RuntimeIncompatibleOrMissing
        );
    }

    #[test]
    fn test_diagnose_unknown_fallback() {
        let err = "unexpected internal assertion failed at line 42";
        let diag = diagnose_model_load_error(err);
        assert_eq!(diag.category, ModelLoadFailureKind::Unknown);
        assert_eq!(diag.technical_details, err);
    }

    #[test]
    fn malformed_or_corrupt_model_errors_are_not_claimed_as_architecture_errors() {
        for err in [
            "magic number mismatch",
            "invalid gguf version",
            "checksum mismatch",
        ] {
            let diag = diagnose_model_load_error(err);
            assert_eq!(diag.category, ModelLoadFailureKind::Unknown, "{err}");
            assert_eq!(diag.technical_details, err);
        }
    }

    #[test]
    fn each_recognized_category_has_actionable_guidance() {
        for err in [
            "file not found",
            "out of memory",
            "CUDA driver unavailable",
            "unsupported architecture",
            "incompatible runtime",
        ] {
            let diag = diagnose_model_load_error(err);
            assert!(!diag.recovery_step.trim().is_empty(), "{err}");
            assert!(!diag.explanation.trim().is_empty(), "{err}");
        }
    }
}
