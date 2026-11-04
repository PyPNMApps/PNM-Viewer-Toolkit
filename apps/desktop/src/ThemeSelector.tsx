/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import { useEffect, useState } from "react";
type ThemePreference = "system" | "dark" | "light";
/** Presents an explicit theme preference; System follows live OS changes.
 * The preference lasts for this application session; no settings are persisted.
 * @returns An accessible selector controlling the document palette.
 */
export function ThemeSelector() {
  const [preference, setPreference] = useState<ThemePreference>("system");
  useEffect(() => {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const apply = () => {
      let resolved: string = preference;
      if (preference === "system") {
        resolved = media.matches ? "dark" : "light";
      }
      document.documentElement.dataset.theme = resolved;
    };
    apply();
    media.addEventListener("change", apply);
    return () => media.removeEventListener("change", apply);
  }, [preference]);
  return (
    <label className="theme-control">
      Appearance
      <select
        aria-label="Appearance"
        value={preference}
        onChange={(event) =>
          setPreference(event.target.value as ThemePreference)
        }
      >
        <option value="system">System</option>
        <option value="dark">Dark</option>
        <option value="light">Light</option>
      </select>
    </label>
  );
}
