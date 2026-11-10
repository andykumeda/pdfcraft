//! pdfcraft-automation — agent control for PdfCraft (architecture §13).
//!
//! - **Layer:** L7. Headless: depends on the engine, never on a UI toolkit.
//! - [`Automation`] is a tool set over an engine [`Session`]: open, inspect, render, extract and
//!   find text, edit pages and metadata, undo/redo, save, combine, extract and split. Every tool
//!   has a JSON Schema ([`tools`]) and takes and returns JSON, so the same table drives the MCP
//!   server ([`mcp`]), `pdfcraft-cli run` and (later) the UI control channel.
//! - Pages are **1-based** in every tool, as people number them. Rectangles are in PDF points
//!   with the origin at the top-left of the displayed page.
//! - An optional root directory confines every path a tool reads or writes. Separately,
//!   [`file_access::AutomationRoots`] (`pdfcraft-cli mcp --automation-read-root` and
//!   `--automation-write-root`) holds a read folder and a write folder, each left out meaning no
//!   access of its kind.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod a11y;
mod batch;
mod bates;
mod comments;
mod content;
mod conventions;
pub mod file_access;
mod forms;
mod hairlines;
mod layers;
mod links;
#[cfg(feature = "mcp")]
pub mod mcp;
mod measure;
mod model3d;
mod model_render;
mod printing;
mod reading;
mod redact;
mod signing;
mod text_cache;
mod tools;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

use pdfcraft_engine::{BlockKey, CombineSource, DocId, Document, Edit, Session, TextBlock, block_overlapping, commands};
use pdfcraft_platform::staging::{StagingName, staging_suffixes};
use pdfcraft_render::{PageRenderer, PageText, RenderConfig, RenderRequest, RequestKind};
use serde_json::{Value, json};

pub use pdfcraft_render::Pixels;
pub use tools::{ToolDef, tools};

use text_cache::TextCache;

mod assemble;
mod bookmark_io;
mod dispatch;
mod documents;
mod error;
mod json;
mod pages;
mod paths;

pub use dispatch::Content;
pub use error::ToolError;
use error::{Result, failed};
pub(crate) use json::destination;
pub use json::view_rect;
use json::{Args, info, summary};
use pages::{DEFAULT_DPI, MAX_DPI, encode_png, rendered_content};
#[cfg(test)]
use pages::{FIND_FIRST_BATCH, FIND_MAX_WARNING_BYTES, FIND_MAX_WARNINGS, extract_parallel};
#[cfg(test)]
pub(crate) use paths::MAX_INPUT;
use paths::{child, lexical, write_atomic};
pub(crate) use paths::{read_input, read_input_bytes};
#[cfg(test)]
use paths::{write_atomic_synced, write_atomic_with};

struct OcrReview {
    handle: u64,
    guard: pdfcraft_engine::ocr::OcrGuard,
    pages: Vec<pdfcraft_engine::ocr::OcrPage>,
}

/// A headless PdfCraft session driven by tool calls.
pub struct Automation {
    session: Session,
    root: Option<PathBuf>,
    /// The read and write folders of `--automation-read-root` / `--automation-write-root`.
    roots: Option<Arc<file_access::AutomationRoots>>,
    /// Synchronous renderers, rebuilt when a document's working bytes change.
    renderers: HashMap<DocId, (Arc<Vec<u8>>, PageRenderer)>,
    /// Extracted page text, under one memory budget across all open documents.
    texts: TextCache,
    /// The paragraphs of the last `text_paragraphs` listing of each page, so a paragraph number
    /// keeps naming that paragraph after edits regroup the page (#2516).
    listed: HashMap<(DocId, usize), Vec<BlockKey>>,
    /// Documents that lost a listing to the cap. A page of one with no listing is refused rather
    /// than read as "a number is a position", which the page may no longer match.
    evicted: HashSet<DocId>,
    /// Review sessions held for `ocr_words` (a guard and the pages read), by document.
    ocr_reviews: HashMap<DocId, OcrReview>,
    next_ocr_review: u64,
    /// What object_copy copied, for object_paste into any open document.
    objects_clipboard: Option<pdfcraft_engine::ObjectClipboard>,
    /// Saved-source identities prevent reused document ids in another session from hitting caches.
    cache_sources: HashMap<DocId, Weak<Vec<u8>>>,
}

impl Default for Automation {
    fn default() -> Self {
        Self::new()
    }
}

