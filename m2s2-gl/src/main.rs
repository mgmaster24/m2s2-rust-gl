extern crate sdl2;

fn main() {
    let sdl2_context = match sdl2::init() {
        Ok(sdl) => sdl,
        Err(s) => {
            println!("Failed to initialize SDL2. Error: {}", s);
            return;
        }
    };

    let video_sub_sys = match sdl2_context.video() {
        Ok(vs) => vs,
        Err(s) => {
            println!("Failed to initialize the video sub system. Error: {}", s);
            return;
        }
    };

    let window = match video_sub_sys
        .window("M2S2 GL Lib Demo", 900, 700)
        .resizable()
        .build()
    {
        Ok(win) => win,
        Err(win_err) => {
            println!("Failed to create the window. Error: {}", win_err);
            return;
        }
    };

    let mut event_pump = match sdl2_context.event_pump() {
        Ok(ep) => ep,
        Err(s) => {
            println!("Failed to create the event pump. Error: {}", s);
            return;
        }
    };

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            };

            // Render Stuff!
        }
    }
}
