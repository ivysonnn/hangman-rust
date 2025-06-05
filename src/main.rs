use std::env;
use std::io::Write;
use clearscreen::{self, clear};

enum Message {
    AlreadyGuessed(char),
    InvalidCharacter,
}

fn draw_hangman(wrong: usize) {
    let stages = [
        "
  +---+
  |   |
      |
      |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
      |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
      |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |
=========",
        "
  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
=========",
    ];

    println!("{}", stages[wrong]);
}
 
fn draw(word: &str, guessed: &[char], wrong_guesses: usize) {
    clear().expect("failed to clear the screen");

    draw_hangman(wrong_guesses);

    let display_word = word
        .chars()
        .map(|c| if guessed.contains(&c) { c.to_string() } else { "_".to_string()})
        .collect::<Vec<String>>()
        .join(" ");

    println!("word: {}", display_word);
    println!("guessed letters: {:?}", guessed);
    println!("lives left: {}", (6 - wrong_guesses));
        
}

fn is_guess_in_word(guess: &char, word: &str) -> Option<usize>{
   word.find(*guess)
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprint!("usage: cargo run [SECRET_WORD]");
        return;
    }

    let mut msg : Option<Message> = None;

    let secret_word = args[1].as_str().to_lowercase();
    let mut guessed : Vec<char> = Vec::new();
    let mut wrong_guesses : usize = 0;

    loop { //Game loop starts
        draw(&secret_word, &guessed, wrong_guesses);
        
        if let Some(m) = msg {
            match m {
                Message::AlreadyGuessed(c) => {
                    println!("{} was already guessed. try another one!", c);
                    msg = None;
                }, 
                Message::InvalidCharacter => {
                    println!("invalid character. try another one!");
                    msg = None;
                },
            }
        }

        print!("take your guess: ");
        std::io::stdout().flush().unwrap();
        let mut buf : String = String::new();
        std::io::stdin().read_line(&mut buf).expect("error");
        let guess = match buf.trim().chars().next() {
            Some(c) if c.is_alphabetic() => c.to_ascii_lowercase(),
            _ => {
                msg = Some(Message::InvalidCharacter);
                continue;
            }
        };

        if guessed.contains(&guess) {
            msg = Some(Message::AlreadyGuessed(guess));
            continue;
        }

        guessed.push(guess);

        if let None = is_guess_in_word(&guess, &secret_word) {
            wrong_guesses += 1;
        } 

        if secret_word.chars().all(|c| guessed.contains(&c)) {
            draw(&secret_word, &guessed, wrong_guesses);
            println!("🎉 you won!");
            break;
        }

         if wrong_guesses >= 6 {
            draw(&secret_word, &guessed, wrong_guesses);
            println!("💀 you lost! the word was '{}'", secret_word);
            break;
        }
    }
}
