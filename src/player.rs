use crate::Tampilkan;
use macroquad::prelude::*;

pub struct Player {
    pub img: Texture2D,
    pub x: f32,
    pub y: f32,
    pub ax: f32,
    pub vx: f32,
    pub rect: Rect,
    pub rot: f32,
    pub rot_speed: f32,
    pub oil_img: Texture2D,
    pub oil_rect: Rect,
}

impl Player {
    pub fn update(&mut self) {
        if is_key_down(KeyCode::Right) && self.rect.right() < 357.0 {
            self.vx += 200.0 * get_frame_time();
            self.rot_speed = 100.;
        } else if is_key_down(KeyCode::Left) && self.rect.left() > 48.0 {
            self.vx += -200.0 * get_frame_time();
            self.rot_speed = -100.;
        } else {
            self.rot = 0.;
            self.rot_speed = 0.;
        }

        self.rot = (self.rot + self.rot_speed * get_frame_time()) % 360.;

        self.ax = -self.vx * 0.2;
        self.vx += self.ax;
        self.x += self.vx;

        self.rect.x = self.x;
        self.rect.y = self.y;
        self.rect.w = self.img.width();
        self.rect.h = self.img.height();

        self.draw();
    }

    pub fn reset(&mut self) {
        self.x = screen_width() * 0.5;
        self.y = screen_height() - 200.0;
        self.ax = 0.0;
        self.vx = 0.0;
        self.oil_rect.w = 200.;
    }
}

impl Tampilkan for Player {
    fn draw(&mut self) {
        draw_texture_ex(
            self.img,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                rotation: self.rot.to_radians(),
                pivot: None,
                ..Default::default()
            },
        );

        // draw oil texture
        draw_texture(self.oil_img, 50., 20., WHITE);
        // oil rect outline
        draw_rectangle_lines(
            self.oil_rect.x,
            self.oil_rect.y,
            200.,
            self.oil_rect.h,
            4.,
            BLACK,
        );
        if self.oil_rect.w > 0. {
            self.oil_rect.w -= 0.03;
        }
        // oil rect
        draw_rectangle(
            self.oil_rect.x,
            self.oil_rect.y,
            self.oil_rect.w,
            self.oil_rect.h,
            GREEN,
        );
    }
}
