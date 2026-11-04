/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import { useState } from "react";
import type { Point, Viewport } from "./api";
const WIDTH = 1000;
const HEIGHT = 260;
const LEFT = 60;
const RIGHT = 24;
const TOP = 24;
const BOTTOM = 52;
const DB_MAX = 64;
const HZ_PER_MHZ = 1_000_000;
/** Draws an exact sample window, breaking the line at unavailable readings.
 * @param view Bounded Rust-produced frequency and dB points; no scientific transforms occur here.
 * @returns An accessible SVG chart and keyboard/pointer sample inspector.
 */
export function RxMerChart({ view }: { view: Viewport }) {
  const [selected, setSelected] = useState(0);
  const count = view.points.length;
  const selectedIndex = Math.min(selected, Math.max(0, count - 1));
  const inspected = view.points[selectedIndex];
  const firstHz = view.points[0]?.frequency_hz ?? 0;
  const lastHz = view.points[count - 1]?.frequency_hz ?? firstHz;
  const xPosition = (point: Point) => {
    if (lastHz === firstHz) {
      return LEFT + (WIDTH - LEFT - RIGHT) / 2;
    }
    return (
      LEFT +
      ((point.frequency_hz - firstHz) / (lastHz - firstHz)) *
        (WIDTH - LEFT - RIGHT)
    );
  };
  const yPosition = (value: number) =>
    TOP + ((DB_MAX - value) / DB_MAX) * (HEIGHT - TOP - BOTTOM);
  let path = "";
  let connected = false;
  for (const point of view.points) {
    if (point.rxmer_db === null) {
      connected = false;
      continue;
    }
    let command = "M";
    if (connected) {
      command = "L";
    }
    path += `${command}${xPosition(point)},${yPosition(point.rxmer_db)} `;
    connected = true;
  }
  return (
    <div className="chart-wrap">
      <svg
        viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
        role="img"
        aria-label="RxMER in dB versus frequency in MHz. Missing readings appear as gaps."
        onPointerMove={(event) => {
          const bounds = event.currentTarget.getBoundingClientRect();
          const fraction =
            (((event.clientX - bounds.left) / bounds.width) * WIDTH - LEFT) /
            (WIDTH - LEFT - RIGHT);
          setSelected(
            Math.max(
              0,
              Math.min(count - 1, Math.round(fraction * (count - 1))),
            ),
          );
        }}
      >
        {[0, 16, 32, 48, 64].map((value) => (
          <g key={value}>
            <line
              className="grid"
              x1={LEFT}
              x2={WIDTH - RIGHT}
              y1={yPosition(value)}
              y2={yPosition(value)}
            />
            <text x={LEFT - 14} y={yPosition(value) + 4} textAnchor="end">
              {value}
            </text>
          </g>
        ))}
        {[0, 0.25, 0.5, 0.75, 1].map((fraction) => (
          <text
            key={fraction}
            x={LEFT + fraction * (WIDTH - LEFT - RIGHT)}
            y={HEIGHT - 26}
            textAnchor="middle"
          >
            {((firstHz + fraction * (lastHz - firstHz)) / HZ_PER_MHZ).toFixed(2)}
          </text>
        ))}
        <text x={LEFT} y={14}>
          RxMER · dB
        </text>
        <text x={WIDTH / 2} y={HEIGHT - 4} textAnchor="middle">
          Frequency · MHz
        </text>
        <path className="signal" d={path} />
        {view.points.map((point, index) => {
          if (
            point.rxmer_db !== null &&
            view.points[index - 1]?.rxmer_db == null &&
            view.points[index + 1]?.rxmer_db == null
          ) {
            return (
              <circle
                key={index}
                className="sample-dot"
                cx={xPosition(point)}
                cy={yPosition(point.rxmer_db)}
                r="3"
              />
            );
          }
          return null;
        })}
        {inspected && inspected.rxmer_db !== null && (
          <circle
            className="sample-dot"
            cx={xPosition(inspected)}
            cy={yPosition(inspected.rxmer_db)}
            r="4"
          />
        )}
      </svg>
      {count === 0 ? (
        <p className="empty-chart">This capture contains no samples.</p>
      ) : (
        <div className="sample-inspector">
          <label htmlFor="sample">Inspect sample</label>
          <input
            id="sample"
            type="range"
            min="0"
            max={count - 1}
            value={selectedIndex}
            onChange={(event) => setSelected(Number(event.target.value))}
          />
          <output>
            #{view.start + selectedIndex} ·{" "}
            {(inspected.frequency_hz / HZ_PER_MHZ).toFixed(3)} MHz ·{" "}
            {inspected.rxmer_db === null
              ? "Unavailable"
              : `${inspected.rxmer_db.toFixed(2)} dB`}
          </output>
        </div>
      )}
    </div>
  );
}
