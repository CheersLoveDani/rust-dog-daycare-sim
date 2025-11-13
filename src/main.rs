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
    // Smooth stat transitions
    display_energy: f32,
    display_happiness: f32,
    display_hunger: f32,
    display_cleanliness: f32,
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
        }
    }

    fn play(&mut self) {
        self.energy = (self.energy - 20).max(0);
        self.happiness = (self.happiness + 15).min(100);
        self.hunger = (self.hunger + 10).min(100);
        self.cleanliness = (self.cleanliness - 15).max(0);
    }

    fn feed(&mut self) {
        self.hunger = (self.hunger - 40).max(0);
        self.happiness = (self.happiness + 10).min(100);
        self.energy = (self.energy + 5).min(100);
    }

    fn nap(&mut self) {
        self.energy = (self.energy + 30).min(100);
        self.happiness = (self.happiness + 5).min(100);
        self.hunger = (self.hunger + 5).min(100);
    }

    fn groom(&mut self) {
        self.cleanliness = 100;
        self.happiness = (self.happiness + 8).min(100);
    }

    fn train(&mut self) {
        self.energy = (self.energy - 15).max(0);
        self.happiness = (self.happiness + 12).min(100);
        self.hunger = (self.hunger + 8).min(100);
    }

    fn pass_time(&mut self) {
        self.energy = (self.energy - 5).max(0);
        self.hunger = (self.hunger + 8).min(100);
        self.happiness = (self.happiness - 3).max(0);
        self.cleanliness = (self.cleanliness - 5).max(0);
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
        let bounce = (time * 2.0 + self.animation_offset).sin() * 3.0;
        let y_pos = self.y + bounce;

        // Draw shadow
        draw_ellipse(
            self.x,
            self.y + 45.0,
            35.0,
            15.0,
            0.0,
            Color::from_rgba(0, 0, 0, 50),
        );

        // Draw body
        draw_circle(self.x, y_pos + 20.0, 25.0, self.breed.color());

        // Draw head
        draw_circle(self.x, y_pos, 20.0, self.breed.color());

        // Draw ears
        draw_circle(self.x - 15.0, y_pos - 5.0, 8.0, self.breed.color());
        draw_circle(self.x + 15.0, y_pos - 5.0, 8.0, self.breed.color());

        // Draw eyes
        let eye_color = if self.happiness > 60 { BLACK } else { GRAY };
        draw_circle(self.x - 7.0, y_pos - 3.0, 3.0, eye_color);
        draw_circle(self.x + 7.0, y_pos - 3.0, 3.0, eye_color);

        // Draw nose
        draw_circle(self.x, y_pos + 5.0, 4.0, BLACK);

        // Draw mouth (smile if happy)
        if self.happiness > 60 {
            draw_line(
                self.x - 8.0,
                y_pos + 8.0,
                self.x,
                y_pos + 12.0,
                2.0,
                BLACK,
            );
            draw_line(self.x, y_pos + 12.0, self.x + 8.0, y_pos + 8.0, 2.0, BLACK);
        }

        // Draw legs
        draw_rectangle(self.x - 15.0, y_pos + 35.0, 8.0, 15.0, self.breed.color());
        draw_rectangle(self.x + 7.0, y_pos + 35.0, 8.0, 15.0, self.breed.color());

        // Draw tail wagging
        let tail_angle = (time * 5.0 + self.animation_offset).sin() * 0.3;
        let tail_x = self.x + 20.0 + tail_angle * 10.0;
        draw_circle(tail_x, y_pos + 15.0, 6.0, self.breed.color());

        // Draw name tag
        let name_width = measure_text(&self.name, None, 16, 1.0).width;
        draw_text(&self.name, self.x - name_width / 2.0, y_pos - 30.0, 16.0, WHITE);
    }
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
                // Store day report data
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
        self.vy += 0.2; // gravity
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

        // Pulse animation
        let pulse = (time * 3.0).sin() * 0.05 + 1.0;
        let offset = if is_hovered { 2.0 } else { 0.0 };

        let draw_color = if is_hovered {
            self.hover_color
        } else {
            self.color
        };

        // Glow effect when hovered
        if is_hovered {
            draw_rectangle(
                self.x - 2.0,
                self.y - 2.0,
                self.width + 4.0,
                self.height + 4.0,
                Color::from_rgba(255, 255, 255, 100),
            );
        }

        draw_rectangle(
            self.x - offset,
            self.y - offset,
            self.width + offset * 2.0,
            self.height + offset * 2.0,
            draw_color,
        );
        draw_rectangle_lines(
            self.x - offset,
            self.y - offset,
            self.width + offset * 2.0,
            self.height + offset * 2.0,
            2.0,
            WHITE,
        );

        let text_dims = measure_text(&self.text, None, 20, 1.0);
        let scale = if is_hovered { pulse } else { 1.0 };
        let font_size = 20.0 * scale;

        draw_text(
            &self.text,
            self.x + (self.width - text_dims.width * scale) / 2.0,
            self.y + (self.height + text_dims.height * scale) / 2.0,
            font_size,
            WHITE,
        );
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

    // Background
    draw_rectangle(x, y, width, 20.0, Color::from_rgba(50, 50, 50, 255));

    // Animated shimmer effect
    let shimmer = (time * 2.0 + x / 100.0).sin() * 0.1 + 0.9;
    let shimmer_color = Color::from_rgba(
        (color.r as f32 * shimmer) as u8,
        (color.g as f32 * shimmer) as u8,
        (color.b as f32 * shimmer) as u8,
        255,
    );

    // Fill bar
    draw_rectangle(x, y, fill_width, 20.0, shimmer_color);

    // Glossy effect
    draw_rectangle(
        x,
        y,
        fill_width,
        8.0,
        Color::from_rgba(255, 255, 255, 50),
    );

    // Border
    draw_rectangle_lines(x, y, width, 20.0, 2.0, WHITE);

    // Label
    draw_text(label, x, y - 5.0, 16.0, WHITE);

    // Value
    let value_text = format!("{:.0}/{}", value, max);
    draw_text(&value_text, x + width + 5.0, y + 15.0, 16.0, WHITE);
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
    draw_rectangle(
        panel_x + 5.0,
        panel_y + 5.0,
        panel_width,
        panel_height,
        Color::from_rgba(0, 0, 0, 100),
    );

    // Panel background with gradient
    draw_rectangle(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Color::from_rgba(101, 67, 33, 255),
    );
    draw_rectangle(
        panel_x,
        panel_y,
        panel_width,
        100.0,
        Color::from_rgba(139, 90, 43, 255),
    );

    // Border
    draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 4.0, GOLD);

    // Title with animation
    let title_scale = 1.0 + (time * 3.0).sin() * 0.05;
    draw_text(
        &format!("Day {} Complete!", daycare.day),
        panel_x + 150.0,
        panel_y + 50.0,
        40.0 * title_scale,
        YELLOW,
    );

    // Score
    draw_text(
        &format!("Day Score: {} points", daycare.day_score),
        panel_x + 200.0,
        panel_y + 100.0,
        24.0,
        WHITE,
    );

    // Divider
    draw_line(
        panel_x + 20.0,
        panel_y + 120.0,
        panel_x + panel_width - 20.0,
        panel_y + 120.0,
        2.0,
        GOLD,
    );

    // Dog reports
    draw_text(
        "Dog Happiness Report:",
        panel_x + 30.0,
        panel_y + 150.0,
        20.0,
        YELLOW,
    );

    for (i, report) in daycare.day_reports.iter().enumerate() {
        let y = panel_y + 180.0 + i as f32 * 40.0;
        let bounce = (time * 3.0 + i as f32 * 0.5).sin() * 2.0;

        // Dog name and breed
        draw_text(
            &format!("{} ({})", report.name, report.breed.as_str()),
            panel_x + 30.0,
            y + bounce,
            18.0,
            WHITE,
        );

        // Happiness bar
        let bar_width = 200.0;
        let bar_x = panel_x + 300.0;
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
        draw_text(emoji, panel_x + 520.0, y + bounce, 24.0, WHITE);
    }

    // Total score
    draw_text(
        &format!("Total Score: {}", daycare.score),
        panel_x + 200.0,
        panel_y + panel_height - 80.0,
        28.0,
        GOLD,
    );

    // Continue button
    let button_y = panel_y + panel_height - 50.0;
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
        Button::new(820.0, 150.0, 200.0, 50.0, "Play (1h)", BLUE),
        Button::new(1040.0, 150.0, 200.0, 50.0, "Feed (1h)", GREEN),
        Button::new(820.0, 220.0, 200.0, 50.0, "Nap (2h)", PURPLE),
        Button::new(1040.0, 220.0, 200.0, 50.0, "Groom (1h)", PINK),
        Button::new(820.0, 290.0, 200.0, 50.0, "Train (1h)", ORANGE),
        Button::new(820.0, 360.0, 420.0, 50.0, "Pass Time (1h)", DARKGRAY),
    ];

    let mut particles: Vec<Particle> = Vec::new();
    let mut background_particles: Vec<Particle> = Vec::new();
    let mut particle_spawn_timer = 0.0;

    loop {
        clear_background(Color::from_rgba(144, 238, 144, 255));

        let time = get_time() as f32;
        let dt = get_frame_time();

        // Spawn background particles (floating hearts/stars)
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

            // Check for continue button click
            let panel_width = 600.0;
            let panel_height = 500.0;
            let panel_x = (SCREEN_WIDTH - panel_width) / 2.0;
            let panel_y = (SCREEN_HEIGHT - panel_height) / 2.0 + (time * 2.0).sin() * 5.0;
            let button_y = panel_y + panel_height - 50.0;

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

            // Draw floating background particles
            draw_floating_particles(&background_particles);

            // Draw title with animation
            let title_scale = 1.0 + (time * 2.0).sin() * 0.02;
            draw_text(
                "Dog Daycare Simulator",
                20.0,
                40.0,
                40.0 * title_scale,
                DARKBROWN,
            );

            // Draw day, time, and score with glow
            let info_text = format!(
                "Day: {} | Time: {}:00 | Score: {}",
                daycare.day, daycare.time, daycare.score
            );
            draw_text(&info_text, 22.0, 82.0, 24.0, BLACK);
            draw_text(&info_text, 20.0, 80.0, 24.0, WHITE);

            // Draw message with slide animation
            if daycare.message_timer > 0.0 {
                daycare.message_timer -= dt;
                let msg_dims = measure_text(&daycare.message, None, 30, 1.0);
                let msg_x = (800.0 - msg_dims.width) / 2.0;
                let slide_offset = if daycare.message_timer > 2.5 {
                    (3.0 - daycare.message_timer) * 100.0
                } else {
                    0.0
                };

                draw_rectangle(
                    msg_x - 10.0,
                    100.0 - slide_offset,
                    msg_dims.width + 20.0,
                    40.0,
                    Color::from_rgba(0, 0, 0, 220),
                );
                draw_text(
                    &daycare.message,
                    msg_x,
                    130.0 - slide_offset,
                    30.0,
                    YELLOW,
                );
            }

            // Update and draw dogs
            for (i, dog) in daycare.dogs.iter_mut().enumerate() {
                dog.update_display_stats();
                dog.draw(time);

                // Highlight selected dog
                if Some(i) == daycare.selected_dog {
                    let pulse = (time * 4.0).sin() * 3.0 + 42.0;
                    draw_circle_lines(dog.x, dog.y + 20.0, pulse, 3.0, YELLOW);
                }

                // Draw alert icons for needs with pulse
                let needs = dog.needs_attention();
                if !needs.is_empty() {
                    let pulse = (time * 6.0).sin() * 2.0 + 12.0;
                    draw_circle(dog.x + 30.0, dog.y - 30.0, pulse, RED);
                    draw_text("!", dog.x + 25.0, dog.y - 22.0, 24.0, WHITE);
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

            // Draw right panel with animated border
            draw_rectangle(
                800.0,
                0.0,
                480.0,
                SCREEN_HEIGHT,
                Color::from_rgba(101, 67, 33, 255),
            );
            let border_glow = ((time * 2.0).sin() * 50.0 + 100.0) as u8;
            draw_rectangle_lines(
                800.0,
                0.0,
                480.0,
                SCREEN_HEIGHT,
                3.0,
                Color::from_rgba(border_glow, border_glow / 2, 0, 255),
            );

            // Draw selected dog info
            if let Some(idx) = daycare.selected_dog {
                if let Some(dog) = daycare.dogs.get(idx) {
                    draw_text("Selected Dog:", 820.0, 30.0, 24.0, WHITE);

                    draw_text(&dog.name, 820.0, 60.0, 32.0, YELLOW);
                    draw_text(
                        &format!("{} - {} years old", dog.breed.as_str(), dog.age),
                        820.0,
                        90.0,
                        18.0,
                        WHITE,
                    );
                    draw_text(
                        &format!("Personality: {}", dog.personality.as_str()),
                        820.0,
                        115.0,
                        18.0,
                        WHITE,
                    );

                    // Draw animated stats
                    draw_animated_stat_bar(
                        820.0,
                        450.0,
                        400.0,
                        dog.display_energy,
                        100.0,
                        BLUE,
                        "Energy",
                        time,
                    );
                    draw_animated_stat_bar(
                        820.0,
                        490.0,
                        400.0,
                        dog.display_happiness,
                        100.0,
                        YELLOW,
                        "Happiness",
                        time,
                    );
                    draw_animated_stat_bar(
                        820.0,
                        530.0,
                        400.0,
                        100.0 - dog.display_hunger,
                        100.0,
                        GREEN,
                        "Fullness",
                        time,
                    );
                    draw_animated_stat_bar(
                        820.0,
                        570.0,
                        400.0,
                        dog.display_cleanliness,
                        100.0,
                        SKYBLUE,
                        "Cleanliness",
                        time,
                    );

                    draw_text(
                        &format!("Mood: {}", dog.get_mood()),
                        820.0,
                        620.0,
                        20.0,
                        WHITE,
                    );

                    let needs = dog.needs_attention();
                    if !needs.is_empty() {
                        draw_text("Needs:", 820.0, 650.0, 20.0, RED);
                        for (i, need) in needs.iter().enumerate() {
                            draw_text(need, 820.0, 675.0 + i as f32 * 25.0, 18.0, ORANGE);
                        }
                    }
                }
            } else {
                let float_offset = (time * 2.0).sin() * 5.0;
                draw_text(
                    "Click a dog to select",
                    820.0,
                    300.0 + float_offset,
                    24.0,
                    WHITE,
                );
            }

            // Draw action buttons with animations
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

                            // Spawn particles at dog location
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
                        daycare.show_message("Select a dog first!");
                    }
                }
            }

            // Draw action particles
            draw_floating_particles(&particles);
        }

        next_frame().await
    }
}
