# Troubleshooting Model Load Failures

This guide outlines common errors encountered when loading local models in deepLocal and safe recovery steps. Categories describe likely causes only when runtime diagnostics provide a recognizable signal. Unknown errors remain unclassified; copy their technical details when reporting an issue. A file-format or integrity error alone does not establish that a model architecture is unsupported.

| Category | Diagnostic Explanation | Recovery Guidance |
| :--- | :--- | :--- |
| **Model File Missing or Unreadable** (`file_not_found_or_unreadable`) | The target model file does not exist at the provided filesystem path or read permissions are missing. | Verify that the file path is correct, check operating system permissions, or locate and re-link the file in settings. |
| **Insufficient Memory** (`insufficient_memory`) | System RAM or GPU VRAM is insufficient for the requested model size, context window, or batch settings. | Lower the context length, decrease the number of GPU offloaded layers, or choose a smaller quantization format (e.g., Q4_K_M instead of Q8_0). |
| **GPU Backend Unavailable** (`gpu_backend_unavailable`) | The requested hardware accelerator (CUDA, Metal, Vulkan, ROCm) failed to initialize or is not present. | Switch the execution backend to CPU mode or ensure GPU drivers and prerequisite runtimes are installed. |
| **Unsupported Architecture** (`unsupported_architecture`) | The model architecture or tensor metadata is not supported by the current runtime engine version. | Ensure the model file is a valid GGUF binary and update your runtime engine to the latest compatible release. |
| **Runtime Incompatible or Missing** (`runtime_incompatible_or_missing`) | The designated backend engine executable or shared library could not be located or launched. | Select another installed runtime from the dashboard or verify required system dependencies. |
| **Unknown Error** (`unknown`) | The engine encountered an unclassified failure. | Copy the technical error details from the diagnostic card and check the runtime log output for deeper inspection. |
