use std::io::{self, Write};

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct Dog {
    name: String,
    breed: Breed,
    personality: Personality,
    age: u8,
    energy: i32,      // 0-100
    happiness: i32,   // 0-100
    hunger: i32,      // 0-100 (higher = more hungry)
    cleanliness: i32, // 0-100
}

impl Dog {
    fn new(name: String, breed: Breed, personality: Personality, age: u8) -> Self {
        Dog {
            name,
            breed,
            personality,
            age,
            energy: 80,
            happiness: 70,
            hunger: 30,
            cleanliness: 100,
        }
    }

    fn status(&self) -> String {
        format!(
            "{} the {} ({}, {} years old)\n  Energy: {} | Happiness: {} | Hunger: {} | Cleanliness: {}",
            self.name,
            self.breed.as_str(),
            self.personality.as_str(),
            self.age,
            self.energy,
            self.happiness,
            self.hunger,
            self.cleanliness
        )
    }

    fn play(&mut self) {
        self.energy = (self.energy - 20).max(0);
        self.happiness = (self.happiness + 15).min(100);
        self.hunger = (self.hunger + 10).min(100);
        self.cleanliness = (self.cleanliness - 15).max(0);
        println!("  {} is playing! Tail wagging intensifies!", self.name);
    }

    fn feed(&mut self) {
        self.hunger = (self.hunger - 40).max(0);
        self.happiness = (self.happiness + 10).min(100);
        self.energy = (self.energy + 5).min(100);
        println!("  {} is eating delicious kibble! Nom nom nom!", self.name);
    }

    fn nap(&mut self) {
        self.energy = (self.energy + 30).min(100);
        self.happiness = (self.happiness + 5).min(100);
        self.hunger = (self.hunger + 5).min(100);
        println!("  {} is taking a nap. Zzz...", self.name);
    }

    fn groom(&mut self) {
        self.cleanliness = 100;
        self.happiness = (self.happiness + 8).min(100);
        println!("  {} is getting groomed! Looking fresh and clean!", self.name);
    }

    fn train(&mut self) {
        self.energy = (self.energy - 15).max(0);
        self.happiness = (self.happiness + 12).min(100);
        self.hunger = (self.hunger + 8).min(100);
        println!("  {} is learning new tricks! Good dog!", self.name);
    }

    fn pass_time(&mut self) {
        // Natural degradation over time
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

    fn needs_attention(&self) -> Vec<String> {
        let mut needs = Vec::new();
        if self.hunger > 70 {
            needs.push("Hungry!".to_string());
        }
        if self.energy < 30 {
            needs.push("Tired".to_string());
        }
        if self.cleanliness < 40 {
            needs.push("Dirty".to_string());
        }
        if self.happiness < 40 {
            needs.push("Needs attention".to_string());
        }
        needs
    }
}

struct Daycare {
    name: String,
    dogs: Vec<Dog>,
    day: u32,
    time: u32, // 0-24 hours
    score: i32,
}

impl Daycare {
    fn new(name: String) -> Self {
        Daycare {
            name,
            dogs: Vec::new(),
            day: 1,
            time: 8, // Start at 8 AM
            score: 0,
        }
    }

    fn add_dog(&mut self, dog: Dog) {
        println!("\n{} has arrived at the daycare!", dog.name);
        self.dogs.push(dog);
    }

    fn show_status(&self) {
        println!("\n========================================");
        println!("  {} - Day {} - {}:00", self.name, self.day, self.time);
        println!("  Score: {}", self.score);
        println!("========================================");
        println!("Dogs in daycare: {}\n", self.dogs.len());

        for (i, dog) in self.dogs.iter().enumerate() {
            println!("{}. {}", i + 1, dog.status());
            println!("   Mood: {}", dog.get_mood());
            let needs = dog.needs_attention();
            if !needs.is_empty() {
                println!("   NEEDS: {}", needs.join(", "));
            }
            println!();
        }
    }

    fn pass_time(&mut self) {
        self.time += 1;
        if self.time >= 18 {
            // End of day
            println!("\n--- End of Day {} ---", self.day);
            println!("Dogs are being picked up by their owners!");

            // Calculate score based on dog happiness
            let day_score: i32 = self.dogs.iter().map(|d| d.happiness).sum();
            self.score += day_score;

            println!("Day score: {} points", day_score);
            println!("Total score: {}\n", self.score);

            self.dogs.clear();
            self.day += 1;
            self.time = 8;

            // Add new dogs for next day
            self.spawn_new_dogs();
        } else {
            // Time passes
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

        for i in 0..num_dogs {
            let idx = (start_idx + i) % dogs_data.len();
            let (name, breed, personality, age) = &dogs_data[idx];
            self.add_dog(Dog::new(
                name.to_string(),
                breed.clone(),
                personality.clone(),
                *age,
            ));
        }
    }
}

fn main() {
    println!("\n🐕 Welcome to Dog Daycare Simulator! 🐕\n");

    print!("Enter your daycare name: ");
    io::stdout().flush().unwrap();

    let mut daycare_name = String::new();
    io::stdin().read_line(&mut daycare_name).unwrap();
    let daycare_name = daycare_name.trim().to_string();

    let mut daycare = Daycare::new(daycare_name);

    println!("\nYour daycare opens at 8:00 AM and closes at 18:00 (6 PM).");
    println!("Take good care of the dogs to earn points!");

    // Spawn initial dogs
    daycare.spawn_new_dogs();

    // Main game loop
    loop {
        daycare.show_status();

        println!("\nWhat would you like to do?");
        println!("1. Play with a dog");
        println!("2. Feed a dog");
        println!("3. Let a dog nap");
        println!("4. Groom a dog");
        println!("5. Train a dog");
        println!("6. Pass time (1 hour)");
        println!("7. Quit");

        print!("\nChoice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" | "2" | "3" | "4" | "5" => {
                if daycare.dogs.is_empty() {
                    println!("\nNo dogs in daycare!");
                    continue;
                }

                print!("Select dog (1-{}): ", daycare.dogs.len());
                io::stdout().flush().unwrap();

                let mut dog_choice = String::new();
                io::stdin().read_line(&mut dog_choice).unwrap();

                if let Ok(idx) = dog_choice.trim().parse::<usize>() {
                    if idx > 0 && idx <= daycare.dogs.len() {
                        let dog = &mut daycare.dogs[idx - 1];
                        println!();
                        match choice.trim() {
                            "1" => dog.play(),
                            "2" => dog.feed(),
                            "3" => dog.nap(),
                            "4" => dog.groom(),
                            "5" => dog.train(),
                            _ => {}
                        }
                    } else {
                        println!("\nInvalid dog selection!");
                    }
                } else {
                    println!("\nInvalid input!");
                }
            }
            "6" => {
                println!("\nTime passes...");
                daycare.pass_time();

                if daycare.dogs.is_empty() {
                    println!("Press Enter to start a new day...");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                }
            }
            "7" => {
                println!("\nThanks for playing!");
                println!("Final Score: {} points", daycare.score);
                break;
            }
            _ => {
                println!("\nInvalid choice!");
            }
        }
    }
}
