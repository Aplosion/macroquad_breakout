use macroquad::prelude::*;

//should be self-explanitory

struct Brick {
    rect: Rect,
    durability: i32,
    color: Color,
}

//durability is how many hits a brick can take before it's destroyed.
//currently pointless since I haven't implemented balls or collision yet.

impl Brick {
    pub fn new(rect: Rect, durability: i32, color: Color) -> Self {
        Self {
            rect,
            durability,
            color,
        }
    }
}

#[macroquad::main("Breakout")]
async fn main() {
    //should probably just be an array, haven't learned how to use those yet.
    //size determined by window size so that it will in thoery work for any size of screen, full/windowed
    //but it's only updated once, so resizing the window while the program is running will break this
    //couldn't figure out an easy way to check if the window was resized, so this is long way around the
    //barn to hard-coding for the moment.
    let brick_size: Vec2 = Vec2::from_array([screen_width() / 12.0, screen_height() / 20.0]);
    let paddle_size: Vec2 = Vec2::from_array([screen_width() / 5.0, screen_height() / 20.0]);

    //I wanted the paddle to be able to wrap around the screen. Currently, this is done by making two
    //draw_rectangle calls every frame, with no checking if the rectangle is actaully on screen.
    //xpos_1 starts on-screen, xpos_2 starts offscreen. When the paddle wrapping the screen, the logic
    //makes sense and works as intended. When the paddle is fully on-screen and not touching the edge,
    //I'm just wasting a draw call every frame.
    let mut paddle_xpos_1 = 0.0;
    let mut paddle_xpos_2 = -screen_width();

    //commenting these out for now. I think it's technically faster to read from an int than to call
    //screen_width or screen_height, but I'm just going to leave the calls for now, for readability.
    //let sw =screen_width();
    //let sh =screen_height();

    //hopefully self-explanitory
    let padding_between_bricks = screen_height() / 50.0;
    let padding_under_paddle = screen_height() / 15.0;

    //wanted the ability to use online hex code palette things, or to at some point let users define colors
    //currently this and hex_to_color are mostly useless, but they'll help if I ever want those features.
    let a_color: u32 = 0xfffb00;

    let mut palette: Vec<Color> = Vec::new();
    let mut bricks: Vec<Brick> = Vec::new();

    //hardcoded for now, assumes default 600 x 800 window.
    let total_bricks = 60;
    let bricks_per_line = 10;

    //doesn't really matter what's in the palette.push() statement for the moment.
    //I'll do something with it, currently it's just there to help me debug where bricks
    //are vs where they should be. If there's a more elegant way to do this than to cast
    //c as an f32 every time I want to use it, I don't know what it is.
    for c in 0..total_bricks + 1 {
        palette.push(Color::new(
            (c as f32) * 1.0 / (total_bricks as f32),
            (c as f32) * 1.0 / (total_bricks as f32),
            (c as f32) * 1.0 / (total_bricks as f32),
            1.0,
        ));
    }

    //This should probably be a double for loop. It's a bit of a mess as currently written.
    //at current_line =0, put bricks at x: (padding_between_bricks*j+1) + (brick_size.x*j),
    //y: padding_between_bricks
    //otherwise, x: (padding_between_bricks*j+1) + (brick_size.x*j),
    //y: padding_between_bricks + (brick_size.y * current_line)

    //hopefully that makes sense, don't know how to explain it better.

    let mut current_line = 0;
    let mut j = 0;
    for i in 0..total_bricks {
        println!("i: {},j: {}, current_line: {}", i, j, current_line);
        current_line = i / (bricks_per_line);
        j = i - (current_line * bricks_per_line);

        //Planning on having a way to read from file to generate levels
        //if statement goes here to make level design possible
        //for example if current_char = 'B', normal block
        //if current_char = 'D', block that drops an extra ball power-up

        bricks.push(Brick::new(
            Rect::new(
                padding_between_bricks * (j as f32 + 1.0) + j as f32 * brick_size.x,
                padding_between_bricks * (current_line as f32 + 1.0)
                    + current_line as f32 * brick_size.y,
                brick_size.x,
                brick_size.y,
            ),
            2,
            palette[i],
        ));
    }

    loop {
        clear_background(hex_to_float(a_color));

        //Two unrelated things going on here
        //First, moving the paddle, labeling the one that's offset as orange and the one that
        //starts in frame as pink. In the final game, they will be the same color.
        draw_rectangle(
            paddle_xpos_2,
            screen_height() - padding_under_paddle,
            paddle_size.x,
            paddle_size.y,
            PINK,
        );

        draw_rectangle(
            paddle_xpos_1,
            screen_height() - padding_under_paddle,
            paddle_size.x,
            paddle_size.y,
            ORANGE,
        );
        //resetting the paddles if they get too far off screen.
        if paddle_xpos_1 >= screen_width() {
            paddle_xpos_1 -= screen_width() * 2.0;
        }
        if paddle_xpos_1 <= -screen_width() {
            paddle_xpos_1 += screen_width() * 2.0;
        }
        if paddle_xpos_2 >= screen_width() {
            paddle_xpos_2 -= screen_width() * 2.0;
        }
        if paddle_xpos_2 <= -screen_width() {
            paddle_xpos_2 += screen_width() * 2.0;
        }
        //for this to work, they need to be updated the same amount at the same time
        paddle_xpos_1 -= 1.0;
        paddle_xpos_2 -= 1.0;

        //reading the vec made earlier. Perhaps wants a helper function, this is a little
        //klunky
        for a in 0..bricks.len() {
            draw_rectangle(
                bricks[a].rect.x,
                bricks[a].rect.y,
                bricks[a].rect.w,
                bricks[a].rect.h,
                bricks[a].color,
            );
        }

        next_frame().await
    }
}

//uses a u32 to store the hex value, so that hex codes can be copied directly into my code.
//wastes the upper four bytes, but I think this is fine, any other way I could think of to do this
//didn't have the convinience and features I wanted. And it's much faster than messing with strings.
fn hex_to_float(a: u32) -> Color {
    let u32_as_bytes: [u8; 4] = a.to_be_bytes();
    Color::new(
        u32_as_bytes[1] as f32 / 255.0,
        u32_as_bytes[2] as f32 / 255.0,
        u32_as_bytes[3] as f32 / 255.0,
        1.0,
    )
}
