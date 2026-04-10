use opentui_core_sys as sys;

/// Allocate a hyperlink URL in the global link pool, returning its ID.
///
/// The returned ID can be packed into cell attributes via
/// [`attributes_with_link`].
pub fn link_alloc(url: &str) -> u32 {
  unsafe { sys::linkAlloc(url.as_ptr(), url.len()) }
}

/// Get the URL for a link ID.
pub fn link_url(id: u32) -> String {
  let mut buf = vec![0u8; 2048];
  let len = unsafe { sys::linkGetUrl(id, buf.as_mut_ptr(), buf.len()) };
  buf.truncate(len);
  String::from_utf8_lossy(&buf).into_owned()
}

/// Pack a link ID into cell attributes.
pub fn attributes_with_link(base_attributes: u32, link_id: u32) -> u32 {
  unsafe { sys::attributesWithLink(base_attributes, link_id) }
}

/// Extract the link ID from cell attributes.
pub fn attributes_get_link_id(attributes: u32) -> u32 {
  unsafe { sys::attributesGetLinkId(attributes) }
}

/// Clear the global link pool, freeing all allocated URLs.
pub fn clear_global_link_pool() {
  unsafe { sys::clearGlobalLinkPool() }
}
