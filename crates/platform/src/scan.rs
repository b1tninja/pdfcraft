//! Image acquisition. The settings, errors and page handling are `pdfcraft-scan`; the Windows WIA
//! backend is added separately. This module is the platform surface the app and the tools share, and
//! it picks the backend: for now every platform answers "unsupported".
//!
//! [`acquire`] and [`ProgressToken`] deliver in-memory [`Page`] buffers. Path helpers such as
//! [`write_pages`] are optional sinks for callers that want files; bindings that stay in memory
//! never need them.

/// The shared BMP header parser and RGBA decoder (`pdfcraft_scan::bmp`).
pub use pdfcraft_scan::bmp;
pub use pdfcraft_scan::{
    Acquired, CancelToken, Color, Compression, Connection, DeviceDump, ErrorContext, HandlingStatus, ImageChoices, ImageFormat, Intent,
    InterruptKind, ItemDump, ListDetails, MAX_DPI, MAX_PAGES, MIN_DPI, Page, PageSize, ParseWireError, Phase, PlannedFrame, ProgressToken,
    PropertyInfo, Recovery, Region, RegionPixels, Rotation, ScanError, ScanErrorInfo, ScanSettings, Scanner, ScannerRef, Source, ValueSpan, WiaCause,
    align_to_span, bmp_size, code_name, code_names, contract_json, deny_list, describe_windows_error, estimated_bmp_bytes, filter_is_denied,
    fit_region, handling_caps, image_format, interleave_manual_duplex, interrupt_kind, kind_for_hresult, listed_page_files, page_path,
    parse_deny_list, pixels_for, planned_frame, preview_dpi, recovery, region_from_inches, region_from_preview, region_pixels, rescale_extent,
    wia_error_message, write_pages,
};

pub use pdfcraft_scan::unsupported::{acquire, dump_devices, scanner_details, scanner_list, scanners};
