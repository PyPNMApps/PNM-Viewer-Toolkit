/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import { useState } from "react";
import { backend, type Diagnostic, type Opened } from "./api";
import { ThemeSelector } from "./ThemeSelector";
import { RxMerChart } from "./RxMerChart";
/** Coordinates the first offline RxMER workflow without owning capture bytes.
 * @returns The desktop shell, metadata and exact-window chart.
 */
export function App() {
  const [capture, setCapture] = useState<Opened | null>(null);
  const [busy, setBusy] = useState(false);
  const [status, setStatus] = useState("Ready to open a local capture");
  const [failure, setFailure] = useState<string | null>(null);
  async function perform(action: () => Promise<void>, label: string) {
    setBusy(true);
    setFailure(null);
    setStatus(label);
    try {
      await action();
    } catch (error) {
      const diagnostic = error as Diagnostic;
      let message = diagnostic.message ?? String(error);
      if (diagnostic.field) {
        message += ` (${diagnostic.field}, byte ${diagnostic.offset})`;
      }
      setFailure(message);
      setStatus("Operation failed");
    } finally {
      setBusy(false);
    }
  }
  async function open() {
    await perform(async () => {
      const opened = await backend.open();
      if (opened) {
        setCapture(opened);
        setStatus("Capture decoded locally");
      } else {
        setStatus("Open canceled");
      }
    }, "Selecting and decoding capture…");
  }
  async function move(start: number) {
    if (!capture) {
      return;
    }
    await perform(async () => {
      const viewport = await backend.viewport(capture.session_id, start);
      setCapture({ ...capture, viewport });
      setStatus("Sample window updated");
    }, "Loading sample window…");
  }
  return (
    <div className="app-shell">
      <aside className="sidebar">
        <a className="brand" href="#workspace">
          <span className="brand-mark">∿</span>
          <span>
            PNM<span className="brand-sub">VIEWER TOOLKIT</span>
          </span>
        </a>
        <div className="section-label">WORKSPACE</div>
        <div className="nav-current">
          <span>⌁</span> RxMER viewer <span className="tiny-badge">01</span>
        </div>
        <div className="sidebar-note">
          <span className="status-dot" /> Offline workspace
          <p>
            Open a capture.
            <br />
            Explore the signal.
          </p>
        </div>
      </aside>
      <main id="workspace">
        <header className="topbar">
          <span>
            Measurements <span className="muted">/</span>{" "}
            <strong>Downstream RxMER</strong>
          </span>
          <ThemeSelector />
        </header>
        <div className="workspace-content">
          <div className="page-heading">
            <div>
              <div className="eyebrow">SIGNAL EXPLORER</div>
              <h1>Downstream RxMER</h1>
              <p>Receive modulation error ratio across OFDM subcarriers.</p>
            </div>
            <button className="primary" disabled={busy} onClick={open}>
              ＋ Open PNM file
            </button>
          </div>
          {failure && (
            <div role="alert" className="error">
              {failure}
            </div>
          )}
          {!capture ? (
            <section className="empty-state">
              <div className="empty-icon">∿</div>
              <span className="eyebrow">YOUR CAPTURE, ON YOUR DESKTOP</span>
              <h2>A clearer view of your signal.</h2>
              <p>
                Open a local RxMER PNM file to inspect its channel,
                <br />
                explore exact readings, and export decoded JSON.
              </p>
              <button className="primary" disabled={busy} onClick={open}>
                Open RxMER capture
              </button>
              <div className="empty-details">
                Local processing <span>·</span> RxMER format 1.0 <span>·</span>{" "}
                No server required
              </div>
            </section>
          ) : (
            <>
              <section className="capture-bar" aria-label="Capture">
                <div>
                  <span className="file-icon">▤</span>
                  <strong>{capture.metadata.filename}</strong>
                  <span className="support-badge">RxMER 1.0</span>
                </div>
                <button
                  disabled={busy}
                  onClick={() =>
                    perform(async () => {
                      await backend.close(capture.session_id);
                      setCapture(null);
                      setStatus("Capture closed");
                    }, "Closing capture…")
                  }
                >
                  Close
                </button>
              </section>
              <section className="metrics">
                <div>
                  <span>CHANNEL</span>
                  <strong>{capture.metadata.channel_id}</strong>
                </div>
                <div>
                  <span>MAC ADDRESS</span>
                  <strong className="mono">
                    {capture.metadata.mac_address}
                  </strong>
                </div>
                <div>
                  <span>SUBCARRIERS</span>
                  <strong>
                    {capture.metadata.sample_count.toLocaleString()}
                  </strong>
                </div>
                <div>
                  <span>SPACING</span>
                  <strong>
                    {capture.metadata.spacing_khz} <small>kHz</small>
                  </strong>
                </div>
              </section>
              <section className="chart-card">
                <div className="chart-heading">
                  <div>
                    <h2>RxMER by frequency</h2>
                    <p>
                      <span className="legend-dot" /> Exact samples · missing
                      readings remain gaps
                    </p>
                  </div>
                  <button
                    disabled={busy}
                    onClick={() =>
                      perform(async () => {
                        const saved = await backend.export(capture.session_id);
                        setStatus(
                          saved
                            ? "Full-resolution JSON exported"
                            : "Export canceled",
                        );
                      }, "Exporting JSON…")
                    }
                  >
                    Export JSON ↗
                  </button>
                </div>
                <RxMerChart view={capture.viewport} />
                <div className="chart-footer">
                  <span>
                    Samples {capture.viewport.start}–
                    {Math.max(
                      capture.viewport.start,
                      capture.viewport.start +
                        capture.viewport.points.length -
                        1,
                    )}{" "}
                    <span className="muted">
                      of {capture.viewport.total} · zero-based indices
                    </span>
                  </span>
                  <div>
                    <button
                      disabled={busy || capture.viewport.start === 0}
                      onClick={() =>
                        move(Math.max(0, capture.viewport.start - 1024))
                      }
                    >
                      ← Previous
                    </button>
                    <button
                      disabled={
                        busy ||
                        capture.viewport.start +
                          capture.viewport.points.length >=
                          capture.viewport.total
                      }
                      onClick={() =>
                        move(
                          capture.viewport.start +
                            capture.viewport.points.length,
                        )
                      }
                    >
                      Next →
                    </button>
                  </div>
                </div>
              </section>
              <p className="footnote">
                Viewing up to 1,024 original samples per window. JSON export
                includes every sample.
              </p>
            </>
          )}
        </div>
        <footer className="statusbar" role="status">
          <span>
            <span className="status-dot" />
            {status}
          </span>
          <span>PNM Viewer Toolkit · In development</span>
        </footer>
      </main>
    </div>
  );
}
