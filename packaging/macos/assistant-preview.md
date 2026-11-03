# PDFCraft Assistant Preview packaging

This local-only package keeps `/Applications/PDFCraft.app` separate. It uses the
bundle identity `ai.storyteller.pdfcraft.assistant-preview` and a portable launcher
that selects `~/Library/Application Support/PDFCraft Assistant Preview` on each Mac.
`PDFCRAFT_PROFILE_DIR` overrides that directory; it must be absolute and writable.
Preferences, logs, recovery and digital IDs follow the profile. Existing profiles
and shared Craft assistant settings, credentials and conversations are not copied.

Build the committed Apple-silicon release with `PDFCRAFT_BUILD_LABEL` set to the
source commit's eight-character prefix. Then run:

```sh
packaging/macos/assistant-preview.sh \
  --binary /path/to/release/pdfcraft \
  --version 0.18.0 \
  --build-sha FULL_40_CHARACTER_SOURCE_COMMIT \
  --output-dir /path/to/dist/assistant-preview/COMMIT
```

The script refuses an architecture, version or commit-label mismatch, older binaries
without `--disable-updates`, and existing outputs. It emits version/commit-named
ARM64 DMG and ZIP packages, SHA-256 checksums and installation instructions. It checks
the assembled app, ZIP extraction, mounted DMG and a copied DMG app with strict
signature verification; extracted launchers and native executable hashes are checked too.

Default signing is ad-hoc. The script does not notarize. A downloaded copy may require
Apple's per-app approval on the receiving Mac; do not disable Gatekeeper globally.
Optional embedded font inputs must have matching licences supplied at packaging time.
No OCR models are added by this preview packager; existing model discovery remains available.

The launcher always passes `--disable-updates`, covering startup and manual checks.
Preview updates require deliberate upstream source integration, verification and a new
preview build. Save and quit normally before replacement, retain a rollback app, and
never replace the main PDFCraft app. Packaging checks do not establish provider
inference correctness or receiving-Mac acceptance.


Local 0.18 candidates use the same committed native executable in the standard and preview
bundles. The preview retains its separate launcher, bundle identifier and document profile;
the assistant settings, credentials and conversations remain shared. A local signing
certificate is not a Developer ID/notarization receipt. Candidate creation does not install,
activate or publish an app or change the configured assistant-enabled app release source.
