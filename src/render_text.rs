pub fn render_text(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font,
    text: &str,
    posx: i32,
    posy: i32,
    col: sdl2::pixels::Color,
) {
    let surface = font
        .render(text)
        .blended(col)
        .map_err(|e| e.to_string())
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_query = texture.query();
    let texture_width = texture_query.width;
    let texture_height = texture_query.height;
    let target = sdl2::rect::Rect::new(posx, posy, texture_width, texture_height);
    canvas.copy(&texture, None, Some(target)).unwrap();
}
