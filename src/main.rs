use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

// Clean color palette
const BG_LIGHT: Color = Color::new(0.95, 0.95, 0.98, 1.0);
const BG_DARK: Color = Color::new(0.25, 0.30, 0.35, 1.0);
const ACCENT_PRIMARY: Color = Color::new(0.30, 0.60, 0.90, 1.0);
const ACCENT_SUCCESS: Color = Color::new(0.30, 0.75, 0.40, 1.0);
const ACCENT_WARNING: Color = Color::new(0.95, 0.60, 0.20, 1.0);
const TEXT_DARK: Color = Color::new(0.15, 0.15, 0.20, 1.0);
const TEXT_LIGHT: Color = Color::new(0.95, 0.95, 0.98, 1.0);

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
    Cat,
    Fursuit,
    Dachshund,
    Greyhound,
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
            Breed::Cat => "Cat (snuck in!)",
            Breed::Fursuit => "Totally Real Dog",
            Breed::Dachshund => "Dachshund",
            Breed::Greyhound => "Greyhound",
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
            Breed::Cat => Color::from_rgba(255, 140, 60, 255), // Orange tabby
            Breed::Fursuit => Color::from_rgba(100, 150, 255, 255), // Blue fursuit
            Breed::Dachshund => Color::from_rgba(140, 70, 20, 255), // Brown sausage dog
            Breed::Greyhound => Color::from_rgba(180, 180, 180, 255), // Grey
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
            Breed::Cat => Color::from_rgba(255, 220, 180, 255), // Light orange stripes
            Breed::Fursuit => Color::from_rgba(255, 200, 160, 255), // Skin tone showing through
            Breed::Dachshund => Color::from_rgba(100, 50, 10, 255), // Dark brown
            Breed::Greyhound => Color::from_rgba(220, 220, 220, 255), // Light grey
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
            Breed::Cat => 0.75,
            Breed::Fursuit => 1.15, // Human-sized
            Breed::Dachshund => 0.7, // Low to ground
            Breed::Greyhound => 1.2, // Tall and slim
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
        let bark = match self.breed {
            Breed::Cat => "Meow!",
            Breed::Fursuit => "Woof! *zipper noise*",
            _ => "Woof! Woof!",
        };
        self.bark(bark);
    }

    fn feed(&mut self) {
        self.hunger = (self.hunger - 40).max(0);
        self.happiness = (self.happiness + 10).min(100);
        self.energy = (self.energy + 5).min(100);
        self.current_action = DogAction::Eating;
        self.action_timer = 2.0;
        let bark = match self.breed {
            Breed::Cat => "Purr~",
            Breed::Fursuit => "Thanks! *thumbs up*",
            _ => "Yum!",
        };
        self.bark(bark);
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
        let bark = match self.breed {
            Breed::Fursuit => "Careful of the seams!",
            _ => "So clean!",
        };
        self.bark(bark);
    }

    fn train(&mut self) {
        self.energy = (self.energy - 15).max(0);
        self.happiness = (self.happiness + 12).min(100);
        self.hunger = (self.hunger + 8).min(100);
        self.current_action = DogAction::Training;
        self.action_timer = 2.0;
        let bark = match self.breed {
            Breed::Cat => "I'll do it when I want to",
            Breed::Fursuit => "I already know this!",
            _ => "I'm learning!",
        };
        self.bark(bark);
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

        // Calculate visual scaling based on stats
        // Fullness (low hunger = more full = fatter/longer)
        let fullness = 1.0 - (self.hunger as f32 / 100.0);
        let fat_scale = 1.0 + fullness * 0.4; // Up to 40% fatter when full
        let length_scale = 1.0 + fullness * 0.6; // Up to 60% longer for sausage dog

        // Greyhound gets taller the more energy it has
        let height_scale = if self.breed == Breed::Greyhound {
            1.0 + (self.energy as f32 / 100.0) * 0.3 // Up to 30% taller
        } else {
            1.0
        };

        let bounce = match self.current_action {
            DogAction::Playing => (time * 8.0 + self.animation_offset).sin() * 8.0,
            DogAction::Eating => (time * 4.0).sin() * 1.5,
            DogAction::Napping => 0.0,
            _ => (time * 2.0 + self.animation_offset).sin() * 3.0,
        };
        let y_pos = self.y + bounce;

        // Shadow (scales with fatness/length)
        let shadow_scale = if self.breed == Breed::Dachshund {
            length_scale
        } else {
            fat_scale
        };
        draw_ellipse(
            self.x,
            self.y + 45.0 * scale,
            35.0 * scale * shadow_scale,
            15.0 * scale,
            0.0,
            Color::from_rgba(0, 0, 0, 50),
        );

        match self.breed {
            Breed::Cat => self.draw_cat(y_pos, scale, time, fat_scale),
            Breed::Fursuit => self.draw_fursuit(y_pos, scale, time, fat_scale),
            Breed::Dachshund => self.draw_dachshund(y_pos, scale, time, length_scale),
            Breed::Greyhound => self.draw_greyhound(y_pos, scale, time, height_scale),
            _ => self.draw_dog(y_pos, scale, time, fat_scale),
        }

        // Draw dirt when cleanliness is low
        if self.cleanliness < 60 {
            self.draw_dirt(y_pos, scale, time);
        }

        // Speech bubble
        if self.bark_timer > 0.0 {
            let bubble_alpha = (self.bark_timer / 2.5 * 255.0).min(255.0) as u8;
            let bubble_y = y_pos - 60.0 - (2.5 - self.bark_timer) * 20.0;

            let text_dims = measure_text(&self.bark_text, None, 16, 1.0);
            let bubble_width = text_dims.width + 24.0;
            let bubble_height = 32.0;
            let bubble_x = self.x - bubble_width / 2.0;

            draw_rounded_rect(
                bubble_x,
                bubble_y,
                bubble_width,
                bubble_height,
                16.0,
                Color::from_rgba(255, 255, 255, bubble_alpha),
            );
            draw_rounded_rect_lines(
                bubble_x,
                bubble_y,
                bubble_width,
                bubble_height,
                16.0,
                2.0,
                Color::from_rgba(60, 60, 80, bubble_alpha),
            );

            draw_triangle(
                vec2(self.x, bubble_y + bubble_height),
                vec2(self.x - 6.0, bubble_y + bubble_height + 8.0),
                vec2(self.x + 6.0, bubble_y + bubble_height),
                Color::from_rgba(255, 255, 255, bubble_alpha),
            );

            let text_color = Color::from_rgba(30, 30, 40, bubble_alpha);
            draw_text(
                &self.bark_text,
                bubble_x + 12.0,
                bubble_y + 21.0,
                16.0,
                text_color,
            );
        }

        // Name tag
        let name_width = measure_text(&self.name, None, 18, 1.0).width;
        draw_rounded_rect(
            self.x - name_width / 2.0 - 6.0,
            y_pos - 38.0,
            name_width + 12.0,
            24.0,
            12.0,
            Color::from_rgba(255, 255, 255, 240),
        );
        draw_text(&self.name, self.x - name_width / 2.0, y_pos - 21.0, 18.0, TEXT_DARK);
    }

    fn draw_dog(&self, y_pos: f32, scale: f32, time: f32, fat_scale: f32) {
        let body_size = 25.0 * scale;
        // Apply fat scaling to body
        draw_ellipse(
            self.x,
            y_pos + 20.0 * scale,
            body_size * fat_scale,
            body_size,
            0.0,
            self.breed.color()
        );

        match self.breed {
            Breed::Beagle | Breed::Husky | Breed::Corgi => {
                draw_ellipse(
                    self.x,
                    y_pos + 25.0 * scale,
                    body_size * 0.7 * fat_scale,
                    body_size * 0.7,
                    0.0,
                    self.breed.secondary_color(),
                );
            }
            Breed::Poodle => {
                draw_circle(self.x - 15.0 * scale * fat_scale, y_pos + 20.0 * scale, 10.0 * scale, self.breed.color());
                draw_circle(self.x + 15.0 * scale * fat_scale, y_pos + 20.0 * scale, 10.0 * scale, self.breed.color());
            }
            _ => {}
        }

        let head_size = 20.0 * scale;
        draw_circle(self.x, y_pos, head_size, self.breed.color());

        match self.breed {
            Breed::Husky => {
                draw_circle(self.x - 8.0 * scale, y_pos - 2.0 * scale, 7.0 * scale, WHITE);
                draw_circle(self.x + 8.0 * scale, y_pos - 2.0 * scale, 7.0 * scale, WHITE);
                draw_circle(self.x, y_pos + 8.0 * scale, 10.0 * scale, WHITE);
            }
            Breed::Beagle => {
                draw_circle(self.x, y_pos, head_size * 0.8, WHITE);
            }
            _ => {}
        }

        // Ears
        match self.breed {
            Breed::Beagle | Breed::Labrador | Breed::GoldenRetriever => {
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
                draw_circle(self.x - 15.0 * scale, y_pos - 5.0 * scale, 10.0 * scale, self.breed.color());
                draw_circle(self.x + 15.0 * scale, y_pos - 5.0 * scale, 10.0 * scale, self.breed.color());
            }
            Breed::Corgi => {
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
                draw_circle(self.x - 15.0 * scale, y_pos, 6.0 * scale, self.breed.color());
                draw_circle(self.x + 15.0 * scale, y_pos, 6.0 * scale, self.breed.color());
            }
            _ => {}
        }

        // Eyes
        let eye_color = if self.happiness > 60 { BLACK } else { GRAY };
        let eye_size = if self.current_action == DogAction::Napping { 1.5 } else { 3.0 };
        draw_circle(self.x - 7.0 * scale, y_pos - 3.0 * scale, eye_size * scale, eye_color);
        draw_circle(self.x + 7.0 * scale, y_pos - 3.0 * scale, eye_size * scale, eye_color);

        if self.current_action != DogAction::Napping {
            draw_circle(self.x - 6.0 * scale, y_pos - 4.0 * scale, 1.5 * scale, WHITE);
            draw_circle(self.x + 8.0 * scale, y_pos - 4.0 * scale, 1.5 * scale, WHITE);
        }

        // Nose
        draw_circle(self.x, y_pos + 5.0 * scale, 4.0 * scale, BLACK);

        // Mouth
        if self.happiness > 60 && self.current_action != DogAction::Eating {
            draw_line(self.x - 8.0 * scale, y_pos + 8.0 * scale, self.x, y_pos + 12.0 * scale, 2.0, BLACK);
            draw_line(self.x, y_pos + 12.0 * scale, self.x + 8.0 * scale, y_pos + 8.0 * scale, 2.0, BLACK);
        }

        if self.current_action == DogAction::Eating {
            let mouth_open = ((time * 10.0).sin() * 0.5 + 0.5) * 5.0;
            draw_circle(self.x, y_pos + 10.0 * scale, 3.0 + mouth_open, Color::from_rgba(50, 25, 25, 255));
        }

        // Legs
        let leg_width = 8.0 * scale;
        let leg_height = if self.breed == Breed::Corgi { 8.0 * scale } else { 15.0 * scale };
        draw_rectangle(self.x - 15.0 * scale, y_pos + 35.0 * scale, leg_width, leg_height, self.breed.color());
        draw_rectangle(self.x + 7.0 * scale, y_pos + 35.0 * scale, leg_width, leg_height, self.breed.color());

        // Tail
        let tail_angle = match self.current_action {
            DogAction::Playing => (time * 10.0 + self.animation_offset).sin() * 0.6,
            DogAction::Eating => (time * 8.0 + self.animation_offset).sin() * 0.5,
            DogAction::Napping => 0.0,
            _ => (time * 5.0 + self.animation_offset).sin() * 0.3,
        };
        let tail_x = self.x + 20.0 * scale + tail_angle * 15.0;

        match self.breed {
            Breed::Husky | Breed::GermanShepherd => {
                draw_circle(tail_x, y_pos + 12.0 * scale, 8.0 * scale, self.breed.color());
                draw_circle(tail_x + 5.0 * scale, y_pos + 8.0 * scale, 7.0 * scale, self.breed.color());
            }
            Breed::Corgi => {
                draw_circle(tail_x - 5.0 * scale, y_pos + 15.0 * scale, 5.0 * scale, self.breed.color());
            }
            _ => {
                draw_circle(tail_x, y_pos + 15.0 * scale, 6.0 * scale, self.breed.color());
            }
        }

        // Action effects
        self.draw_action_effects(y_pos, time);
    }

    fn draw_cat(&self, y_pos: f32, scale: f32, time: f32, fat_scale: f32) {
        // Sleeker cat body (gets fatter when full)
        draw_ellipse(self.x, y_pos + 18.0 * scale, 22.0 * scale * fat_scale, 18.0 * scale, 0.0, self.breed.color());

        // Stripes (scale with fatness)
        for i in 0..3 {
            let stripe_y = y_pos + 15.0 * scale + i as f32 * 6.0 * scale;
            draw_line(
                self.x - 15.0 * scale * fat_scale,
                stripe_y,
                self.x + 15.0 * scale * fat_scale,
                stripe_y,
                2.0 * scale,
                self.breed.secondary_color(),
            );
        }

        // Head
        draw_circle(self.x, y_pos, 18.0 * scale, self.breed.color());

        // Pointy cat ears
        draw_triangle(
            vec2(self.x - 10.0 * scale, y_pos - 16.0 * scale),
            vec2(self.x - 14.0 * scale, y_pos - 4.0 * scale),
            vec2(self.x - 6.0 * scale, y_pos - 4.0 * scale),
            self.breed.color(),
        );
        draw_triangle(
            vec2(self.x + 10.0 * scale, y_pos - 16.0 * scale),
            vec2(self.x + 6.0 * scale, y_pos - 4.0 * scale),
            vec2(self.x + 14.0 * scale, y_pos - 4.0 * scale),
            self.breed.color(),
        );

        // Cat eyes (slitted)
        draw_circle(self.x - 6.0 * scale, y_pos - 2.0 * scale, 4.0 * scale, Color::from_rgba(180, 255, 180, 255));
        draw_circle(self.x + 6.0 * scale, y_pos - 2.0 * scale, 4.0 * scale, Color::from_rgba(180, 255, 180, 255));
        draw_line(self.x - 6.0 * scale, y_pos - 5.0 * scale, self.x - 6.0 * scale, y_pos + 1.0 * scale, 1.5 * scale, BLACK);
        draw_line(self.x + 6.0 * scale, y_pos - 5.0 * scale, self.x + 6.0 * scale, y_pos + 1.0 * scale, 1.5 * scale, BLACK);

        // Pink nose
        draw_circle(self.x, y_pos + 5.0 * scale, 3.0 * scale, Color::from_rgba(255, 150, 180, 255));

        // Whiskers
        for i in -1..=1 {
            let whisker_y = y_pos + i as f32 * 3.0 * scale;
            draw_line(self.x - 18.0 * scale, whisker_y, self.x - 8.0 * scale, whisker_y, 1.0, BLACK);
            draw_line(self.x + 8.0 * scale, whisker_y, self.x + 18.0 * scale, whisker_y, 1.0, BLACK);
        }

        // Legs
        draw_rectangle(self.x - 12.0 * scale, y_pos + 28.0 * scale, 6.0 * scale, 14.0 * scale, self.breed.color());
        draw_rectangle(self.x + 6.0 * scale, y_pos + 28.0 * scale, 6.0 * scale, 14.0 * scale, self.breed.color());

        // Swishy cat tail
        let tail_angle = (time * 4.0 + self.animation_offset).sin() * 0.8;
        for i in 0..3 {
            let tail_x = self.x + 18.0 * scale + (i as f32 * 8.0 + tail_angle * 20.0);
            let tail_y = y_pos + 15.0 * scale + (i as f32 * 4.0);
            draw_circle(tail_x, tail_y, 5.0 * scale, self.breed.color());
        }

        self.draw_action_effects(y_pos, time);
    }

    fn draw_fursuit(&self, y_pos: f32, scale: f32, time: f32, fat_scale: f32) {
        // Body (fursuit) - person inside gets wider when full!
        draw_ellipse(self.x, y_pos + 18.0 * scale, 28.0 * scale * fat_scale, 28.0 * scale, 0.0, self.breed.color());

        // Head (costume head)
        draw_circle(self.x, y_pos - 2.0 * scale, 22.0 * scale, self.breed.color());

        // Visible seam/zipper
        draw_line(
            self.x,
            y_pos + 40.0 * scale,
            self.x,
            y_pos + 10.0 * scale,
            2.0,
            Color::from_rgba(80, 80, 90, 255),
        );

        // Zipper pull
        draw_circle(self.x, y_pos + 15.0 * scale, 3.0 * scale, Color::from_rgba(150, 150, 160, 255));

        // Large costume ears (floppy)
        draw_ellipse(
            self.x - 18.0 * scale,
            y_pos - 12.0 * scale,
            10.0 * scale,
            18.0 * scale,
            0.5,
            self.breed.color(),
        );
        draw_ellipse(
            self.x + 18.0 * scale,
            y_pos - 12.0 * scale,
            10.0 * scale,
            18.0 * scale,
            -0.5,
            self.breed.color(),
        );

        // Big cartoony eyes (mesh visible)
        draw_circle(self.x - 8.0 * scale, y_pos - 4.0 * scale, 6.0 * scale, BLACK);
        draw_circle(self.x + 8.0 * scale, y_pos - 4.0 * scale, 6.0 * scale, BLACK);
        draw_circle(self.x - 8.0 * scale, y_pos - 4.0 * scale, 4.0 * scale, WHITE);
        draw_circle(self.x + 8.0 * scale, y_pos - 4.0 * scale, 4.0 * scale, WHITE);

        // Visible mesh pattern
        for dx in -1..=1 {
            for dy in -1..=1 {
                draw_circle(
                    self.x - 8.0 * scale + dx as f32 * 2.0,
                    y_pos - 4.0 * scale + dy as f32 * 2.0,
                    0.5,
                    Color::from_rgba(60, 60, 70, 100),
                );
            }
        }

        // Big foam nose
        draw_circle(self.x, y_pos + 6.0 * scale, 5.0 * scale, BLACK);
        draw_circle(self.x - 1.0 * scale, y_pos + 5.0 * scale, 2.0 * scale, Color::from_rgba(100, 100, 110, 255));

        // Permanent smile (stitched on)
        draw_line(self.x - 10.0 * scale, y_pos + 10.0 * scale, self.x, y_pos + 14.0 * scale, 2.5, BLACK);
        draw_line(self.x, y_pos + 14.0 * scale, self.x + 10.0 * scale, y_pos + 10.0 * scale, 2.5, BLACK);

        // Human hands visible (skin tone)
        draw_circle(self.x - 18.0 * scale, y_pos + 32.0 * scale, 5.0 * scale, self.breed.secondary_color());
        draw_circle(self.x + 18.0 * scale, y_pos + 32.0 * scale, 5.0 * scale, self.breed.secondary_color());

        // Fingers
        for i in 0..3 {
            draw_rectangle(
                self.x - 18.0 * scale - 2.0 + i as f32 * 2.0,
                y_pos + 35.0 * scale,
                1.5,
                4.0 * scale,
                self.breed.secondary_color(),
            );
            draw_rectangle(
                self.x + 18.0 * scale - 2.0 + i as f32 * 2.0,
                y_pos + 35.0 * scale,
                1.5,
                4.0 * scale,
                self.breed.secondary_color(),
            );
        }

        // Feetpaws
        draw_ellipse(self.x - 12.0 * scale, y_pos + 42.0 * scale, 8.0 * scale, 6.0 * scale, 0.0, self.breed.color());
        draw_ellipse(self.x + 12.0 * scale, y_pos + 42.0 * scale, 8.0 * scale, 6.0 * scale, 0.0, self.breed.color());

        // Paw pads
        for i in 0..3 {
            draw_circle(
                self.x - 12.0 * scale - 3.0 + i as f32 * 3.0,
                y_pos + 40.0 * scale,
                1.5,
                Color::from_rgba(80, 80, 90, 255),
            );
            draw_circle(
                self.x + 12.0 * scale - 3.0 + i as f32 * 3.0,
                y_pos + 40.0 * scale,
                1.5,
                Color::from_rgba(80, 80, 90, 255),
            );
        }

        // Tail (obviously fake - stuffed)
        let tail_angle = (time * 3.0 + self.animation_offset).sin() * 0.2; // Less movement
        let tail_x = self.x + 22.0 * scale + tail_angle * 8.0;
        draw_circle(tail_x, y_pos + 16.0 * scale, 7.0 * scale, self.breed.color());
        draw_circle(tail_x + 6.0 * scale, y_pos + 14.0 * scale, 6.0 * scale, self.breed.color());

        self.draw_action_effects(y_pos, time);
    }

    fn draw_dachshund(&self, y_pos: f32, scale: f32, time: f32, length_scale: f32) {
        // Long sausage body that gets LONGER when full (not fatter!)
        let body_length = 40.0 * scale * length_scale;
        let body_height = 18.0 * scale;

        // Draw elongated body
        draw_ellipse(self.x, y_pos + 20.0 * scale, body_length, body_height, 0.0, self.breed.color());

        // Belly marking
        draw_ellipse(self.x, y_pos + 22.0 * scale, body_length * 0.8, body_height * 0.6, 0.0, self.breed.secondary_color());

        // Head
        draw_circle(self.x - body_length * 0.6, y_pos + 5.0 * scale, 15.0 * scale, self.breed.color());

        // Long floppy ears
        draw_ellipse(
            self.x - body_length * 0.6 - 12.0 * scale,
            y_pos + 10.0 * scale,
            6.0 * scale,
            18.0 * scale,
            0.3,
            self.breed.color(),
        );
        draw_ellipse(
            self.x - body_length * 0.6 + 12.0 * scale,
            y_pos + 10.0 * scale,
            6.0 * scale,
            18.0 * scale,
            -0.3,
            self.breed.color(),
        );

        // Eyes
        let eye_size = if self.current_action == DogAction::Napping { 1.5 } else { 3.0 };
        draw_circle(self.x - body_length * 0.6 - 5.0 * scale, y_pos + 2.0 * scale, eye_size * scale, BLACK);
        draw_circle(self.x - body_length * 0.6 + 5.0 * scale, y_pos + 2.0 * scale, eye_size * scale, BLACK);

        // Nose
        draw_circle(self.x - body_length * 0.6, y_pos + 8.0 * scale, 3.0 * scale, BLACK);

        // Short legs (stay same size)
        let leg_width = 6.0 * scale;
        let leg_height = 10.0 * scale;
        draw_rectangle(self.x - body_length * 0.4, y_pos + 32.0 * scale, leg_width, leg_height, self.breed.color());
        draw_rectangle(self.x - body_length * 0.2, y_pos + 32.0 * scale, leg_width, leg_height, self.breed.color());
        draw_rectangle(self.x + body_length * 0.2, y_pos + 32.0 * scale, leg_width, leg_height, self.breed.color());
        draw_rectangle(self.x + body_length * 0.4, y_pos + 32.0 * scale, leg_width, leg_height, self.breed.color());

        // Tail at the back
        let tail_angle = (time * 5.0 + self.animation_offset).sin() * 0.3;
        let tail_x = self.x + body_length * 0.6 + tail_angle * 10.0;
        draw_circle(tail_x, y_pos + 18.0 * scale, 5.0 * scale, self.breed.color());

        self.draw_action_effects(y_pos, time);
    }

    fn draw_greyhound(&self, y_pos: f32, scale: f32, time: f32, height_scale: f32) {
        // Tall, slim greyhound that gets TALLER with more energy
        let body_width = 20.0 * scale;
        let body_height = 25.0 * scale * height_scale;

        // Slim body
        draw_ellipse(self.x, y_pos + 15.0 * scale, body_width, body_height, 0.0, self.breed.color());

        // White chest
        draw_ellipse(self.x, y_pos + 18.0 * scale, body_width * 0.7, body_height * 0.6, 0.0, self.breed.secondary_color());

        // Long neck extending upward
        let neck_y = y_pos - 5.0 * scale * height_scale;
        draw_ellipse(self.x, neck_y, 10.0 * scale, 15.0 * scale * height_scale, 0.0, self.breed.color());

        // Small head
        let head_y = neck_y - 12.0 * scale * height_scale;
        draw_ellipse(self.x, head_y, 12.0 * scale, 14.0 * scale, 0.0, self.breed.color());

        // Long snout
        draw_ellipse(self.x, head_y + 6.0 * scale, 8.0 * scale, 10.0 * scale, 0.0, self.breed.secondary_color());

        // Small pointy ears
        draw_triangle(
            vec2(self.x - 8.0 * scale, head_y - 10.0 * scale),
            vec2(self.x - 10.0 * scale, head_y - 2.0 * scale),
            vec2(self.x - 6.0 * scale, head_y - 2.0 * scale),
            self.breed.color(),
        );
        draw_triangle(
            vec2(self.x + 8.0 * scale, head_y - 10.0 * scale),
            vec2(self.x + 6.0 * scale, head_y - 2.0 * scale),
            vec2(self.x + 10.0 * scale, head_y - 2.0 * scale),
            self.breed.color(),
        );

        // Eyes
        let eye_size = if self.current_action == DogAction::Napping { 1.5 } else { 3.0 };
        draw_circle(self.x - 4.0 * scale, head_y, eye_size * scale, BLACK);
        draw_circle(self.x + 4.0 * scale, head_y, eye_size * scale, BLACK);

        // Nose
        draw_circle(self.x, head_y + 8.0 * scale, 3.0 * scale, BLACK);

        // Long thin legs
        let leg_width = 5.0 * scale;
        let leg_height = 20.0 * scale;
        draw_rectangle(self.x - 12.0 * scale, y_pos + 35.0 * scale, leg_width, leg_height, self.breed.color());
        draw_rectangle(self.x + 7.0 * scale, y_pos + 35.0 * scale, leg_width, leg_height, self.breed.color());

        // Thin tail
        let tail_angle = (time * 6.0 + self.animation_offset).sin() * 0.4;
        let tail_x = self.x + 18.0 * scale + tail_angle * 12.0;
        for i in 0..3 {
            let tx = tail_x + i as f32 * 5.0;
            let ty = y_pos + 20.0 * scale + i as f32 * 3.0;
            draw_circle(tx, ty, 3.0 * scale, self.breed.color());
        }

        self.draw_action_effects(y_pos, time);
    }

    fn draw_dirt(&self, y_pos: f32, scale: f32, time: f32) {
        // Draw dirt smudges when cleanliness is low
        let dirt_amount = 1.0 - (self.cleanliness as f32 / 60.0); // 0.0 to 1.0
        let dirt_alpha = (dirt_amount * 180.0) as u8;
        let dirt_color = Color::from_rgba(80, 60, 40, dirt_alpha);

        // Random-ish dirt spots (use animation_offset as seed)
        let spots = [
            (0.0, 10.0, 6.0),
            (12.0, 15.0, 8.0),
            (-10.0, 12.0, 7.0),
            (8.0, 22.0, 5.0),
            (-15.0, 20.0, 6.0),
        ];

        for (i, (x_off, y_off, size)) in spots.iter().enumerate() {
            let wobble = ((time + self.animation_offset + i as f32) * 2.0).sin() * 0.5;
            draw_circle(
                self.x + x_off * scale + wobble,
                y_pos + y_off * scale,
                size * scale * dirt_amount,
                dirt_color,
            );
        }

        // Extra dirt lines/smudges
        if self.cleanliness < 30 {
            for i in 0..3 {
                let y = y_pos + (15.0 + i as f32 * 5.0) * scale;
                draw_line(
                    self.x - 8.0 * scale,
                    y,
                    self.x + 8.0 * scale,
                    y,
                    1.5,
                    Color::from_rgba(70, 50, 30, dirt_alpha + 50),
                );
            }
        }
    }

    fn draw_action_effects(&self, y_pos: f32, time: f32) {
        match self.current_action {
            DogAction::Grooming => {
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
                for i in 0..3 {
                    let line_offset = i as f32 * 8.0 - 8.0;
                    draw_line(
                        self.x + line_offset,
                        y_pos - 35.0,
                        self.x + line_offset,
                        y_pos - 45.0,
                        2.0,
                        SKYBLUE,
                    );
                }
            }
            DogAction::Napping => {
                let z_offset = (time * 2.0).sin() * 3.0;
                draw_text("Z", self.x + 20.0, y_pos - 15.0 + z_offset, 20.0, Color::from_rgba(100, 100, 120, 200));
                draw_text("z", self.x + 28.0, y_pos - 22.0 + z_offset * 0.7, 16.0, Color::from_rgba(100, 100, 120, 150));
                draw_text("z", self.x + 33.0, y_pos - 28.0 + z_offset * 0.5, 12.0, Color::from_rgba(100, 100, 120, 100));
            }
            _ => {}
        }
    }
}

fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

fn draw_rounded_rect_lines(x: f32, y: f32, w: f32, h: f32, r: f32, thickness: f32, color: Color) {
    draw_rectangle(x + r, y - thickness / 2.0, w - 2.0 * r, thickness, color);
    draw_rectangle(x + r, y + h - thickness / 2.0, w - 2.0 * r, thickness, color);
    draw_rectangle(x - thickness / 2.0, y + r, thickness, h - 2.0 * r, color);
    draw_rectangle(x + w - thickness / 2.0, y + r, thickness, h - 2.0 * r, color);
    draw_circle_lines(x + r, y + r, r, thickness, color);
    draw_circle_lines(x + w - r, y + r, r, thickness, color);
    draw_circle_lines(x + r, y + h - r, r, thickness, color);
    draw_circle_lines(x + w - r, y + h - r, r, thickness, color);
}

fn draw_bone_bar(x: f32, y: f32, width: f32, value: f32, max: f32, color: Color, label: &str, time: f32) {
    let fill_width = (value / max) * width;
    let bone_height = 28.0;
    let end_radius = 14.0;

    // Background
    draw_rectangle(x + end_radius, y, width - 2.0 * end_radius, bone_height, Color::from_rgba(220, 220, 230, 255));
    draw_circle(x + end_radius, y + bone_height / 2.0, end_radius, Color::from_rgba(220, 220, 230, 255));
    draw_circle(x + width - end_radius, y + bone_height / 2.0, end_radius, Color::from_rgba(220, 220, 230, 255));

    // Fill
    let shimmer = (time * 2.0 + x / 100.0).sin() * 0.1 + 0.9;
    let shimmer_color = Color::new(
        color.r * shimmer,
        color.g * shimmer,
        color.b * shimmer,
        1.0,
    );

    if fill_width > end_radius * 2.0 {
        draw_rectangle(x + end_radius, y, fill_width - 2.0 * end_radius, bone_height, shimmer_color);
        draw_circle(x + end_radius, y + bone_height / 2.0, end_radius, shimmer_color);
        draw_circle(x + fill_width - end_radius, y + bone_height / 2.0, end_radius, shimmer_color);
    } else if fill_width > 0.0 {
        draw_circle(x + end_radius, y + bone_height / 2.0, end_radius, shimmer_color);
    }

    // Label
    draw_text(label, x, y - 8.0, 20.0, TEXT_DARK);

    // Value
    let value_text = format!("{:.0}/{}", value, max);
    draw_text(&value_text, x + width + 12.0, y + 20.0, 18.0, TEXT_DARK);
}

