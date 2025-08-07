use std::error::Error;
use std::time::{Duration, Instant};
use rusty_audio::Audio;
use std::io;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal;
use crossterm::cursor::{Hide, Show}; 
use crossterm::ExecutableCommand;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use std::sync::mpsc;
use std::thread;
use my_invaders::{frame, render};
use my_invaders::frame::{new_frame, Drawable};
use my_invaders::player::Player;
use my_invaders::invaders::Invaders;


fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "audio/explode.wav");
    audio.add("lose", "audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");

    // Terminal Stuff
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    let _ = stdout.execute(Hide);

    audio.play("startup");

    // Render loop in a separate thread (probs not necessary for this kind of simple game, but good practice!)
    let (render_tx, render_rx) = mpsc::channel(); // crossbeam better for more complex things
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameLoop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now(); // instantly update instant
        let mut curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode:: Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameLoop;
                    },
                    
                    _ => {},
                }
            }
        }

        // updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & render
        
        /* player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);*/ 

        // Replacing earlier two lines with "Drawables"
        // kinda stupid for this, but would be more optimal for larger project with more drawable things to render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame)
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            audio.play("win");
            break 'gameLoop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameLoop;
        }
    }
    


    //clean up
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    //reversing stdout and terminal set-up to return to normal
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}