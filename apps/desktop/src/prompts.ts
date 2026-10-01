/** Instructions with the highest priority in a model request. */
export interface HighestPriorityPrompt {
  content(): string;
}

export class DefaultHighestPriorityPrompt implements HighestPriorityPrompt {
  constructor(private readonly additionalInstructions?: string | null) {}

  content(): string {
    return [
      "You are a helpful local assistant. Answer in the user's language. Be accurate and concise. " +
        "When asked for code, produce complete valid code, avoid repeating tokens or unfinished fragments, and do not claim that code is complete when it is not.",
      this.additionalInstructions?.trim(),
    ]
      .filter(Boolean)
      .join("\n\n");
  }
}
