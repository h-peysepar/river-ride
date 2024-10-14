use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, size, Clear},
    ExecutableCommand, QueueableCommand,
};

struct World {
    player_c: u16,
    player_l: u16,
    maxc: u16,
    maxl: u16,
    map: Vec<(u16, u16)>,
}

fn draw(sc: &mut Stdout, world: &World) {
    sc.queue(Clear(terminal::ClearType::All));
    for line in 0..world.map.len() {
        sc.queue(MoveTo(0, line as u16));
        sc.queue(Print("+".repeat(world.map[line].0 as usize)));
        sc.queue(MoveTo(world.map[line].1, line as u16));
        sc.queue(Print("+".repeat((world.maxc - world.map[line].1) as usize)));

    };
    sc.queue(MoveTo(world.player_c, world.player_l));
    sc.queue(Print('P'));
    sc.flush();
}

fn main() -> std::io::Result<()> {
    // // using the macro
    // execute!(
    //     stdout(),
    //     SetForegroundColor(Color::Blue),
    //     SetBackgroundColor(Color::Red),
    //     Print("Styled text here."),
    //     ResetColor
    // )?;

    // or using functions
    let mut sc = stdout();
    enable_raw_mode();
    // cs.execute()
    // Hide();
    // .execute(MoveTo(10, 20))?
    // .execute(Print("Styled text here."))?
    // .execute(ResetColor)?;

    // init the game
    let (maxc, maxl) = size().unwrap();
    let mut world = World {
        player_c: maxc / 2,
        player_l: maxl - 1,
        maxc,
        maxl,
        map: vec![(maxc / 2 - 5, maxc / 2 + 5); maxl as usize]
    };
    sc.execute(Hide);
    loop {
        // ready and apply keyboards
        // physics

        if poll(Duration::from_millis(10))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('w') => {
                        if world.player_l > 1 {
                            world.player_l -= 1
                        }
                    }
                    KeyCode::Char('s') => {
                        if world.player_l < maxl - 1 {
                            world.player_l += 1
                        };
                    }
                    KeyCode::Char('a') => {
                        if world.player_c > 1 {
                            world.player_c -= 1
                        };
                    }
                    KeyCode::Char('d') => {
                        if world.player_c < maxc - 1 {
                            world.player_c += 1
                        };
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        //draw
        draw(&mut sc, &world);
        // break;
    }
    disable_raw_mode();
    sc.execute(Show);

    // Show();
    Ok(())
}
