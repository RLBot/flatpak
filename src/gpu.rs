//! GPU workarounds the GUI needs before WebKitGTK starts.
//!
//! WebKitGTK renders through the DMA-BUF renderer on Linux, which maps
//! dma-bufs it imports from the GPU. The proprietary NVIDIA driver fails that
//! map silently, so the GUI comes up blank (webkit bug 262607 / 180739), and
//! `WEBKIT_DISABLE_DMABUF_RENDERER=1` makes it fall back to a path that works.
//!
//! The DMA-BUF renderer also leaks memory unboundedly when a page shows many
//! `backdrop-filter` elements: opening the GUI's RocketHost page (about twenty
//! blurred bot pills) makes the WebKitWebProcess grow by hundreds of megabytes
//! every few seconds at 100%+ CPU until the GUI freezes and the system runs
//! out of RAM. The same binary on the host is stable, so this is specific to
//! the sandbox/runtime renderer path. Disabling the renderer avoids that too,
//! so the launcher now sets the variable unconditionally (unless the user
//! already set it themselves).

use std::env;

/// The WebKitGTK switch that turns off the DMA-BUF renderer.
pub const DMABUF_RENDERER_ENV: &str = "WEBKIT_DISABLE_DMABUF_RENDERER";

/// True when the GUI needs `WEBKIT_DISABLE_DMABUF_RENDERER` set on its
/// environment: always, unless the user has set the variable themselves.
///
/// Any value counts as the user having spoken, since WebKit treats a set
/// variable as "disabled" regardless of its contents. There is currently no
/// way to re-enable the DMA-BUF renderer short of changing this code; that
/// is intentional until the runtime's WebKit fixes the leak.
pub fn needs_dmabuf_workaround() -> bool {
    env::var_os(DMABUF_RENDERER_ENV).is_none()
}
