use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Breed {
    GoldenRetriever,
    Labrador,
    Poodle,
    Beagle,
    Husky,
    Corgi,
    Bulldog,
    GermanShepherd,
}

impl Breed {
    fn as_str(&self) -> &str {
        match self {
            Breed::GoldenRetriever => "Golden Retriever",
            Breed::Labrador => "Labrador",
            Breed::Poodle => "Poodle",
            Breed::Beagle => "Beagle",
            Breed::Husky => "Husky",
            Breed::Corgi => "Corgi",
            Breed::Bulldog => "Bulldog",
            Breed::GermanShepherd => "German Shepherd",
        }
    }

    fn color(&self) -> Color {
        match self {
            Breed::GoldenRetriever => Color::from_rgba(218, 165, 32, 255),
            Breed::Labrador => Color::from_rgba(139, 90, 43, 255),
            Breed::Poodle => Color::from_rgba(255, 255, 255, 255),
            Breed::Beagle => Color::from_rgba(160, 82, 45, 255),
            Breed::Husky => Color::from_rgba(128, 128, 128, 255),
            Breed::Corgi => Color::from_rgba(210, 105, 30, 255),
            Breed::Bulldog => Color::from_rgba(205, 133, 63, 255),
            Breed::GermanShepherd => Color::from_rgba(101, 67, 33, 255),
        }
    }

    fn secondary_color(&self) -> Color {
        match self {
            Breed::GoldenRetriever => Color::from_rgba(255, 215, 100, 255),
            Breed::Labrador => Color::from_rgba(160, 110, 60, 255),
            Breed::Poodle => Color::from_rgba(240, 240, 240, 255),
            Breed::Beagle => Color::from_rgba(255, 255, 255, 255),
            Breed::Husky => Color::from_rgba(255, 255, 255, 255),
            Breed::Corgi => Color::from_rgba(255, 255, 255, 255),
            Breed::Bulldog => Color::from_rgba(180, 120, 70, 255),
            Breed::GermanShepherd => Color::from_rgba(50, 40, 20, 255),
        }
    }