impl Automation {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
            root: None,
            roots: None,
            renderers: HashMap::new(),
            texts: TextCache::default(),
            listed: HashMap::new(),
            evicted: HashSet::new(),
            ocr_reviews: HashMap::new(),
            next_ocr_review: 0,
            objects_clipboard: None,
            cache_sources: HashMap::new(),
        }
    }

    /// Confine every path the tools read or write to `root` (relative paths resolve inside it).
    pub fn with_root(mut self, root: impl Into<PathBuf>) -> std::io::Result<Self> {
        let root = root.into().canonicalize()?;
        if !root.is_dir() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "the root must be a folder"));
        }
        self.root = Some(root);
        Ok(self)
    }

    /// Confine the files the tools read to the read root and the files they write to the write
    /// root (`None`: no confinement). Applies on top of [`with_root`](Self::with_root) when both
    /// are set.
    pub fn with_automation_roots(mut self, roots: Option<Arc<file_access::AutomationRoots>>) -> Self {
        self.roots = roots;
        self
    }

    /// Use a fixed clock for saves (deterministic output in tests).
    pub fn with_clock(mut self, clock: fn() -> i64) -> Self {
        self.session = std::mem::take(&mut self.session).with_clock(clock);
        self
    }

    /// Multiply the time XFA form scripts may take (tests). See [`Session::with_xfa_time_scale`].
    pub fn with_xfa_time_scale(mut self, factor: u32) -> Self {
        self.session = std::mem::take(&mut self.session).with_xfa_time_scale(factor);
        self
    }

    /// The clock and time zone of a host without its own. See [`Session::with_wall_clock`].
    pub fn with_wall_clock(mut self, clock: fn() -> i64, utc_offset: fn() -> i64) -> Self {
        self.session = std::mem::take(&mut self.session).with_wall_clock(clock, utc_offset);
        self
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Run an existing tool against the caller's live session, including its Undo history.
    ///
    /// No document is reopened or copied. The session is returned even if dispatch unwinds;
    /// an escaped panic becomes a fixed error without exposing its potentially private payload.
    /// Callers still own authorization, cancellation and refreshing their document views.
    pub fn call_on_session(&mut self, session: &mut Session, name: &str, args: &Value) -> Result<Vec<Content>> {
        self.on_session(session, |automation| automation.call(name, args))
    }

    fn on_session<T>(&mut self, session: &mut Session, run: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        std::mem::swap(&mut self.session, session);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(self)));
        std::mem::swap(&mut self.session, session);
        result.unwrap_or_else(|_| {
            self.renderers.clear();
            self.texts = TextCache::default();
            self.evicted.extend(self.listed.keys().map(|(doc, _)| *doc));
            self.listed.clear();
            Err(failed("tool execution failed unexpectedly; inspect the document before retrying"))
        })
    }

    fn prepare_session_caches(&mut self) {
        let sources: HashMap<_, _> = self.session.docs().iter().map(|doc| (doc.id, Arc::downgrade(&doc.saved_bytes()))).collect();
        let same = |id: &DocId| self.cache_sources.get(id).zip(sources.get(id)).is_some_and(|(old, new)| old.ptr_eq(new));
        self.renderers.retain(|id, _| same(id));
        self.ocr_reviews.retain(|id, _| same(id));
        self.evicted.retain(|id| sources.contains_key(id));
        self.listed.retain(|(id, _), _| {
            if same(id) {
                true
            } else {
                self.evicted.insert(*id);
                false
            }
        });
        for id in self.cache_sources.keys().filter(|id| !same(id)) {
            self.texts.remove(id);
        }
        self.cache_sources = sources;
    }

    /// Save `bytes` that a caller writes on the session's behalf, such as `pdfcraft-cli run`
    /// saving a rendered page. With a root, it is confined like a tool's own output (a relative
    /// path resolves inside it). Written atomically either way (#2166), except to something
    /// that is not a regular file, such as `/dev/stdout` or a pipe, which is written as given.
    /// Returns where the file went.
    pub fn write_output(&self, path: &str, bytes: &[u8]) -> Result<PathBuf> {
        let target = self.resolve(path, true)?;
        let special = self.root.is_none() && self.roots.is_none() && std::fs::metadata(&target).is_ok_and(|m| !m.is_file() && !m.is_dir());
        if special {
            std::fs::write(&target, bytes).map_err(|e| failed(format!("{path}: {e}")))?;
        } else {
            write_atomic(&target, bytes)?;
        }
        Ok(target)
    }
}

#[cfg(test)]
mod tests;
