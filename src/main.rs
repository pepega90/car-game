use macroquad::prelude::*;
use macroquad::audio::*;

fn config() -> Conf {
    Conf {
        window_width: 400,
        window_height: 640,
        window_title: "Car Game".to_string(),
        window_resizable: false,
        ..Default::default()
    }
}

enum Scene {
    MENU,
    PLAY,
    GAMEOVER,
}

trait Tampilkan {
    fn draw(&mut self);
}

struct Player {
    img: Texture2D,
    x: f32,
    y: f32,
    ax: f32,
    vx: f32,
    rect: Rect,
    rot: f32,
    rot_speed: f32,
    oil_img: Texture2D,
    oil_rect: Rect,
}

impl Player {
    fn update(&mut self) {
        if is_key_down(KeyCode::Right) && self.rect.right() < 357.0
        {
            self.vx += 200.0 * get_frame_time();
            self.rot_speed = 100.;
        }

        else if is_key_down(KeyCode::Left) && self.rect.left() > 48.0
        {
            self.vx += -200.0 * get_frame_time();
            self.rot_speed = -100.;
        }
        else
        { 
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

    fn reset(&mut self) {
        self.x = screen_width()* 0.5; 
        self.y = screen_height() - 200.0;
        self.ax = 0.0;
        self.vx = 0.0;
        self.oil_rect.w = 200.;
    }
}

impl Tampilkan for Player {
    fn draw(&mut self) {
        draw_texture_ex(self.img, self.x, self.y, WHITE, DrawTextureParams{
            rotation: self.rot.to_radians(),
            pivot: None,
            ..Default::default()
        });
        

        // draw oil texture
        draw_texture(self.oil_img, 50., 20., WHITE);
        // oil rect outline
        draw_rectangle_lines(self.oil_rect.x, self.oil_rect.y, 200., self.oil_rect.h, 4.,BLACK);
        if self.oil_rect.w > 0. {
            self.oil_rect.w -= 0.03;
        }
        // oil rect
        draw_rectangle(self.oil_rect.x, self.oil_rect.y, self.oil_rect.w, self.oil_rect.h, GREEN);
    }
}

struct Panah{
    img: Texture2D,
    x: f32,
    y: f32,
}

struct Car {
    img: Texture2D,
    x: f32,
    y: f32,
    rect: Rect,
}

impl Car {
    fn update(&mut self, gambar: [Texture2D; 4]){
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

    fn reset(&mut self, gambar: [Texture2D; 4])
    {
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

struct Batu {
    img: Texture2D,
    x: f32,
    y: f32,
    rect: Rect,
}

impl Batu {
    fn update(&mut self, gambar: [Texture2D; 3]){
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

    fn reset(&mut self, gambar: [Texture2D; 3])
    {
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

struct Oli {
    img: Texture2D,
    x: f32,
    y: f32,
    rect: Rect,
}


impl Oli {
    fn update(&mut self){
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

    fn reset(&mut self)
    {
        self.x = rand::gen_range(82.0, 322.0);
        self.y = -250.0;
    }
}


impl Tampilkan for Oli {
    fn draw(&mut self) {
        draw_texture(self.img, self.x, self.y, WHITE);
        
    }
}


#[macroquad::main(config)]
async fn main() {
    // load font
    let f = load_ttf_font("texture/nasalization-rg.ttf").await.unwrap();

    // load sfx
    let car_crash = load_sound("texture/car_crash_sfx.ogg").await.unwrap();
    let driving_sfx = load_sound("texture/driving.ogg").await.unwrap();

    // load assets
    let car = load_texture("texture/car_blue_3.png").await.unwrap();
    let oil = load_texture("texture/oil.png").await.unwrap();
    let panah_img = load_texture("texture/arrow_white.png").await.unwrap();
    let tribune = load_texture("texture/tribune_overhang_striped.png").await.unwrap();
    let car0 = load_texture("texture/car_0.png").await.unwrap();
    let car1 = load_texture("texture/car_1.png").await.unwrap();
    let car2 = load_texture("texture/car_2.png").await.unwrap();
    let car3 = load_texture("texture/car_3.png").await.unwrap();
    let batu0 = load_texture("texture/rock_0.png").await.unwrap();
    let batu1 = load_texture("texture/rock_1.png").await.unwrap();
    let batu2 = load_texture("texture/rock_2.png").await.unwrap();
    
    // game variable
    let list_car : [Texture2D; 4] = [car0, car1, car2, car3];
    let list_batu : [Texture2D; 3] = [batu0, batu1, batu2];

    let mut npc = Car {
        img: list_car[rand::gen_range(0, 4)],
        x: rand::gen_range(82.0, 322.0),
        y: -250.0,
        rect: Default::default(),
    };

    let mut batu = Batu {
        img: list_batu[rand::gen_range(0, 3)],
        x: rand::gen_range(82.0, 322.0),
        y: -250.0,
        rect: Default::default(),
    };

    let mut player = Player{
        img: car, 
        x: screen_width()* 0.5, 
        y: screen_height() - 200.0,
        ax: 0.,
        vx: 0.,
        rect: Default::default(),
        rot: 0.,
        rot_speed: 0.,
        oil_img: oil,
        oil_rect: Rect::new(120., 40., 200., 20.),
    };

    let mut bensin = Oli {
        img: oil,
        x: rand::gen_range(82.0, 322.0),
        y: -250.0,
        rect: Default::default(),
    };

    let mut list_panah : Vec<Panah> = Vec::with_capacity(3);

    for i in 0..3 {
        let p = Panah{
            img: panah_img, 
            x: screen_width() * 0.5 - 50 as f32, 
            y: (i * 230 + 40) as f32,
        };

        list_panah.push(p);
    }
    
    let mut score = 0;
    let mut last_update = get_time();
    let mut current_scene = Scene::MENU;

    loop {
        clear_background(GRAY);
        
        // draw tribune
        draw_texture_ex(tribune, -100.0, 0.0, WHITE, DrawTextureParams{
            ..Default::default()
        });

        draw_texture_ex(tribune,screen_width() - 40.0, 0.0, WHITE, DrawTextureParams{
            ..Default::default()
        });

        match current_scene {
            Scene::MENU => {
                draw_text_ex("Car Game", screen_width()*0.5 - 90.0, screen_height() / 4.0, TextParams{
                    font: f,
                    font_size: 40,
                    ..Default::default()
                });
                draw_text_ex("Tekan \"SPACE\" untuk play", 70.0, screen_height() / 2.0, TextParams{
                    font: f,
                    font_size: 20,
                    color: BLACK,
                    ..Default::default()
                });

                draw_text_ex("created by aji mustofa @pepega90", 60., screen_height() - 20., TextParams{
                    font: f,
                    font_size: 15,
                    color: BLACK,
                    ..Default::default()
                });
                
                if is_key_pressed(KeyCode::Space) {
                    current_scene = Scene::PLAY;
                    play_sound(driving_sfx, PlaySoundParams{
                        looped: true,
                        ..Default::default()
                    });
                }
            },
            Scene::PLAY =>  {
                // draw panah

                for i in 0..3 {
                    list_panah[i].y += 1.0;
                    draw_texture(list_panah[i].img, list_panah[i].x, list_panah[i].y, WHITE);

                    if list_panah[i].y > screen_height() {
                        list_panah[i].y = -80.0;
                    }
                }

                if get_time() - last_update > 0.5 {
                    last_update = get_time();
                    score += 1;
                }

                // draw score
                draw_text_ex(format!("{}km", score).as_str(), 300., 30., TextParams{
                    font: f,
                    font_size: 20,
                    ..Default::default()
                });

                // draw oli
                bensin.update();

                // draw npc
                npc.update(list_car);

                // draw batu
                batu.update(list_batu);
                
                // draw player
                player.update();
                
                // check jika player kena batu atau kena mobil lain
                if player.rect.overlaps(&npc.rect) || player.rect.overlaps(&batu.rect) || player.oil_rect.w < 1. {
                    current_scene = Scene::GAMEOVER;
                    play_sound_once(car_crash);
                    stop_sound(driving_sfx);
                }

                // check jika player kena oli
                if player.rect.overlaps(&bensin.rect) && player.oil_rect.w < 200. {
                    player.oil_rect.w += 20.;
                    bensin.reset();
                }

            },
            Scene::GAMEOVER => {
                draw_text_ex("Game Over", 100., screen_height() / 4.0, TextParams{
                    font: f,
                    color: RED,
                    font_size: 35,
                    ..Default::default()
                });

                draw_text_ex(format!("Kamu berjalan sejauh {}km", score).as_str(), 60., screen_height() / 2., TextParams{
                    font: f,
                    color: WHITE,
                    font_size: 20,
                    ..Default::default()
                });

                draw_text_ex("Tekan \"R\" untuk restart", 50.0, screen_height() - 200., TextParams{
                    font: f,
                    color: BLACK,
                    font_size: 25,
                    ..Default::default()
                });


                if is_key_pressed(KeyCode::R) {
                    current_scene = Scene::PLAY;
                    player.reset();
                    npc.reset(list_car);
                    batu.reset(list_batu);
                    bensin.reset();
                    score = 0;
                    play_sound(driving_sfx, PlaySoundParams{
                        looped: true,
                        ..Default::default()
                    });
                }
            },
        }
       
        next_frame().await
    }
}