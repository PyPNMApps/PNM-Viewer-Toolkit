/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react";
import { App } from "./App";
import { backend } from "./api";
import { RxMerChart } from "./RxMerChart";
vi.mock("./api", () => ({
  backend: {
    open: vi.fn(),
    viewport: vi.fn(),
    close: vi.fn(),
    export: vi.fn(),
  },
}));
let darkSystem = false;
let onSystemChange: (() => void) | undefined;
beforeEach(() => {
  darkSystem = false;
  onSystemChange = undefined;
  vi.clearAllMocks();
  Object.defineProperty(window, "matchMedia", {
    configurable: true,
    value: () => ({
      get matches() {
        return darkSystem;
      },
      addEventListener: (_name: string, listener: () => void) => {
        onSystemChange = listener;
      },
      removeEventListener: () => {
        onSystemChange = undefined;
      },
    }),
  });
});
afterEach(cleanup);
describe("desktop workflow", () => {
  it("follows system changes and respects explicit theme overrides", () => {
    render(<App />);
    expect(document.documentElement.dataset.theme).toBe("light");
    darkSystem = true;
    onSystemChange?.();
    expect(document.documentElement.dataset.theme).toBe("dark");
    fireEvent.change(screen.getByLabelText("Appearance"), {
      target: { value: "light" },
    });
    onSystemChange?.();
    expect(document.documentElement.dataset.theme).toBe("light");
    fireEvent.change(screen.getByLabelText("Appearance"), {
      target: { value: "dark" },
    });
    expect(document.documentElement.dataset.theme).toBe("dark");
    darkSystem = false;
    fireEvent.change(screen.getByLabelText("Appearance"), {
      target: { value: "system" },
    });
    expect(document.documentElement.dataset.theme).toBe("light");
  });
  it("opens bounded data, pages, exports and closes", async () => {
    const first = {
      start: 0,
      total: 2,
      points: [{ frequency_hz: 640000000, rxmer_db: 42.75 }],
    };
    vi.mocked(backend.open).mockResolvedValue({
      session_id: 1,
      metadata: {
        filename: "rxmer.bin",
        channel_id: 34,
        mac_address: "a1:b2:c3:d4:e5:f6",
        sample_count: 2,
        spacing_khz: 25,
      },
      viewport: first,
    });
    vi.mocked(backend.viewport).mockResolvedValue({ ...first, start: 1 });
    vi.mocked(backend.export).mockResolvedValue(true);
    render(<App />);
    fireEvent.click(screen.getByText("Open RxMER capture"));
    await screen.findByText("rxmer.bin");
    expect(screen.getByText(/42.75 dB/)).toBeTruthy();
    fireEvent.click(screen.getByText("Next →"));
    await waitFor(() => expect(backend.viewport).toHaveBeenCalledWith(1, 1));
    await screen.findByText("Sample window updated");
    fireEvent.click(screen.getByText("Export JSON ↗"));
    await screen.findByText("Full-resolution JSON exported");
    expect(backend.export).toHaveBeenCalledWith(1);
    fireEvent.click(screen.getByText("Close"));
    await screen.findByText("Open RxMER capture");
    expect(backend.close).toHaveBeenCalledWith(1);
  });
  it("handles dialog cancel and reports binary diagnostics", async () => {
    vi.mocked(backend.open)
      .mockResolvedValueOnce(null)
      .mockRejectedValueOnce({
        message: "Unsupported version",
        field: "format_version",
        offset: 4,
      });
    render(<App />);
    fireEvent.click(screen.getByText("Open RxMER capture"));
    await screen.findByText("Open canceled");
    fireEvent.click(screen.getByText("Open RxMER capture"));
    expect((await screen.findByRole("alert")).textContent).toContain(
      "format_version, byte 4",
    );
  });
  it("does not connect lines across unavailable values", () => {
    const { container } = render(
      <RxMerChart
        view={{
          start: 0,
          total: 3,
          points: [
            { frequency_hz: 1, rxmer_db: 40 },
            { frequency_hz: 2, rxmer_db: null },
            { frequency_hz: 3, rxmer_db: 41 },
          ],
        }}
      />,
    );
    const path = container.querySelector("path")!.getAttribute("d")!;
    expect(path.split("M")).toHaveLength(3);
    expect(path).not.toContain("L");
    fireEvent.change(screen.getByLabelText("Inspect sample"), {
      target: { value: "1" },
    });
    expect(screen.getByText(/Unavailable/)).toBeTruthy();
  });
});
