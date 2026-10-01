import { describe, expect, it } from "vitest";
import { DefaultHighestPriorityPrompt, type HighestPriorityPrompt } from "../src/prompts";

describe("HighestPriorityPrompt", () => {
  it("provides the default model instructions", () => {
    expect(new DefaultHighestPriorityPrompt().content()).toContain("helpful local assistant");
  });

  it("composes optional instructions after the default instructions", () => {
    const content = new DefaultHighestPriorityPrompt("Be especially concise.").content();
    expect(content).toContain("Be especially concise.");
    expect(content.indexOf("helpful local assistant")).toBeLessThan(content.indexOf("Be especially concise."));
  });

  it("supports future prompt providers through the interface", () => {
    const customPrompt: HighestPriorityPrompt = { content: () => "Workspace policy" };
    expect(customPrompt.content()).toBe("Workspace policy");
  });
});
