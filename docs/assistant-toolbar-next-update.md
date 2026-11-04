# Deferred assistant toolbar changes — 2026-11-04

Status: **Not started.** The user requested documentation for the next agent only; do not implement as part of this documentation change.

- **GridCraft:** Move the Assistant button shown at the left of the View ribbon to the far right of that ribbon, and add its missing icon. The screenshot also shows an existing Assistant control in the title bar; preserve its behavior unless the user directs otherwise.
- **WordCraft:** Move the Assistant button shown at the left of the View ribbon to the far right of that ribbon. Retain its existing icon.
- **PdfCraft:** Replace the text-only Assistant toolbar control with an AI sparkle-star icon. Preserve an accessible Assistant name and a clear tooltip.

Before implementing, inspect each app's current layout and nearby controls; confirm placement and icon behavior at narrow widths, in light and dark themes, and with keyboard/accessibility navigation. Preserve the existing assistant toggle behavior. Use assets permitted by each repository's asset policy. Test the actual interface and relevant automated checks. Record implementation, verification and any deployment separately; none has been performed for this request.
