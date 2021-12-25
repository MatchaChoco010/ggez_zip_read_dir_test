use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, Image};
use ggez::{filesystem, Context, ContextBuilder, GameResult};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");
    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    _images: Vec<Image>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let mut images = vec![];

        let exist_img = filesystem::exists(ctx, "/images/baboon.png");
        println!("Is exists /images/baboon.png : {}", exist_img);

        for p in filesystem::read_dir(ctx, "/images/").unwrap() {
            println!("{:?}", p);
            let image = Image::new(ctx, p).unwrap();
            images.push(image);
        }

        MyGame { _images: images }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        graphics::present(ctx)
    }
}
