use sfml::{
    graphics::{
        RenderTarget, RenderWindow, Color, Texture, Sprite, Transformable, Text, Font, RectangleShape, Shape
    },
    window::{
        ContextSettings, Style, Event, Key
    },
    system::{
        Clock
    },
};
use rand::Rng;

enum Game {
    Start,
    Running,
    Died
}

fn main() {
    let context_settings = ContextSettings::default();

    let font = Font::from_file("src/手书体.ttf").unwrap();

    let mut w = RenderWindow::new((360, 640), "bird", Style::CLOSE, &context_settings);
//    w.set_framerate_limit(60);

    let mut rng = rand::thread_rng();

    let bird1 = Texture::from_file("src/bird1.png").unwrap();
    let mut bird_speed_x:f32 = 0.;
    let mut bird_speed_y:f32 = 0.;
    let mut bird_acc:f32 = 0.;
    let mut bird_x:f32 = 180.;
    let mut bird_y:f32 = 320.;
    let mut bird = Sprite::new();
    bird.set_origin((23.5, 19.5));
    bird.set_position((bird_x, bird_y));
    bird.set_texture(&bird1, false);

    let mut wall_first_up = RectangleShape::new();
    let mut wall_first_h:f32 = rng.gen_range(160.. 240) as f32;
    let wall_w:f32 = 80.;
    let  wall_speed_x:f32 = -0.0001;
    let mut wall_first_x = 360.;
    wall_first_up.set_size((wall_w, wall_first_h));
    wall_first_up.set_fill_color(Color::MAGENTA);
    wall_first_up.set_position((wall_first_x, 0.));

    let mut wall_first_down = RectangleShape::new();
    let wall_dis:f32 = 160.;
    wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
    wall_first_down.set_fill_color(Color::MAGENTA);
    wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));


    let mut wall_second_up = RectangleShape::new();
    let mut wall_second_h:f32 = rng.gen_range(160.. 240) as f32;
    let mut wall_second_x = 580.;
    wall_second_up.set_size((wall_w, wall_second_h));
    wall_second_up.set_fill_color(Color::MAGENTA);
    wall_second_up.set_position((wall_second_x, 0.));

    let mut wall_second_down = RectangleShape::new();
    wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
    wall_second_down.set_fill_color(Color::MAGENTA);
    wall_second_down.set_position((wall_second_x, wall_dis + wall_second_h));

    let mut statu = Game::Start;

    let mut clock = Clock::start();
    let mut clock2 = Clock::start();

    let mut fps = 0;
    let mut fps_txt = Text::new("", &font, 24);

    let mut cnt = 0;
    let mut lst = 2;
    let mut cnt_txt = Text::new("", &font, 36);
    cnt_txt.set_fill_color(Color::BLACK);

    while w.is_open() {
        while let Some(event) = w.poll_event() {
            match event {
                Event::Closed => w.close(),
                Event::KeyPressed{
                    code : Key::Space, ..
                } => {
                    match statu {
                        Game::Running => {
                            bird_speed_y = -0.0006;
                        },
                        Game::Start => {
                            bird_acc = 0.000000002;
                            wall_first_h = rng.gen_range(160.. 240) as f32;
                            wall_first_x = 360.;
                            wall_first_up.set_size((wall_w, wall_first_h));
                            wall_first_up.set_fill_color(Color::MAGENTA);
                            wall_first_up.set_position((wall_first_x, 0.));
                            wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
                            wall_first_down.set_fill_color(Color::MAGENTA);
                            wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));
                            wall_second_x = 580.;
                            wall_second_up.set_size((wall_w, wall_second_h));
                            wall_second_up.set_fill_color(Color::MAGENTA);
                            wall_second_up.set_position((wall_second_x, 0.));
                            wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
                            wall_second_down.set_fill_color(Color::MAGENTA);
                            wall_second_down.set_position((wall_second_x, wall_dis + wall_second_h));
                            cnt = 0;
                            lst = 2;
                            statu = Game::Running;
                        },
                        _ => ()
                    }
                },
                Event::KeyPressed{
                    code : Key::R, ..
                } => {
                    bird_acc = 0.000000002;
                    wall_first_h = rng.gen_range(160.. 240) as f32;
                    wall_first_x = 360.;
                    wall_first_up.set_size((wall_w, wall_first_h));
                    wall_first_up.set_fill_color(Color::MAGENTA);
                    wall_first_up.set_position((wall_first_x, 0.));
                    wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
                    wall_first_down.set_fill_color(Color::MAGENTA);
                    wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));
                    wall_second_x = 580.;
                    wall_second_up.set_size((wall_w, wall_second_h));
                    wall_second_up.set_fill_color(Color::MAGENTA);
                    wall_second_up.set_position((wall_second_x, 0.));
                    wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
                    wall_second_down.set_fill_color(Color::MAGENTA);
                    wall_second_down.set_position((wall_second_x, 160. + wall_second_h));
                    cnt = 0;
                    lst = 2;
                    statu = Game::Running;
                },
                _ => {}
            }
        }
        w.clear(Color::WHITE);

        let etime = clock.elapsed_time().as_microseconds() as f32;
        clock.restart();
        match statu {
            Game::Start => {
                w.draw(&bird);
            },
            Game::Running => {
                bird_x += bird_speed_x * etime;
                bird_y += bird_speed_y * etime + 0.5 * bird_acc * etime * etime;
                bird_speed_y += bird_acc * etime;
                bird.set_position((bird_x, bird_y));
                if bird_speed_y > 0. {
                    bird.set_rotation(20.);
                } else if bird_speed_y == 0. {
                    bird.set_rotation(0.);
                } else {
                    bird.set_rotation(-20.);
                }

                wall_first_x += wall_speed_x * etime;
                wall_first_up.set_position((wall_first_x, 0.));
                wall_first_down.set_position((wall_first_x, wall_dis + wall_first_h));

                wall_second_x += wall_speed_x * etime;
                wall_second_up.set_position((wall_second_x, 0.));
                wall_second_down.set_position((wall_second_x, wall_dis + wall_second_h));

                if wall_first_x <= -80. {
                    wall_first_x = 360.;
                    wall_first_h = rng.gen_range(0.. 480) as f32;
                    wall_first_up.set_size((wall_w, wall_first_h));
                    wall_first_down.set_size((wall_w, 640. - (wall_first_h + wall_dis)));
                }

                if wall_second_x <= -80. {
                    wall_second_x = 360.;
                    wall_second_h = rng.gen_range(0.. 480) as f32;
                    wall_second_up.set_size((wall_w, wall_second_h));
                    wall_second_down.set_size((wall_w, 640. - (wall_second_h + wall_dis)));
                }

                if bird_x >= wall_first_x && bird_x <= wall_first_x + wall_w {
                    if bird_y <= wall_first_h || bird_y >= wall_first_h + wall_dis {
                        statu = Game::Died;
                    }
                }
                if bird_x >= wall_second_x && bird_x <= wall_second_x + wall_w {
                    if bird_y <= wall_second_h || bird_y >= wall_second_h + wall_dis {
                        statu = Game::Died;
                    }
                }

                if lst == 1 {
                    if bird_x > wall_second_x + wall_w {
                        cnt += 1;
                        lst = 2;
                    }
                }

                if lst == 2 {
                    if bird_x > wall_first_x + wall_w {
                        cnt += 1;
                        lst = 1;
                    }
                }

                cnt_txt.set_string(&format!("{}", cnt));
                cnt_txt.set_position((180. - cnt_txt.character_size() as f32 / 2., 0.));

                w.draw(&bird);
                w.draw(&wall_first_up);
                w.draw(&wall_first_down);
                w.draw(&wall_second_up);
                w.draw(&wall_second_down);
                w.draw(&cnt_txt);

                if bird_y > 640. || bird_y < 0. {
                    statu = Game::Died;
                }

//                println!("{} {}", etime, bird_y);
            },
            Game::Died => {
                bird_x = 180.;
                bird_y = 320.;
                bird.set_rotation(0.);
                bird_speed_y = 0.;
                bird_speed_x = 0.;
                bird_acc = 0.;
                bird.set_position((bird_x, bird_y));
                w.draw(&bird);
                w.draw(&cnt_txt);
            }
        }
        
//        println!("{} {}", bird_x, bird_y);
        fps += 1;
        let etime2 = clock2.elapsed_time().as_microseconds();
        if etime2 > 1000000 {
            fps_txt.set_string(&format!("fps:{}", fps));
            fps_txt.set_fill_color(Color::BLACK);
            fps_txt.set_position((0., 0.));
            fps = 0;
            clock2.restart();
        }

        w.draw(&fps_txt);
        w.display();

    }
}
