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
        draw_ellipse(self.x, self.y + 45.0, 35.0, 15.0, 0.0, Color::from_rgba(0, 0, 0, 50));

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

struct Daycare {
    dogs: Vec<Dog>,
    day: u32,
    time: u32,
    score: i32,
    selected_dog: Option<usize>,
    message: String,
    message_timer: f32,
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

    fn pass_time(&mut self) {
        self.time += 1;
        if self.time >= 18 {
            let day_score: i32 = self.dogs.iter().map(|d| d.happiness).sum();
            self.score += day_score;
            self.show_message(&format!("Day {} ended! Score: +{}", self.day, day_score));

            self.dogs.clear();
            self.day += 1;
            self.time = 8;
            self.selected_dog = None;
            self.spawn_new_dogs();
        } else {
            for dog in &mut self.dogs {
                dog.pass_time();
            }
        }
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
        Button {
            x,
            y,
            width,
            height,
            text: text.to_string(),
            color,
            hover_color: Color::from_rgba(
                (color.r * 1.2).min(1.0) as u8,
                (color.g * 1.2).min(1.0) as u8,
                (color.b * 1.2).min(1.0) as u8,
                255,
            ),
        }
    }

    fn draw(&self) {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height;

        let draw_color = if is_hovered {
            self.hover_color
        } else {
            self.color
        };

        draw_rectangle(self.x, self.y, self.width, self.height, draw_color);
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, WHITE);

        let text_dims = measure_text(&self.text, None, 20, 1.0);
        draw_text(
            &self.text,
            self.x + (self.width - text_dims.width) / 2.0,
            self.y + (self.height + text_dims.height) / 2.0,
            20.0,
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

fn draw_stat_bar(x: f32, y: f32, width: f32, value: i32, max: i32, color: Color, label: &str) {
    let fill_width = (value as f32 / max as f32) * width;

    draw_rectangle(x, y, width, 20.0, Color::from_rgba(50, 50, 50, 255));
    draw_rectangle(x, y, fill_width, 20.0, color);
    draw_rectangle_lines(x, y, width, 20.0, 2.0, WHITE);

    draw_text(label, x, y - 5.0, 16.0, WHITE);
    draw_text(&format!("{}/{}", value, max), x + width + 5.0, y + 15.0, 16.0, WHITE);
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
        Button::new(820.0, 150.0, 200.0, 50.0, "Play", BLUE),
        Button::new(1040.0, 150.0, 200.0, 50.0, "Feed", GREEN),
        Button::new(820.0, 220.0, 200.0, 50.0, "Nap", PURPLE),
        Button::new(1040.0, 220.0, 200.0, 50.0, "Groom", PINK),
        Button::new(820.0, 290.0, 200.0, 50.0, "Train", ORANGE),
        Button::new(820.0, 360.0, 420.0, 50.0, "Pass Time (1 hour)", DARKGRAY),
    ];

    loop {
        clear_background(Color::from_rgba(144, 238, 144, 255)); // Light green background

        let time = get_time() as f32;

        // Draw grass/floor
        draw_rectangle(0.0, 500.0, 800.0, 220.0, Color::from_rgba(34, 139, 34, 255));

        // Draw sky gradient
        for i in 0..50 {
            let y = i as f32 * 10.0;
            let alpha = 100 - (i * 2);
            draw_rectangle(
                0.0,
                y,
                800.0,
                10.0,
                Color::from_rgba(135, 206, 235, alpha as u8),
            );
        }

        // Draw title
        draw_text("Dog Daycare Simulator", 20.0, 40.0, 40.0, DARKBROWN);

        // Draw day, time, and score
        draw_text(
            &format!("Day: {} | Time: {}:00 | Score: {}", daycare.day, daycare.time, daycare.score),
            20.0,
            80.0,
            24.0,
            WHITE,
        );

        // Draw message
        if daycare.message_timer > 0.0 {
            daycare.message_timer -= get_frame_time();
            let msg_dims = measure_text(&daycare.message, None, 30, 1.0);
            let msg_x = (800.0 - msg_dims.width) / 2.0;
            draw_rectangle(
                msg_x - 10.0,
                100.0,
                msg_dims.width + 20.0,
                40.0,
                Color::from_rgba(0, 0, 0, 200),
            );
            draw_text(&daycare.message, msg_x, 130.0, 30.0, YELLOW);
        }

        // Draw dogs
        for (i, dog) in daycare.dogs.iter().enumerate() {
            dog.draw(time);

            // Highlight selected dog
            if Some(i) == daycare.selected_dog {
                draw_circle_lines(dog.x, dog.y + 20.0, 40.0, 3.0, YELLOW);
            }

            // Draw alert icons for needs
            let needs = dog.needs_attention();
            if !needs.is_empty() {
                draw_circle(dog.x + 30.0, dog.y - 30.0, 12.0, RED);
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

        // Draw right panel
        draw_rectangle(800.0, 0.0, 480.0, SCREEN_HEIGHT, Color::from_rgba(101, 67, 33, 255));

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

                // Draw stats
                draw_stat_bar(820.0, 450.0, 400.0, dog.energy, 100, BLUE, "Energy");
                draw_stat_bar(820.0, 490.0, 400.0, dog.happiness, 100, YELLOW, "Happiness");
                draw_stat_bar(
                    820.0,
                    530.0,
                    400.0,
                    100 - dog.hunger,
                    100,
                    GREEN,
                    "Fullness",
                );
                draw_stat_bar(
                    820.0,
                    570.0,
                    400.0,
                    dog.cleanliness,
                    100,
                    SKYBLUE,
                    "Cleanliness",
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
            draw_text("Click a dog to select", 820.0, 300.0, 24.0, WHITE);
        }

        // Draw action buttons
        for (i, button) in action_buttons.iter().enumerate() {
            button.draw();

            if button.is_clicked() {
                if let Some(idx) = daycare.selected_dog {
                    let message = if let Some(dog) = daycare.dogs.get_mut(idx) {
                        let name = dog.name.clone();
                        match i {
                            0 => {
                                dog.play();
                                Some(format!("{} is playing!", name))
                            }
                            1 => {
                                dog.feed();
                                Some(format!("{} is eating!", name))
                            }
                            2 => {
                                dog.nap();
                                Some(format!("{} is napping!", name))
                            }
                            3 => {
                                dog.groom();
                                Some(format!("{} is being groomed!", name))
                            }
                            4 => {
                                dog.train();
                                Some(format!("{} is training!", name))
                            }
                            _ => None
                        }
                    } else {
                        None
                    };
                    if let Some(msg) = message {
                        daycare.show_message(&msg);
                    }
                } else if i == 5 {
                    daycare.pass_time();
                } else {
                    daycare.show_message("Select a dog first!");
                }
            } else if i == 5 && button.is_clicked() {
                daycare.pass_time();
            }
        }

        next_frame().await
    }
}