    fn size_scale(&self) -> f32 {
        match self {
            Breed::GoldenRetriever => 1.0,
            Breed::Labrador => 1.0,
            Breed::Poodle => 0.9,
            Breed::Beagle => 0.85,
            Breed::Husky => 1.0,
            Breed::Corgi => 0.8,
            Breed::Bulldog => 0.95,
            Breed::GermanShepherd => 1.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Personality {
    Energetic,
    Calm,
    Playful,
    Shy,
    Friendly,
}

impl Personality {
    fn as_str(&self) -> &str {
        match self {
            Personality::Energetic => "Energetic",
            Personality::Calm => "Calm",
            Personality::Playful => "Playful",
            Personality::Shy => "Shy",
            Personality::Friendly => "Friendly",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DogAction {
    Idle,
    Playing,
    Eating,
    Napping,
    Grooming,
    Training,
}

#[derive(Debug, Clone)]
struct Dog {
    name: String,
    breed: Breed,
    personality: Personality,
    age: u8,
    energy: i32,
    happiness: i32,
    hunger: i32,
    cleanliness: i32,
    x: f32,
    y: f32,
    animation_offset: f32,
    display_energy: f32,
    display_happiness: f32,
    display_hunger: f32,
    display_cleanliness: f32,
    current_action: DogAction,
    action_timer: f32,
    bark_timer: f32,
    bark_text: String,
}

impl Dog {
    fn new(name: String, breed: Breed, personality: Personality, age: u8, x: f32, y: f32) -> Self {
        Dog {
            name,
            breed,
            personality,
            age,
            energy: 80,
            happiness: 70,
            hunger: 30,
            cleanliness: 100,
            x,
            y,
            animation_offset: rand::gen_range(0.0, std::f32::consts::PI * 2.0),
            display_energy: 80.0,
            display_happiness: 70.0,
            display_hunger: 30.0,
            display_cleanliness: 100.0,
            current_action: DogAction::Idle,
            action_timer: 0.0,
            bark_timer: 0.0,
            bark_text: String::new(),
        }
    }

    fn play(&mut self) {
        self.energy = (self.energy - 20).max(0);
        self.happiness = (self.happiness + 15).min(100);
        self.hunger = (self.hunger + 10).min(100);
        self.cleanliness = (self.cleanliness - 15).max(0);
        self.current_action = DogAction::Playing;
        self.action_timer = 2.0;
        self.bark("Woof! Woof!");
    }

    fn feed(&mut self) {
        self.hunger = (self.hunger - 40).max(0);
        self.happiness = (self.happiness + 10).min(100);
        self.energy = (self.energy + 5).min(100);
        self.current_action = DogAction::Eating;
        self.action_timer = 2.0;
        self.bark("Yum!");
    }

    fn nap(&mut self) {
        self.energy = (self.energy + 30).min(100);
        self.happiness = (self.happiness + 5).min(100);
        self.hunger = (self.hunger + 5).min(100);
        self.current_action = DogAction::Napping;
        self.action_timer = 3.0;
        self.bark("Zzz...");
    }

    fn groom(&mut self) {
        self.cleanliness = 100;
        self.happiness = (self.happiness + 8).min(100);
        self.current_action = DogAction::Grooming;
        self.action_timer = 2.0;
        self.bark("So clean!");
    }

    fn train(&mut self) {
        self.energy = (self.energy - 15).max(0);
        self.happiness = (self.happiness + 12).min(100);
        self.hunger = (self.hunger + 8).min(100);
        self.current_action = DogAction::Training;
        self.action_timer = 2.0;
        self.bark("I'm learning!");
    }

    fn bark(&mut self, text: &str) {
        self.bark_text = text.to_string();
        self.bark_timer = 2.5;
    }

    fn pass_time(&mut self) {
        self.energy = (self.energy - 5).max(0);
        self.hunger = (self.hunger + 8).min(100);
        self.happiness = (self.happiness - 3).max(0);
        self.cleanliness = (self.cleanliness - 5).max(0);
    }

    fn update(&mut self, dt: f32) {
        self.update_display_stats();
        if self.action_timer > 0.0 {
            self.action_timer -= dt;
            if self.action_timer <= 0.0 {
                self.current_action = DogAction::Idle;
            }
        }
        if self.bark_timer > 0.0 {
            self.bark_timer -= dt;
        }
    }

    fn update_display_stats(&mut self) {
        let lerp_speed = 0.1;
        self.display_energy += (self.energy as f32 - self.display_energy) * lerp_speed;
        self.display_happiness += (self.happiness as f32 - self.display_happiness) * lerp_speed;
        self.display_hunger += (self.hunger as f32 - self.display_hunger) * lerp_speed;
        self.display_cleanliness +=
            (self.cleanliness as f32 - self.display_cleanliness) * lerp_speed;
    }

    fn get_mood(&self) -> &str {
        if self.happiness > 80 {
            "Very Happy!"
        } else if self.happiness > 60 {
            "Content"
        } else if self.happiness > 40 {
            "Okay"
        } else if self.happiness > 20 {
            "Unhappy"
        } else {
            "Very Sad"
        }
    }

    fn needs_attention(&self) -> Vec<&str> {
        let mut needs = Vec::new();
        if self.hunger > 70 {
            needs.push("Hungry!");
        }
        if self.energy < 30 {
            needs.push("Tired");
        }
        if self.cleanliness < 40 {
            needs.push("Dirty");
        }
        if self.happiness < 40 {
            needs.push("Sad");
        }
        needs
    }

    fn draw(&self, time: f32) {
        let scale = self.breed.size_scale();
        let bounce = match self.current_action {
            DogAction::Playing => (time * 8.0 + self.animation_offset).sin() * 8.0,
            DogAction::Eating => (time * 4.0).sin() * 1.5,
            DogAction::Napping => 0.0,
            _ => (time * 2.0 + self.animation_offset).sin() * 3.0,
        };
        let y_pos = self.y + bounce;

        // Draw shadow
        draw_ellipse(
            self.x,
            self.y + 45.0 * scale,
            35.0 * scale,
            15.0 * scale,
            0.0,
            Color::from_rgba(0, 0, 0, 50),
        );

        // Draw body
        let body_size = 25.0 * scale;
        draw_circle(self.x, y_pos + 20.0 * scale, body_size, self.breed.color());

        // Breed-specific body patterns
        match self.breed {
            Breed::Beagle | Breed::Husky | Breed::Corgi => {
                // White belly
                draw_circle(
                    self.x,
                    y_pos + 25.0 * scale,
                    body_size * 0.7,
                    self.breed.secondary_color(),
                );
            }
            Breed::Poodle => {
                // Fluffy poodle cuts
                draw_circle(self.x - 15.0 * scale, y_pos + 20.0 * scale, 10.0 * scale, self.breed.color());
                draw_circle(self.x + 15.0 * scale, y_pos + 20.0 * scale, 10.0 * scale, self.breed.color());
            }
            _ => {}
        }

        // Draw head
        let head_size = 20.0 * scale;
        draw_circle(self.x, y_pos, head_size, self.breed.color());

        // Breed-specific head patterns
        match self.breed {
            Breed::Husky => {
                // Face mask pattern
                draw_circle(self.x - 8.0 * scale, y_pos - 2.0 * scale, 7.0 * scale, WHITE);
                draw_circle(self.x + 8.0 * scale, y_pos - 2.0 * scale, 7.0 * scale, WHITE);
                draw_circle(self.x, y_pos + 8.0 * scale, 10.0 * scale, WHITE);
            }
            Breed::Beagle => {
                // Beagle face marking
                draw_circle(self.x, y_pos, head_size * 0.8, WHITE);
            }
            _ => {}
        }

        // Draw breed-specific ears
        match self.breed {
            Breed::Beagle | Breed::Labrador | Breed::GoldenRetriever => {
                // Floppy ears
                draw_ellipse(
                    self.x - 18.0 * scale,
                    y_pos + 5.0 * scale,
                    8.0 * scale,
                    15.0 * scale,
                    0.3,
                    self.breed.color(),
                );
                draw_ellipse(
                    self.x + 18.0 * scale,
                    y_pos + 5.0 * scale,
                    8.0 * scale,
                    15.0 * scale,
                    -0.3,
                    self.breed.color(),
                );
            }
            Breed::Husky | Breed::GermanShepherd => {
                // Pointy ears
                draw_triangle(
                    vec2(self.x - 15.0 * scale, y_pos - 15.0 * scale),
                    vec2(self.x - 18.0 * scale, y_pos - 5.0 * scale),
                    vec2(self.x - 12.0 * scale, y_pos - 5.0 * scale),
                    self.breed.color(),
                );
                draw_triangle(
                    vec2(self.x + 15.0 * scale, y_pos - 15.0 * scale),
                    vec2(self.x + 12.0 * scale, y_pos - 5.0 * scale),
                    vec2(self.x + 18.0 * scale, y_pos - 5.0 * scale),
                    self.breed.color(),
                );
            }
            Breed::Poodle => {
                // Fluffy round ears
                draw_circle(self.x - 15.0 * scale, y_pos - 5.0 * scale, 10.0 * scale, self.breed.color());
                draw_circle(self.x + 15.0 * scale, y_pos - 5.0 * scale, 10.0 * scale, self.breed.color());
            }
            Breed::Corgi => {
                // Large pointed ears
                draw_triangle(
                    vec2(self.x - 12.0 * scale, y_pos - 18.0 * scale),
                    vec2(self.x - 16.0 * scale, y_pos - 2.0 * scale),
                    vec2(self.x - 8.0 * scale, y_pos - 2.0 * scale),
                    self.breed.color(),
                );
                draw_triangle(
                    vec2(self.x + 12.0 * scale, y_pos - 18.0 * scale),
                    vec2(self.x + 8.0 * scale, y_pos - 2.0 * scale),
                    vec2(self.x + 16.0 * scale, y_pos - 2.0 * scale),
                    self.breed.color(),
                );
            }
            Breed::Bulldog => {
                // Small folded ears
                draw_circle(self.x - 15.0 * scale, y_pos, 6.0 * scale, self.breed.color());
                draw_circle(self.x + 15.0 * scale, y_pos, 6.0 * scale, self.breed.color());
            }
        }

        // Draw eyes
        let eye_color = if self.happiness > 60 { BLACK } else { GRAY };
        let eye_size = match self.current_action {
            DogAction::Napping => 1.5,
            _ => 3.0,
        };
        draw_circle(self.x - 7.0 * scale, y_pos - 3.0 * scale, eye_size * scale, eye_color);
        draw_circle(self.x + 7.0 * scale, y_pos - 3.0 * scale, eye_size * scale, eye_color);

        // Add eye shine
        if self.current_action != DogAction::Napping {
            draw_circle(
                self.x - 6.0 * scale,
                y_pos - 4.0 * scale,
                1.5 * scale,
                WHITE,
            );
            draw_circle(
                self.x + 8.0 * scale,
                y_pos - 4.0 * scale,
                1.5 * scale,
                WHITE,
            );
        }

        // Draw nose
        draw_circle(self.x, y_pos + 5.0 * scale, 4.0 * scale, BLACK);

        // Draw mouth (smile if happy)
        if self.happiness > 60 && self.current_action != DogAction::Eating {
            draw_line(
                self.x - 8.0 * scale,
                y_pos + 8.0 * scale,
                self.x,
                y_pos + 12.0 * scale,
                2.0,
                BLACK,
            );
            draw_line(
                self.x,
                y_pos + 12.0 * scale,
                self.x + 8.0 * scale,
                y_pos + 8.0 * scale,
                2.0,
                BLACK,
            );
        }

        // Draw eating mouth
        if self.current_action == DogAction::Eating {
            let mouth_open = ((time * 10.0).sin() * 0.5 + 0.5) * 5.0;
            draw_circle(self.x, y_pos + 10.0 * scale, 3.0 + mouth_open, Color::from_rgba(50, 25, 25, 255));
        }

        // Draw legs
        let leg_width = 8.0 * scale;
        let leg_height = 15.0 * scale;
        if self.breed == Breed::Corgi {
            // Corgi has short legs
            let corgi_leg_height = 8.0 * scale;
            draw_rectangle(
                self.x - 15.0 * scale,
                y_pos + 35.0 * scale,
                leg_width,
                corgi_leg_height,
                self.breed.color(),
            );
            draw_rectangle(
                self.x + 7.0 * scale,
                y_pos + 35.0 * scale,
                leg_width,
                corgi_leg_height,
                self.breed.color(),
            );
        } else {
            draw_rectangle(
                self.x - 15.0 * scale,
                y_pos + 35.0 * scale,
                leg_width,
                leg_height,
                self.breed.color(),
            );
            draw_rectangle(
                self.x + 7.0 * scale,
                y_pos + 35.0 * scale,
                leg_width,
                leg_height,
                self.breed.color(),
            );
        }

        // Draw tail wagging
        let tail_angle = match self.current_action {
            DogAction::Playing => (time * 10.0 + self.animation_offset).sin() * 0.6,
            DogAction::Eating => (time * 8.0 + self.animation_offset).sin() * 0.5,
            DogAction::Napping => 0.0,
            _ => (time * 5.0 + self.animation_offset).sin() * 0.3,
        };
        let tail_x = self.x + 20.0 * scale + tail_angle * 15.0;

        // Breed-specific tails
        match self.breed {
            Breed::Husky | Breed::GermanShepherd => {
                // Bushy tail
                draw_circle(tail_x, y_pos + 12.0 * scale, 8.0 * scale, self.breed.color());
                draw_circle(tail_x + 5.0 * scale, y_pos + 8.0 * scale, 7.0 * scale, self.breed.color());
            }
            Breed::Corgi => {
                // Short fluffy tail
                draw_circle(tail_x - 5.0 * scale, y_pos + 15.0 * scale, 5.0 * scale, self.breed.color());
            }
            _ => {
                draw_circle(tail_x, y_pos + 15.0 * scale, 6.0 * scale, self.breed.color());
            }
        }

        // Draw action-specific effects
        match self.current_action {
            DogAction::Grooming => {
                // Sparkles
                for i in 0..5 {
                    let sparkle_angle = time * 3.0 + i as f32 * 1.2;
                    let sparkle_dist = 30.0 + (time * 2.0 + i as f32).sin() * 5.0;
                    let sx = self.x + sparkle_angle.cos() * sparkle_dist;
                    let sy = y_pos + sparkle_angle.sin() * sparkle_dist;
                    let sparkle_size = 2.0 + (time * 5.0 + i as f32).sin().abs() * 2.0;
                    draw_circle(sx, sy, sparkle_size, YELLOW);
                    draw_circle(sx, sy, sparkle_size * 0.5, WHITE);
                }
            }
            DogAction::Training => {
                // Concentration lines
                for i in 0..3 {
                    let line_offset = i as f32 * 8.0 - 8.0;
                    draw_line(
                        self.x + line_offset,
                        y_pos - 40.0,
                        self.x + line_offset,
                        y_pos - 50.0,
                        2.0,
                        SKYBLUE,
                    );
                }
            }
            DogAction::Napping => {
                // Zzz animation
                let z_offset = (time * 2.0).sin() * 3.0;
                draw_text_with_outline("Z", self.x + 25.0, y_pos - 20.0 + z_offset, 20.0, WHITE, BLACK);
                draw_text_with_outline("z", self.x + 35.0, y_pos - 30.0 + z_offset * 0.7, 16.0, WHITE, BLACK);
                draw_text_with_outline("z", self.x + 42.0, y_pos - 38.0 + z_offset * 0.5, 12.0, WHITE, BLACK);
            }
            _ => {}
        }

        // Draw bark speech bubble
        if self.bark_timer > 0.0 {
            let bubble_alpha = (self.bark_timer / 2.5 * 255.0).min(255.0) as u8;
            let bubble_y = y_pos - 50.0 - (2.5 - self.bark_timer) * 20.0;

            // Measure text
            let text_dims = measure_text(&self.bark_text, None, 18, 1.0);
            let bubble_width = text_dims.width + 20.0;
            let bubble_height = 30.0;
            let bubble_x = self.x - bubble_width / 2.0;

            // Draw speech bubble with rounded corners
            draw_rounded_rect(
                bubble_x,
                bubble_y,
                bubble_width,
                bubble_height,
                15.0,
                Color::from_rgba(255, 255, 255, bubble_alpha),
            );
            draw_rounded_rect_lines(
                bubble_x,
                bubble_y,
                bubble_width,
                bubble_height,
                15.0,
                3.0,
                Color::from_rgba(100, 100, 100, bubble_alpha),
            );

            // Draw bubble tail
            draw_triangle(
                vec2(self.x, bubble_y + bubble_height),
                vec2(self.x - 8.0, bubble_y + bubble_height + 10.0),
                vec2(self.x + 8.0, bubble_y + bubble_height),
                Color::from_rgba(255, 255, 255, bubble_alpha),
            );

            // Draw text
            let text_color = Color::from_rgba(50, 50, 50, bubble_alpha);
            draw_text(
                &self.bark_text,
                bubble_x + 10.0,
                bubble_y + 20.0,
                18.0,
                text_color,
            );
        }

        // Draw name tag with outline
        draw_text_with_outline(&self.name, self.x - measure_text(&self.name, None, 16, 1.0).width / 2.0, y_pos - 30.0, 16.0, WHITE, BLACK);
    }
}

fn draw_text_with_outline(text: &str, x: f32, y: f32, font_size: f32, color: Color, outline_color: Color) {
    // Draw outline
    for dx in [-1.0, 0.0, 1.0].iter() {
        for dy in [-1.0, 0.0, 1.0].iter() {
            if *dx != 0.0 || *dy != 0.0 {
                draw_text(text, x + dx, y + dy, font_size, outline_color);
            }
        }
    }
    // Draw main text
    draw_text(text, x, y, font_size, color);
}

fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    // Main rectangle
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);

    // Corners
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

fn draw_rounded_rect_lines(x: f32, y: f32, w: f32, h: f32, r: f32, thickness: f32, color: Color) {
    // Top and bottom lines
    draw_rectangle(x + r, y - thickness / 2.0, w - 2.0 * r, thickness, color);
    draw_rectangle(x + r, y + h - thickness / 2.0, w - 2.0 * r, thickness, color);

    // Left and right lines
    draw_rectangle(x - thickness / 2.0, y + r, thickness, h - 2.0 * r, color);
    draw_rectangle(x + w - thickness / 2.0, y + r, thickness, h - 2.0 * r, color);

    // Corners
    draw_circle_lines(x + r, y + r, r, thickness, color);
    draw_circle_lines(x + w - r, y + r, r, thickness, color);
    draw_circle_lines(x + r, y + h - r, r, thickness, color);
    draw_circle_lines(x + w - r, y + h - r, r, thickness, color);
}

#[derive(Clone)]
struct DogReport {
    name: String,
    breed: Breed,
    happiness: i32,
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    DayReport,
}

struct Daycare {
    dogs: Vec<Dog>,
    day: u32,
    time: u32,
    score: i32,
    selected_dog: Option<usize>,
    message: String,
    message_timer: f32,
    state: GameState,
    day_reports: Vec<DogReport>,
    day_score: i32,
}

impl Daycare {
    fn new() -> Self {
        Daycare {
            dogs: Vec::new(),
            day: 1,
            time: 8,
            score: 0,
            selected_dog: None,
            message: String::new(),
            message_timer: 0.0,
            state: GameState::Playing,
            day_reports: Vec::new(),
            day_score: 0,
        }
    }

    fn add_dog(&mut self, dog: Dog) {
        self.show_message(&format!("{} arrived!", dog.name));
        self.dogs.push(dog);
    }

    fn show_message(&mut self, msg: &str) {
        self.message = msg.to_string();
        self.message_timer = 3.0;
    }

    fn pass_time(&mut self, hours: u32) {
        for _ in 0..hours {
            self.time += 1;
            if self.time >= 18 {
                self.day_reports = self
                    .dogs
                    .iter()
                    .map(|d| DogReport {
                        name: d.name.clone(),
                        breed: d.breed,
                        happiness: d.happiness,
                    })
                    .collect();

                self.day_score = self.dogs.iter().map(|d| d.happiness).sum();
                self.score += self.day_score;
                self.state = GameState::DayReport;
                return;
            } else {
                for dog in &mut self.dogs {
                    dog.pass_time();
                }
            }
        }
    }

    fn start_new_day(&mut self) {
        self.dogs.clear();
        self.day += 1;
        self.time = 8;
        self.selected_dog = None;
        self.state = GameState::Playing;
        self.spawn_new_dogs();
    }

    fn spawn_new_dogs(&mut self) {
        use Breed::*;
        use Personality::*;

        let dogs_data = vec![
            ("Max", GoldenRetriever, Friendly, 3),
            ("Bella", Labrador, Playful, 2),
            ("Charlie", Beagle, Energetic, 4),
            ("Luna", Husky, Energetic, 1),
            ("Cooper", Corgi, Calm, 5),
            ("Daisy", Poodle, Shy, 2),
            ("Rocky", Bulldog, Calm, 6),
            ("Sadie", GermanShepherd, Friendly, 3),
        ];

        let num_dogs = (self.day % 5 + 2).min(4) as usize;
        let start_idx = ((self.day - 1) * 2) as usize % dogs_data.len();

        let positions = vec![
            (200.0, 300.0),
            (400.0, 350.0),
            (600.0, 320.0),
            (350.0, 450.0),
        ];

        for i in 0..num_dogs {
            let idx = (start_idx + i) % dogs_data.len();
            let (name, breed, personality, age) = &dogs_data[idx];
            let (x, y) = positions[i % positions.len()];
            self.add_dog(Dog::new(
                name.to_string(),
                *breed,
                *personality,
                *age,
                x,
                y,
            ));
        }
    }
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    color: Color,
    size: f32,
}

impl Particle {
    fn new(x: f32, y: f32, color: Color) -> Self {
        Particle {
            x,
            y,
            vx: rand::gen_range(-2.0, 2.0),
            vy: rand::gen_range(-4.0, -1.0),
            life: 1.0,
            max_life: 1.0,
            color,
            size: rand::gen_range(3.0, 8.0),
        }
    }

    fn update(&mut self, dt: f32) {
        self.x += self.vx;
        self.y += self.vy;
        self.vy += 0.2;
        self.life -= dt;
    }

    fn draw(&self) {
        let alpha = (self.life / self.max_life * 255.0) as u8;
        let mut color = self.color;
        color.a = alpha as f32 / 255.0;
        draw_circle(self.x, self.y, self.size, color);
    }

    fn is_alive(&self) -> bool {
        self.life > 0.0
    }
}

struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    color: Color,
    hover_color: Color,
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: &str, color: Color) -> Self {
        let hover_color = Color::from_rgba(
            ((color.r as f32 * 1.2).min(255.0)) as u8,
            ((color.g as f32 * 1.2).min(255.0)) as u8,
            ((color.b as f32 * 1.2).min(255.0)) as u8,
            255,
        );
        Button {
            x,
            y,
            width,
            height,
            text: text.to_string(),
            color,
            hover_color,
        }
    }

    fn draw(&self, time: f32) {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height;

        let pulse = (time * 3.0).sin() * 0.05 + 1.0;
        let scale = if is_hovered { pulse } else { 1.0 };

        let draw_x = self.x - (self.width * scale - self.width) / 2.0;
        let draw_y = self.y - (self.height * scale - self.height) / 2.0;
        let draw_width = self.width * scale;
        let draw_height = self.height * scale;

        let draw_color = if is_hovered {
            self.hover_color
        } else {
            self.color
        };

        // Glow effect when hovered
        if is_hovered {
            draw_rounded_rect(
                draw_x - 4.0,
                draw_y - 4.0,
                draw_width + 8.0,
                draw_height + 8.0,
                20.0,
                Color::from_rgba(255, 255, 255, 100),
            );
        }

        // Draw button with rounded corners
        draw_rounded_rect(draw_x, draw_y, draw_width, draw_height, 15.0, draw_color);
        draw_rounded_rect_lines(draw_x, draw_y, draw_width, draw_height, 15.0, 3.0, WHITE);

        // Draw text with shadow
        let text_dims = measure_text(&self.text, None, 22, 1.0);
        let text_x = draw_x + (draw_width - text_dims.width) / 2.0;
        let text_y = draw_y + (draw_height + text_dims.height) / 2.0;

        draw_text_with_outline(&self.text, text_x, text_y, 22.0, WHITE, BLACK);
    }

    fn is_clicked(&self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            mouse_x >= self.x
                && mouse_x <= self.x + self.width
                && mouse_y >= self.y
                && mouse_y <= self.y + self.height
        } else {
            false
        }
    }
}

fn draw_animated_stat_bar(
    x: f32,
    y: f32,
    width: f32,
    value: f32,
    max: f32,
    color: Color,
    label: &str,
    time: f32,
) {
    let fill_width = (value / max) * width;

    // Background with rounded corners
    draw_rounded_rect(x, y, width, 24.0, 12.0, Color::from_rgba(40, 40, 40, 255));

    // Animated shimmer effect
    let shimmer = (time * 2.0 + x / 100.0).sin() * 0.1 + 0.9;
    let shimmer_color = Color::from_rgba(
        (color.r as f32 * shimmer) as u8,
        (color.g as f32 * shimmer) as u8,
        (color.b as f32 * shimmer) as u8,
        255,
    );

    // Fill bar with rounded corners
    if fill_width > 24.0 {
        draw_rounded_rect(x, y, fill_width, 24.0, 12.0, shimmer_color);
    } else if fill_width > 0.0 {
        draw_circle(x + 12.0, y + 12.0, 12.0, shimmer_color);
    }

    // Glossy effect
    if fill_width > 0.0 {
        draw_rounded_rect(
            x,
            y,
            fill_width.min(width),
            10.0,
            12.0,
            Color::from_rgba(255, 255, 255, 50),
        );
    }

    // Border with rounded corners
    draw_rounded_rect_lines(x, y, width, 24.0, 12.0, 3.0, WHITE);

    // Label with shadow
    draw_text_with_outline(label, x, y - 5.0, 18.0, WHITE, BLACK);

    // Value with shadow
    let value_text = format!("{:.0}/{}", value, max);
    draw_text_with_outline(&value_text, x + width + 10.0, y + 18.0, 18.0, WHITE, BLACK);
}

fn draw_floating_particles(particles: &[Particle]) {
    for particle in particles {
        particle.draw();
    }
}

fn draw_background(time: f32) {
    // Animated sky gradient
    for i in 0..50 {
        let y = i as f32 * 10.0;
        let alpha = 100 - (i * 2);
        let wave = (time * 0.5 + i as f32 * 0.1).sin() * 10.0;
        draw_rectangle(
            0.0,
            y,
            800.0 + wave,
            10.0,
            Color::from_rgba(135, 206, 235, alpha as u8),
        );
    }

    // Animated grass
    draw_rectangle(0.0, 500.0, 800.0, 220.0, Color::from_rgba(34, 139, 34, 255));

    // Grass blades animation
    for i in 0..20 {
        let x = i as f32 * 40.0 + (time * 2.0 + i as f32).sin() * 2.0;
        let y = 500.0 + (time + i as f32 * 0.5).sin() * 3.0;
        draw_line(x, y + 20.0, x, y, 2.0, Color::from_rgba(50, 180, 50, 255));
    }
}

fn draw_day_report(daycare: &Daycare, time: f32) {
    // Semi-transparent overlay
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        Color::from_rgba(0, 0, 0, 200),
    );

    // Animated report panel
    let panel_width = 600.0;
    let panel_height = 500.0;
    let panel_x = (SCREEN_WIDTH - panel_width) / 2.0;
    let panel_y = (SCREEN_HEIGHT - panel_height) / 2.0 + (time * 2.0).sin() * 5.0;

    // Panel shadow
    draw_rounded_rect(
        panel_x + 5.0,
        panel_y + 5.0,
        panel_width,
        panel_height,
        30.0,
        Color::from_rgba(0, 0, 0, 150),
    );

    // Panel background with gradient
    draw_rounded_rect(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        30.0,
        Color::from_rgba(101, 67, 33, 255),
    );
    draw_rounded_rect(
        panel_x,
        panel_y,
        panel_width,
        100.0,
        30.0,
        Color::from_rgba(139, 90, 43, 255),
    );

    // Border with glow
    draw_rounded_rect_lines(panel_x, panel_y, panel_width, panel_height, 30.0, 4.0, GOLD);

    // Title with animation
    let title = format!("Day {} Complete!", daycare.day);
    let title_dims = measure_text(&title, None, 40, 1.0);
    draw_text_with_outline(
        &title,
        panel_x + (panel_width - title_dims.width) / 2.0,
        panel_y + 60.0,
        40.0,
        YELLOW,
        Color::from_rgba(100, 50, 0, 255),
    );

    // Score
    let score_text = format!("Day Score: {} points", daycare.day_score);
    let score_dims = measure_text(&score_text, None, 24, 1.0);
    draw_text_with_outline(
        &score_text,
        panel_x + (panel_width - score_dims.width) / 2.0,
        panel_y + 100.0,
        24.0,
        WHITE,
        BLACK,
    );

    // Divider
    draw_line(
        panel_x + 20.0,
        panel_y + 120.0,
        panel_x + panel_width - 20.0,
        panel_y + 120.0,
        3.0,
        GOLD,
    );

    // Dog reports
    draw_text_with_outline(
        "Dog Happiness Report:",
        panel_x + 30.0,
        panel_y + 150.0,
        22.0,
        YELLOW,
        BLACK,
    );

    for (i, report) in daycare.day_reports.iter().enumerate() {
        let y = panel_y + 185.0 + i as f32 * 45.0;
        let bounce = (time * 3.0 + i as f32 * 0.5).sin() * 2.0;

        // Dog name and breed
        draw_text_with_outline(
            &format!("{} ({})", report.name, report.breed.as_str()),
            panel_x + 30.0,
            y + bounce,
            18.0,
            WHITE,
            BLACK,
        );

        // Happiness bar
        let bar_width = 180.0;
        let bar_x = panel_x + 310.0;
        draw_animated_stat_bar(
            bar_x,
            y - 15.0 + bounce,
            bar_width,
            report.happiness as f32,
            100.0,
            YELLOW,
            "",
            time,
        );

        // Emoji based on happiness
        let emoji = if report.happiness > 80 {
            "😊"
        } else if report.happiness > 60 {
            "🙂"
        } else if report.happiness > 40 {
            "😐"
        } else {
            "☹️"
        };
        draw_text(emoji, panel_x + 510.0, y + bounce, 28.0, WHITE);
    }

    // Total score
    let total_text = format!("Total Score: {}", daycare.score);
    let total_dims = measure_text(&total_text, None, 30, 1.0);
    draw_text_with_outline(
        &total_text,
        panel_x + (panel_width - total_dims.width) / 2.0,
        panel_y + panel_height - 70.0,
        30.0,
        GOLD,
        BLACK,
    );

    // Continue button
    let button_y = panel_y + panel_height - 45.0;
    let button = Button::new(
        panel_x + 200.0,
        button_y,
        200.0,
        40.0,
        "Start Next Day",
        GREEN,
    );
    button.draw(time);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dog Daycare Simulator".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut daycare = Daycare::new();
    daycare.spawn_new_dogs();

    let action_buttons = vec![
        Button::new(820.0, 150.0, 200.0, 55.0, "🎾 Play (1h)", BLUE),
        Button::new(1040.0, 150.0, 200.0, 55.0, "🍖 Feed (1h)", GREEN),
        Button::new(820.0, 220.0, 200.0, 55.0, "😴 Nap (2h)", PURPLE),
        Button::new(1040.0, 220.0, 200.0, 55.0, "🛁 Groom (1h)", PINK),
        Button::new(820.0, 290.0, 200.0, 55.0, "📚 Train (1h)", ORANGE),
        Button::new(820.0, 370.0, 420.0, 55.0, "⏰ Pass Time (1h)", DARKGRAY),
    ];

    let mut particles: Vec<Particle> = Vec::new();
    let mut background_particles: Vec<Particle> = Vec::new();
    let mut particle_spawn_timer = 0.0;

    loop {
        clear_background(Color::from_rgba(144, 238, 144, 255));

        let time = get_time() as f32;
        let dt = get_frame_time();

        // Spawn background particles
        particle_spawn_timer += dt;
        if particle_spawn_timer > 0.5 {
            particle_spawn_timer = 0.0;
            if rand::gen_range(0, 3) == 0 {
                background_particles.push(Particle {
                    x: rand::gen_range(0.0, 800.0),
                    y: SCREEN_HEIGHT,
                    vx: 0.0,
                    vy: -1.0,
                    life: 5.0,
                    max_life: 5.0,
                    color: Color::from_rgba(255, 200, 200, 100),
                    size: 5.0,
                });
            }
        }

        // Update background particles
        background_particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });

        // Update particles
        particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });

        if daycare.state == GameState::DayReport {
            draw_day_report(&daycare, time);

            let panel_width = 600.0;
            let panel_height = 500.0;
            let panel_x = (SCREEN_WIDTH - panel_width) / 2.0;
            let panel_y = (SCREEN_HEIGHT - panel_height) / 2.0 + (time * 2.0).sin() * 5.0;
            let button_y = panel_y + panel_height - 45.0;

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if mouse_x >= panel_x + 200.0
                    && mouse_x <= panel_x + 400.0
                    && mouse_y >= button_y
                    && mouse_y <= button_y + 40.0
                {
                    daycare.start_new_day();
                }
            }
        } else {
            draw_background(time);
            draw_floating_particles(&background_particles);

            // Draw title with animation and shadow
            let title_scale = 1.0 + (time * 2.0).sin() * 0.02;
            draw_text_with_outline(
                "🐕 Dog Daycare Simulator 🐕",
                15.0,
                45.0,
                40.0 * title_scale,
                Color::from_rgba(255, 220, 180, 255),
                DARKBROWN,
            );

            // Draw day, time, and score with bubble background
            let info_text = format!(
                "Day: {} | Time: {}:00 | Score: {}",
                daycare.day, daycare.time, daycare.score
            );
            let info_dims = measure_text(&info_text, None, 24, 1.0);
            draw_rounded_rect(
                15.0,
                55.0,
                info_dims.width + 20.0,
                35.0,
                17.0,
                Color::from_rgba(101, 67, 33, 220),
            );
            draw_text_with_outline(&info_text, 25.0, 80.0, 24.0, WHITE, BLACK);

            // Draw message with bubble
            if daycare.message_timer > 0.0 {
                daycare.message_timer -= dt;
                let msg_dims = measure_text(&daycare.message, None, 28, 1.0);
                let msg_x = (800.0 - msg_dims.width) / 2.0 - 15.0;
                let slide_offset = if daycare.message_timer > 2.5 {
                    (3.0 - daycare.message_timer) * 100.0
                } else {
                    0.0
                };

                draw_rounded_rect(
                    msg_x,
                    105.0 - slide_offset,
                    msg_dims.width + 30.0,
                    45.0,
                    22.0,
                    Color::from_rgba(255, 200, 50, 240),
                );
                draw_rounded_rect_lines(
                    msg_x,
                    105.0 - slide_offset,
                    msg_dims.width + 30.0,
                    45.0,
                    22.0,
                    3.0,
                    Color::from_rgba(200, 150, 0, 255),
                );
                draw_text_with_outline(
                    &daycare.message,
                    msg_x + 15.0,
                    135.0 - slide_offset,
                    28.0,
                    WHITE,
                    Color::from_rgba(100, 50, 0, 255),
                );
            }

            // Update and draw dogs
            for (i, dog) in daycare.dogs.iter_mut().enumerate() {
                dog.update(dt);
                dog.draw(time);

                // Highlight selected dog
                if Some(i) == daycare.selected_dog {
                    let pulse = (time * 4.0).sin() * 3.0 + 45.0;
                    draw_circle_lines(dog.x, dog.y + 20.0, pulse, 4.0, YELLOW);
                }

                // Draw alert icons for needs with pulse
                let needs = dog.needs_attention();
                if !needs.is_empty() {
                    let pulse = (time * 6.0).sin() * 2.0 + 14.0;
                    draw_circle(dog.x + 35.0, dog.y - 35.0, pulse, RED);
                    draw_circle(dog.x + 35.0, dog.y - 35.0, pulse * 0.6, Color::from_rgba(255, 100, 100, 255));
                    draw_text_with_outline("!", dog.x + 30.0, dog.y - 27.0, 28.0, WHITE, BLACK);
                }
            }

            // Check for dog selection
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if mouse_x < 800.0 {
                    daycare.selected_dog = None;
                    for (i, dog) in daycare.dogs.iter().enumerate() {
                        let dx = mouse_x - dog.x;
                        let dy = mouse_y - (dog.y + 20.0);
                        if dx * dx + dy * dy < 40.0 * 40.0 {
                            daycare.selected_dog = Some(i);
                            break;
                        }
                    }
                }
            }

            // Draw right panel with bubbly design
            draw_rectangle(
                800.0,
                0.0,
                480.0,
                SCREEN_HEIGHT,
                Color::from_rgba(101, 67, 33, 255),
            );

            // Decorative bubbles on panel
            for i in 0..8 {
                let bubble_y = i as f32 * 100.0 + (time + i as f32).sin() * 10.0;
                let bubble_x = 790.0 + (time * 0.5 + i as f32 * 0.8).cos() * 8.0;
                draw_circle(bubble_x, bubble_y, 15.0, Color::from_rgba(120, 80, 40, 100));
            }

            let border_glow = ((time * 2.0).sin() * 50.0 + 100.0) as u8;
            draw_line(
                800.0,
                0.0,
                800.0,
                SCREEN_HEIGHT,
                5.0,
                Color::from_rgba(border_glow, border_glow / 2, 0, 255),
            );

            // Draw selected dog info
            if let Some(idx) = daycare.selected_dog {
                if let Some(dog) = daycare.dogs.get(idx) {
                    // Info bubble
                    draw_rounded_rect(815.0, 15.0, 450.0, 125.0, 20.0, Color::from_rgba(139, 90, 43, 255));
                    draw_rounded_rect_lines(815.0, 15.0, 450.0, 125.0, 20.0, 3.0, GOLD);

                    draw_text_with_outline("Selected Dog:", 830.0, 40.0, 22.0, YELLOW, BLACK);
                    draw_text_with_outline(&dog.name, 830.0, 70.0, 32.0, WHITE, Color::from_rgba(100, 50, 0, 255));
                    draw_text_with_outline(
                        &format!("{} - {} years old", dog.breed.as_str(), dog.age),
                        830.0,
                        95.0,
                        18.0,
                        Color::from_rgba(255, 230, 200, 255),
                        BLACK,
                    );
                    draw_text_with_outline(
                        &format!("Personality: {}", dog.personality.as_str()),
                        830.0,
                        118.0,
                        18.0,
                        Color::from_rgba(255, 230, 200, 255),
                        BLACK,
                    );

                    // Draw animated stats
                    draw_animated_stat_bar(
                        830.0,
                        460.0,
                        400.0,
                        dog.display_energy,
                        100.0,
                        BLUE,
                        "⚡ Energy",
                        time,
                    );
                    draw_animated_stat_bar(
                        830.0,
                        505.0,
                        400.0,
                        dog.display_happiness,
                        100.0,
                        YELLOW,
                        "😊 Happiness",
                        time,
                    );
                    draw_animated_stat_bar(
                        830.0,
                        550.0,
                        400.0,
                        100.0 - dog.display_hunger,
                        100.0,
                        GREEN,
                        "🍖 Fullness",
                        time,
                    );
                    draw_animated_stat_bar(
                        830.0,
                        595.0,
                        400.0,
                        dog.display_cleanliness,
                        100.0,
                        SKYBLUE,
                        "✨ Cleanliness",
                        time,
                    );

                    draw_text_with_outline(
                        &format!("Mood: {}", dog.get_mood()),
                        830.0,
                        655.0,
                        22.0,
                        WHITE,
                        BLACK,
                    );

                    let needs = dog.needs_attention();
                    if !needs.is_empty() {
                        draw_text_with_outline("⚠️ Needs:", 830.0, 685.0, 20.0, RED, BLACK);
                        for (i, need) in needs.iter().enumerate() {
                            draw_text_with_outline(
                                need,
                                830.0,
                                710.0 + i as f32 * 25.0,
                                18.0,
                                ORANGE,
                                BLACK,
                            );
                        }
                    }
                }
            } else {
                let float_offset = (time * 2.0).sin() * 5.0;
                draw_rounded_rect(850.0, 280.0 + float_offset, 350.0, 60.0, 30.0, Color::from_rgba(139, 90, 43, 200));
                draw_text_with_outline(
                    "👆 Click a dog to select",
                    880.0,
                    320.0 + float_offset,
                    24.0,
                    WHITE,
                    BLACK,
                );
            }

            // Draw action buttons
            for (i, button) in action_buttons.iter().enumerate() {
                button.draw(time);

                if button.is_clicked() {
                    if let Some(idx) = daycare.selected_dog {
                        let message = if let Some(dog) = daycare.dogs.get_mut(idx) {
                            let name = dog.name.clone();
                            let (msg, hours, particle_color) = match i {
                                0 => {
                                    dog.play();
                                    (
                                        format!("{} is playing!", name),
                                        1,
                                        Color::from_rgba(100, 150, 255, 255),
                                    )
                                }
                                1 => {
                                    dog.feed();
                                    (
                                        format!("{} is eating!", name),
                                        1,
                                        Color::from_rgba(100, 255, 100, 255),
                                    )
                                }
                                2 => {
                                    dog.nap();
                                    (
                                        format!("{} is napping!", name),
                                        2,
                                        Color::from_rgba(200, 150, 255, 255),
                                    )
                                }
                                3 => {
                                    dog.groom();
                                    (
                                        format!("{} is being groomed!", name),
                                        1,
                                        Color::from_rgba(255, 150, 200, 255),
                                    )
                                }
                                4 => {
                                    dog.train();
                                    (
                                        format!("{} is training!", name),
                                        1,
                                        Color::from_rgba(255, 200, 100, 255),
                                    )
                                }
                                _ => (String::new(), 0, WHITE),
                            };

                            if hours > 0 {
                                for _ in 0..15 {
                                    particles.push(Particle::new(dog.x, dog.y, particle_color));
                                }
                            }

                            daycare.pass_time(hours);
                            Some(msg)
                        } else {
                            None
                        };

                        if let Some(msg) = message {
                            daycare.show_message(&msg);
                        }
                    } else if i == 5 {
                        daycare.pass_time(1);
                    } else {
                        daycare.show_message("Select a dog first!");
                    }
                }
            }

            draw_floating_particles(&particles);
        }

        next_frame().await
    }
}
