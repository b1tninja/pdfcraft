# PdfCraft CLI

Command-line entry points live in the `pdfcraft-cli` crate (see the repository README for the full command list).

## Scan (`pdfcraft-cli scan`)

Scanning is Windows/WIA only in this version: on other platforms `pdfcraft-cli scan` exits with the line `scan: scanning is only supported on Windows (WIA) in this version`, and the `scan_devices` and `scan_acquire` tools return the same sentence. Other backends are future work behind the same seam.

Windows acquire uses WIA through `pdfcraft-wia` (the COM layer; `pdfcraft-scan` holds the shared rules). See [hardware-parity.md](hardware-parity.md#windows-wia-scanner-acquire) for devices, feeder, WSD, and duplex behavior.

**Checking the scan stack (`pdfcraft-cli scan check`)**

`scan check` reports what the scan stack sees, so real hardware can be validated without scanning. Nothing is written unless `--out` is given, and nothing moves unless `--acquire` is given.

| Command | What it does |
|---------|--------------|
| `scan check` | Lists the WIA devices (id, name, port, connection). No device is opened. |
| `scan check --details` | Opens every device that is not denied, read-only, and prints sources (glass, feeder, duplex), resolutions, colour modes, formats, bed and sheet sizes, and the document-handling state. |
| `scan check --device TEXT` | The same for exactly one device. TEXT is a WIA id (exact) or a unique part of the id, name or port; zero or several matches is an error. |
| `scan check --device TEXT --acquire [--out DIR]` | Also makes one 100 dpi grey acquire and reports format, size and byte count. The glass is used when the device has one, else the feeder. Prints a one-line warning to stderr first. The pixels are saved only into `--out`. `--acquire` without `--device` is refused. |
| `--json` | One line of JSON instead of text. Keys: `supported`, `mode` (`list`, `details`, `device`), `count`, `scanners` (the same rows as `scan devices`), `acquire` (`null`, or `{ok, source, requested_dpi, count, pages[{format, width, height, dpi, bytes, path}]}`, or `{ok: false, error, error_info}`), `ok`, `problems`. |

Exit status is 1 when a device would not open, the acquire failed, or the arguments are wrong; the report is still printed first. A device named in `PDFCRAFT_SCAN_DENY` is never opened (a check that names it is refused). On other platforms every `scan check` ends with `scan check: scanning is only supported on Windows (WIA) in this version`.

**Diagnostics**

| Variable | Effect |
|----------|--------|
| `PDFCRAFT_SCAN_TRACE` | `1` or `true`: append one JSON object per acquire phase to stderr. Any other non-empty value is treated as a path and lines are appended to that JSONL file. Phases include `device_open`, `props_written`, `transfer_start`, `transfer_end`, and `status_read` (handling bits only). No device names, ids, ports, or image data. |
| `PDFCRAFT_SCAN_STATUS_LOG` | `1`: log each WIA transfer STATUS callback (`percent`, `transferred` bytes) to stderr during file transfer. |

Example (stderr trace during acquire):

```powershell
$env:PDFCRAFT_SCAN_TRACE = "1"
pdfcraft-cli scan acquire --device "…" --source feeder --pages 1 --out-dir .\out
```

Manual check: run a short acquire with `PDFCRAFT_SCAN_TRACE=1` and confirm phase lines appear on stderr in order through `transfer_end`.
