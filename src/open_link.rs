// helper for opening links to websites
// can be handled with the `open` crate on native platforms
// but wasm needs a special handler
pub fn open_url(url: String) -> Result<(), std::io::Error> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        window.open_with_url(&url).unwrap();
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        open::that(url)
    }
}
