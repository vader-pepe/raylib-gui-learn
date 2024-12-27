use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    let btn = rl
        .load_texture(
            &thread,
            "resources/01_Flat_Theme/Spritesheets/Spritesheet_UI_Flat.png",
        )
        .unwrap();

    let font = rl
        .load_font(&thread, "resources/font_arcadeclassic/ARCADECLASSIC.TTF")
        .unwrap();

    let btn_rec = Rectangle {
        x: 288.0,
        y: 128.0,
        width: 64.0,
        height: 32.0,
    };

    let btn_bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        height: btn_rec.height as f32,
        width: btn_rec.width as f32,
    };

    let mut is_clicked = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mouse_point = d.get_mouse_position();

        d.clear_background(Color::WHITE);
        draw_btn(
            &mut d,
            &font,
            &btn,
            &btn_rec,
            "START",
            &Rectangle {
                x: ((SCREEN_WIDTH / 2) - (32 * 2)) as f32,
                y: ((SCREEN_HEIGHT / 2) - (32 * 2) - 50) as f32,
                height: 32.0 * 2.0,
                width: 64.0 * 2.0,
            },
        );
        draw_btn(
            &mut d,
            &font,
            &btn,
            &btn_rec,
            "EXIT",
            &Rectangle {
                x: ((SCREEN_WIDTH / 2) - (32 * 2)) as f32,
                y: ((SCREEN_HEIGHT / 2) - (32 * 2) + 50) as f32,
                height: 32.0 * 2.0,
                width: 64.0 * 2.0,
            },
        );
        draw_btn(
            &mut d,
            &font,
            &btn,
            &btn_rec,
            "SETTINGS",
            &Rectangle {
                x: ((SCREEN_WIDTH / 2) - (32 * 2)) as f32,
                y: ((SCREEN_HEIGHT / 2) - (32 * 2)) as f32,
                height: 32.0 * 2.0,
                width: 64.0 * 2.0,
            },
        );

        if btn_bounds.check_collision_point_rec(mouse_point) {
            if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                is_clicked = true;
            } else {
                is_clicked = false;
            };
        }
        if is_clicked {
            println!("clicked");
        }
    }
}

fn draw_btn(
    d: &mut RaylibDrawHandle,
    font: &Font,
    texture: &Texture2D,
    rec: &Rectangle,
    text: &str,
    pos: &Rectangle,
) {
    let font_dims = d.measure_text(text, 24);
    d.draw_texture_pro(
        &texture,
        rec,
        pos,
        Vector2 { x: 0.0, y: 0.0 },
        0.0,
        Color::WHITE,
    );
    d.draw_text_ex(
        &font,
        text,
        Vector2 {
            x: pos.x + (pos.width / 2.0) - ((font_dims as f32 / 2.5) as f32),
            y: pos.y + (pos.height / 2.0) - 15.0,
        },
        24.0,
        1.0,
        Color::BLACK,
    );
}
