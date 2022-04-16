use crate::Tampilkan;
use macroquad::prelude::*;

pub struct Panah {
    pub img: Texture2D,
    pub x: f32,
    pub y: f32,
}

pub struct Car {
    pub img: Texture2D,
    pub x: f32,
    pub y: f32,
    pub rect: Rect,
}

impl Car {
    pub fn update(&mut self, gambar: [Texture2D; 4]) {
        self.y += 2.0;

        if self.y > screen_height() {
            self.y = -250.0;
            self.x = rand::gen_range(82.0, 322.0);
            self.img = gambar[rand::gen_range(0, 4)];
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
        self.rect.w = self.img.width();
        self.rect.h = self.img.height();

        self.draw();
    }

    pub fn reset(&mut self, gambar: [Texture2D; 4]) {
        self.x = rand::gen_range(82.0, 322.0);
        self.y = -250.0;
        self.img = gambar[rand::gen_range(0, 4)];
    }
}

impl Tampilkan for Car {
    fn draw(&mut self) {
        draw_texture(self.img, self.x, self.y, WHITE);
    }
}

pub struct Batu {
    pub img: Texture2D,
    pub x: f32,
    pub y: f32,
    pub rect: Rect,
}

impl Batu {
    pub fn update(&mut self, gambar: [Texture2D; 3]) {
        self.y += 1.0;

        if self.y > screen_height() {
            self.y = -80.0;
            self.x = rand::gen_range(82.0, 322.0);
            self.img = gambar[rand::gen_range(0, 3)];
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
        self.rect.w = self.img.width();
        self.rect.h = self.img.height();

        self.draw();
    }

    pub fn reset(&mut self, gambar: [Texture2D; 3]) {
        self.x = rand::gen_range(82.0, 322.0);
        self.y = -250.0;
        self.img = gambar[rand::gen_range(0, 3)];
    }
}

impl Tampilkan for Batu {
    fn draw(&mut self) {
        draw_texture(self.img, self.x, self.y, WHITE);
    }
}

pub struct Oli {
    pub img: Texture2D,
    pub x: f32,
    pub y: f32,
    pub rect: Rect,
}

impl Oli {
    pub fn update(&mut self) {
        self.y += 1.0;

        if self.y > screen_height() {
            self.y = -80.0;
            self.x = rand::gen_range(82.0, 322.0);
        }

        self.rect.x = self.x;
        self.rect.y = self.y;
        self.rect.w = self.img.width();
        self.rect.h = self.img.height();

        self.draw();
    }

    pub fn reset(&mut self) {
        self.x = rand::gen_range(82.0, 322.0);
        self.y = -250.0;
    }
}

impl Tampilkan for Oli {
    fn draw(&mut self) {
        draw_texture(self.img, self.x, self.y, WHITE);
    }
}