#[derive(Clone)]
struct DogReport {
    name: String,
    breed: Breed,
    happiness: i32,
}

#[derive(Clone, Copy)]
struct Upgrade {
    name: &'static str,
    description: &'static str,
    cost: i32,
    level: i32,
    max_level: i32,
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    DayReport,
    Shop,
}

struct Daycare {
    dogs: Vec<Dog>,
    day: u32,
    time: u32,
    score: i32,
    money: i32,
    selected_dog: Option<usize>,
    message: String,
    message_timer: f32,
    state: GameState,
    day_reports: Vec<DogReport>,
    day_score: i32,
    upgrades: Vec<Upgrade>,
}

impl Daycare {
    fn new() -> Self {
        let upgrades = vec![
            Upgrade {
                name: "Better Food",
                description: "+10% happiness from feeding",
                cost: 50,
                level: 0,
                max_level: 5,
            },
            Upgrade {
                name: "Comfy Beds",
                description: "+15% energy from naps",
                cost: 75,
                level: 0,
                max_level: 5,
            },
            Upgrade {
                name: "Premium Toys",
                description: "+10% happiness from play",
                cost: 60,
                level: 0,
                max_level: 5,
            },
            Upgrade {
                name: "Training Program",
                description: "+20% effectiveness",
                cost: 100,
                level: 0,
                max_level: 3,
            },
            Upgrade {
                name: "Spa Treatment",
                description: "Grooming gives +15 happiness",
                cost: 80,
                level: 0,
                max_level: 3,
            },
        ];

        Daycare {
            dogs: Vec::new(),
            day: 1,
            time: 8,
            score: 0,
            money: 100,
            selected_dog: None,
            message: String::new(),
            message_timer: 0.0,
            state: GameState::Playing,
            day_reports: Vec::new(),
            day_score: 0,
            upgrades,
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
                let earned_money = (self.day_score as f32 * 0.5) as i32;
                self.money += earned_money;

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
            ("Whiskers", Cat, Calm, 4),
            ("Steve", Fursuit, Friendly, 28), // Obviously an adult human
            ("Slinky", Dachshund, Playful, 5), // Sausage dog!
            ("Zoom", Greyhound, Energetic, 3), // Fast and tall
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

    fn buy_upgrade(&mut self, index: usize) -> bool {
        if index < self.upgrades.len() {
            let upgrade_level = self.upgrades[index].level;
            let upgrade_max_level = self.upgrades[index].max_level;
            let upgrade_cost = self.upgrades[index].cost;

            if upgrade_level < upgrade_max_level && self.money >= upgrade_cost {
                self.money -= upgrade_cost;
                self.upgrades[index].level += 1;
                self.upgrades[index].cost = (upgrade_cost as f32 * 1.5) as i32;
                return true;
            }
        }
        false
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
        let alpha = self.life / self.max_life;
        let mut color = self.color;
        color.a = alpha;
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
}

impl Button {
    fn new(x: f32, y: f32, width: f32, height: f32, text: &str, color: Color) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: text.to_string(),
            color,
        }
    }

    fn draw(&self, time: f32) {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height;

        let pulse = if is_hovered { 1.03 } else { 1.0 };
        let draw_x = self.x - (self.width * pulse - self.width) / 2.0;
        let draw_y = self.y - (self.height * pulse - self.height) / 2.0;
        let draw_width = self.width * pulse;
        let draw_height = self.height * pulse;

        let draw_color = if is_hovered {
            Color::new(self.color.r * 1.1, self.color.g * 1.1, self.color.b * 1.1, 1.0)
        } else {
            self.color
        };

        draw_rounded_rect(draw_x, draw_y, draw_width, draw_height, 12.0, draw_color);

        let text_dims = measure_text(&self.text, None, 22, 1.0);
        let text_x = draw_x + (draw_width - text_dims.width) / 2.0;
        let text_y = draw_y + (draw_height + text_dims.height) / 2.0;

        draw_text(&self.text, text_x, text_y, 22.0, TEXT_LIGHT);
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

fn draw_floating_particles(particles: &[Particle]) {
    for particle in particles {
        particle.draw();
    }
}

fn draw_background() {
    // Clean gradient sky
    for i in 0..60 {
        let y = i as f32 * 12.0;
        let t = i as f32 / 60.0;
        let color = Color::new(
            0.5 + t * 0.3,
            0.7 + t * 0.2,
            0.9,
            1.0,
        );
        draw_rectangle(0.0, y, 800.0, 12.0, color);
    }

    // Simple grass
    draw_rectangle(0.0, 500.0, 800.0, 220.0, Color::new(0.4, 0.7, 0.3, 1.0));
}

fn draw_shop(daycare: &mut Daycare, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.7));

    let panel_width = 700.0;
    let panel_height = 600.0;
    let panel_x = (SCREEN_WIDTH - panel_width) / 2.0;
    let panel_y = (SCREEN_HEIGHT - panel_height) / 2.0;

    draw_rounded_rect(panel_x, panel_y, panel_width, panel_height, 20.0, BG_LIGHT);
    draw_rounded_rect_lines(panel_x, panel_y, panel_width, panel_height, 20.0, 3.0, ACCENT_PRIMARY);

    draw_text("Shop", panel_x + 280.0, panel_y + 50.0, 48.0, TEXT_DARK);
    draw_text(
        &format!("Money: ${}", daycare.money),
        panel_x + 30.0,
        panel_y + 100.0,
        28.0,
        ACCENT_SUCCESS,
    );

    let upgrades_start_y = panel_y + 140.0;
    for (i, upgrade) in daycare.upgrades.iter().enumerate() {
        let y = upgrades_start_y + i as f32 * 85.0;
        let is_maxed = upgrade.level >= upgrade.max_level;
        let can_afford = daycare.money >= upgrade.cost;

        let bg_color = if is_maxed {
            Color::new(0.7, 0.85, 0.7, 1.0)
        } else if can_afford {
            BG_LIGHT
        } else {
            Color::new(0.90, 0.90, 0.92, 1.0)
        };

        draw_rounded_rect(panel_x + 30.0, y, 640.0, 75.0, 12.0, bg_color);
        draw_rounded_rect_lines(panel_x + 30.0, y, 640.0, 75.0, 12.0, 2.0, ACCENT_PRIMARY);

        draw_text(upgrade.name, panel_x + 45.0, y + 30.0, 24.0, TEXT_DARK);
        draw_text(upgrade.description, panel_x + 45.0, y + 55.0, 18.0, Color::new(0.4, 0.4, 0.5, 1.0));

        let level_text = format!("Lv {}/{}", upgrade.level, upgrade.max_level);
        draw_text(&level_text, panel_x + 480.0, y + 35.0, 20.0, TEXT_DARK);

        if !is_maxed {
            let cost_text = format!("${}", upgrade.cost);
            let cost_color = if can_afford { ACCENT_SUCCESS } else { ACCENT_WARNING };
            draw_text(&cost_text, panel_x + 580.0, y + 60.0, 22.0, cost_color);
        } else {
            draw_text("MAX", panel_x + 590.0, y + 60.0, 22.0, ACCENT_SUCCESS);
        }
    }

    let close_button = Button::new(panel_x + 250.0, panel_y + panel_height - 70.0, 200.0, 50.0, "Close", BG_DARK);
    close_button.draw(time);

    if close_button.is_clicked() {
        daycare.state = GameState::Playing;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        for (i, _) in daycare.upgrades.clone().iter().enumerate() {
            let y = upgrades_start_y + i as f32 * 85.0;
            if mouse_x >= panel_x + 30.0
                && mouse_x <= panel_x + 670.0
                && mouse_y >= y
                && mouse_y <= y + 75.0
            {
                if daycare.buy_upgrade(i) {
                    daycare.show_message("Purchased!");
                } else {
                    daycare.show_message("Can't afford or maxed!");
                }
            }
        }
    }
}

fn draw_day_report(daycare: &Daycare, time: f32) {
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.7));

    let panel_width = 650.0;
    let panel_height = 550.0;
    let panel_x = (SCREEN_WIDTH - panel_width) / 2.0;
    let panel_y = (SCREEN_HEIGHT - panel_height) / 2.0;

    draw_rounded_rect(panel_x, panel_y, panel_width, panel_height, 20.0, BG_LIGHT);
    draw_rounded_rect_lines(panel_x, panel_y, panel_width, panel_height, 20.0, 3.0, ACCENT_PRIMARY);

    draw_text(&format!("Day {} Complete!", daycare.day), panel_x + 180.0, panel_y + 60.0, 42.0, TEXT_DARK);

    let earned_money = (daycare.day_score as f32 * 0.5) as i32;
    draw_text(
        &format!("Earned: ${} | Score: {}", earned_money, daycare.day_score),
        panel_x + 180.0,
        panel_y + 105.0,
        24.0,
        ACCENT_SUCCESS,
    );

    draw_line(panel_x + 30.0, panel_y + 125.0, panel_x + panel_width - 30.0, panel_y + 125.0, 2.0, ACCENT_PRIMARY);

    draw_text("Performance", panel_x + 40.0, panel_y + 160.0, 26.0, TEXT_DARK);

    for (i, report) in daycare.day_reports.iter().enumerate() {
        let y = panel_y + 195.0 + i as f32 * 50.0;

        draw_text(
            &format!("{}", report.name),
            panel_x + 40.0,
            y,
            20.0,
            TEXT_DARK,
        );

        let bar_width = 250.0;
        let bar_x = panel_x + 320.0;
        draw_bone_bar(bar_x, y - 22.0, bar_width, report.happiness as f32, 100.0, ACCENT_PRIMARY, "", time);

        let emoji = if report.happiness > 80 {
            "😊"
        } else if report.happiness > 60 {
            "🙂"
        } else if report.happiness > 40 {
            "😐"
        } else {
            "☹️"
        };
        draw_text(emoji, panel_x + 590.0, y, 28.0, TEXT_DARK);
    }

    draw_text(
        &format!("Total: ${} | Score: {}", daycare.money, daycare.score),
        panel_x + 200.0,
        panel_y + panel_height - 90.0,
        26.0,
        TEXT_DARK,
    );

    let button_y = panel_y + panel_height - 60.0;
    let button1 = Button::new(panel_x + 60.0, button_y, 240.0, 45.0, "Shop", ACCENT_PRIMARY);
    let button2 = Button::new(panel_x + 350.0, button_y, 240.0, 45.0, "Next Day", ACCENT_SUCCESS);
    button1.draw(time);
    button2.draw(time);
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
        Button::new(820.0, 150.0, 200.0, 50.0, "Play (1h)", ACCENT_PRIMARY),
        Button::new(1040.0, 150.0, 200.0, 50.0, "Feed (1h)", ACCENT_SUCCESS),
        Button::new(820.0, 215.0, 200.0, 50.0, "Nap (2h)", Color::new(0.6, 0.4, 0.8, 1.0)),
        Button::new(1040.0, 215.0, 200.0, 50.0, "Groom (1h)", Color::new(0.9, 0.5, 0.7, 1.0)),
        Button::new(820.0, 280.0, 200.0, 50.0, "Train (1h)", ACCENT_WARNING),
        Button::new(1040.0, 280.0, 200.0, 50.0, "Pass Time", BG_DARK),
    ];

    let shop_button = Button::new(820.0, 350.0, 420.0, 50.0, "Open Shop", Color::new(0.5, 0.3, 0.7, 1.0));

    let mut particles: Vec<Particle> = Vec::new();

    loop {
        clear_background(BG_LIGHT);

        let time = get_time() as f32;
        let dt = get_frame_time();

        particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });

        if daycare.state == GameState::Shop {
            draw_shop(&mut daycare, time);
        } else if daycare.state == GameState::DayReport {
            draw_day_report(&daycare, time);

            let panel_width = 650.0;
            let panel_height = 550.0;
            let panel_x = (SCREEN_WIDTH - panel_width) / 2.0;
            let panel_y = (SCREEN_HEIGHT - panel_height) / 2.0;
            let button_y = panel_y + panel_height - 60.0;

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if mouse_x >= panel_x + 60.0 && mouse_x <= panel_x + 300.0 && mouse_y >= button_y && mouse_y <= button_y + 45.0 {
                    daycare.state = GameState::Shop;
                } else if mouse_x >= panel_x + 350.0 && mouse_x <= panel_x + 590.0 && mouse_y >= button_y && mouse_y <= button_y + 45.0 {
                    daycare.start_new_day();
                }
            }
        } else {
            draw_background();

            draw_text("Dog Daycare Simulator", 20.0, 45.0, 42.0, TEXT_DARK);

            let info_text = format!("Day {} | {}:00 | ${} | Score: {}", daycare.day, daycare.time, daycare.money, daycare.score);
            draw_rounded_rect(15.0, 55.0, 480.0, 42.0, 21.0, Color::new(1.0, 1.0, 1.0, 0.9));
            draw_text(&info_text, 25.0, 83.0, 24.0, TEXT_DARK);

            if daycare.message_timer > 0.0 {
                daycare.message_timer -= dt;
                let msg_dims = measure_text(&daycare.message, None, 26, 1.0);
                let msg_x = (800.0 - msg_dims.width) / 2.0 - 20.0;

                draw_rounded_rect(msg_x, 115.0, msg_dims.width + 40.0, 50.0, 25.0, ACCENT_PRIMARY);
                draw_text(&daycare.message, msg_x + 20.0, 148.0, 26.0, TEXT_LIGHT);
            }

            for (i, dog) in daycare.dogs.iter_mut().enumerate() {
                dog.update(dt);
                dog.draw(time);

                if Some(i) == daycare.selected_dog {
                    let pulse = (time * 4.0).sin() * 4.0 + 50.0;
                    draw_circle_lines(dog.x, dog.y + 20.0, pulse, 4.0, ACCENT_WARNING);
                }

                let needs = dog.needs_attention();
                if !needs.is_empty() {
                    draw_circle(dog.x + 40.0, dog.y - 40.0, 12.0, ACCENT_WARNING);
                    draw_text("!", dog.x + 35.0, dog.y - 30.0, 26.0, TEXT_LIGHT);
                }
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if mouse_x < 800.0 {
                    daycare.selected_dog = None;
                    for (i, dog) in daycare.dogs.iter().enumerate() {
                        let dx = mouse_x - dog.x;
                        let dy = mouse_y - (dog.y + 20.0);
                        if dx * dx + dy * dy < 45.0 * 45.0 {
                            daycare.selected_dog = Some(i);
                            break;
                        }
                    }
                }
            }

            draw_rectangle(800.0, 0.0, 480.0, SCREEN_HEIGHT, BG_DARK);

            if let Some(idx) = daycare.selected_dog {
                if let Some(dog) = daycare.dogs.get(idx) {
                    draw_rounded_rect(820.0, 20.0, 440.0, 130.0, 15.0, Color::new(0.3, 0.35, 0.4, 1.0));

                    draw_text("Selected:", 840.0, 50.0, 22.0, Color::new(0.7, 0.7, 0.75, 1.0));
                    draw_text(&dog.name, 840.0, 80.0, 32.0, TEXT_LIGHT);
                    draw_text(&format!("{} - {} yrs", dog.breed.as_str(), dog.age), 840.0, 108.0, 18.0, Color::new(0.8, 0.8, 0.85, 1.0));
                    draw_text(&format!("{}", dog.personality.as_str()), 840.0, 133.0, 18.0, Color::new(0.8, 0.8, 0.85, 1.0));

                    draw_bone_bar(830.0, 500.0, 410.0, dog.display_energy, 100.0, ACCENT_PRIMARY, "Energy", time);
                    draw_bone_bar(830.0, 548.0, 410.0, dog.display_happiness, 100.0, Color::new(0.95, 0.85, 0.3, 1.0), "Happiness", time);
                    draw_bone_bar(830.0, 596.0, 410.0, 100.0 - dog.display_hunger, 100.0, ACCENT_SUCCESS, "Fullness", time);
                    draw_bone_bar(830.0, 644.0, 410.0, dog.display_cleanliness, 100.0, Color::new(0.4, 0.8, 0.9, 1.0), "Clean", time);

                    let needs = dog.needs_attention();
                    if !needs.is_empty() {
                        draw_text("Needs:", 840.0, 690.0, 20.0, ACCENT_WARNING);
                        for (i, need) in needs.iter().enumerate() {
                            draw_text(need, 840.0, 713.0 + i as f32 * 22.0, 18.0, TEXT_LIGHT);
                        }
                    }
                }
            } else {
                let float_offset = (time * 2.0).sin() * 3.0;
                draw_rounded_rect(870.0, 300.0 + float_offset, 340.0, 70.0, 35.0, Color::new(0.3, 0.35, 0.4, 1.0));
                draw_text("Click a dog/cat/person", 900.0, 342.0 + float_offset, 24.0, TEXT_LIGHT);
            }

            for (i, button) in action_buttons.iter().enumerate() {
                button.draw(time);

                if button.is_clicked() {
                    if let Some(idx) = daycare.selected_dog {
                        let message = if let Some(dog) = daycare.dogs.get_mut(idx) {
                            let name = dog.name.clone();
                            let (msg, hours, particle_color) = match i {
                                0 => {
                                    dog.play();
                                    (format!("{} playing!", name), 1, ACCENT_PRIMARY)
                                }
                                1 => {
                                    dog.feed();
                                    (format!("{} eating!", name), 1, ACCENT_SUCCESS)
                                }
                                2 => {
                                    dog.nap();
                                    (format!("{} napping!", name), 2, Color::new(0.6, 0.4, 0.8, 1.0))
                                }
                                3 => {
                                    dog.groom();
                                    (format!("{} grooming!", name), 1, Color::new(0.9, 0.5, 0.7, 1.0))
                                }
                                4 => {
                                    dog.train();
                                    (format!("{} training!", name), 1, ACCENT_WARNING)
                                }
                                _ => (String::new(), 0, WHITE),
                            };

                            if hours > 0 {
                                for _ in 0..10 {
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
                        daycare.show_message("Select first!");
                    }
                }
            }

            shop_button.draw(time);
            if shop_button.is_clicked() {
                daycare.state = GameState::Shop;
            }

            draw_floating_particles(&particles);
        }

        next_frame().await
    }
}
