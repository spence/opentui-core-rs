use opentui_core::{attributes_get_link_id, attributes_with_link, link_alloc, link_url, Renderer};

// The global link pool is initialized when a renderer is created.
fn ensure_initialized() -> Renderer {
  Renderer::with_options(10, 10, true, false).unwrap()
}

#[test]
fn link_alloc_and_get() {
  let _r = ensure_initialized();
  let id = link_alloc("https://example.com");
  assert!(id > 0);

  let url = link_url(id);
  assert_eq!(url, "https://example.com");
}

#[test]
fn link_alloc_multiple() {
  let _r = ensure_initialized();
  let id1 = link_alloc("https://one.com");
  let id2 = link_alloc("https://two.com");
  assert_ne!(id1, id2);

  assert_eq!(link_url(id1), "https://one.com");
  assert_eq!(link_url(id2), "https://two.com");
}

#[test]
fn link_nonexistent_id() {
  let _r = ensure_initialized();
  let url = link_url(99999);
  assert!(url.is_empty());
}

#[test]
fn attributes_link_roundtrip() {
  let _r = ensure_initialized();
  let link_id = link_alloc("https://roundtrip.com");
  let attrs = attributes_with_link(0, link_id);
  let extracted = attributes_get_link_id(attrs);
  assert_eq!(extracted, link_id);
}

#[test]
fn attributes_no_link() {
  let id = attributes_get_link_id(0);
  assert_eq!(id, 0);
}
