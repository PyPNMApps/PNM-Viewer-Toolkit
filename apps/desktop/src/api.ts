/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import { invoke } from "@tauri-apps/api/core";
export interface Metadata {
  filename: string;
  channel_id: number;
  mac_address: string;
  sample_count: number;
  spacing_khz: number;
}
export interface Point {
  frequency_hz: number;
  rxmer_db: number | null;
}
export interface Viewport {
  start: number;
  total: number;
  points: Point[];
}
export interface Opened {
  session_id: number;
  metadata: Metadata;
  viewport: Viewport;
}
export interface Diagnostic {
  code: string;
  message: string;
  field?: string | null;
  offset?: number | null;
}
export const backend = {
  /** Opens a native picker and replaces the session only after successful validation.
   * @returns Metadata and a bounded viewport, or null when canceled.
   */
  open: () => invoke<Opened | null>("open_capture"),
  /** Fetches exact samples from a Rust-owned session.
   * @param sessionId Opaque active session handle.
   * @param start Zero-based sample index.
   * @returns At most 1,024 samples, preserving null readings.
   */
  viewport: (sessionId: number, start: number) =>
    invoke<Viewport>("viewport", { sessionId, start }),
  /** Releases the matching Rust session without closing a newer capture.
   * @param sessionId Handle to release.
   * @returns Completion after Rust releases its capture reference.
   */
  close: (sessionId: number) => invoke<void>("close_capture", { sessionId }),
  /** Exports complete core JSON through a Rust-owned native save dialog.
   * @param sessionId Active capture handle.
   * @returns True on save, false on cancel; rejects errors and existing paths.
   */
  export: (sessionId: number) => invoke<boolean>("export_json", { sessionId }),
};
