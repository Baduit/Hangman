use std::fs::File;
use std::io::prelude::*;


struct GameInfo
{
	word_to_guess: String,
	string_discovered: Vec<(char, bool)>,
	chars_used: Vec<char>,
	nb_failure_left: i32,
	hangman: Vec<String>,
}

enum GameState
{
	RUNNING,
	WIN,
	LOST,
}

impl GameInfo
{
    fn new(word_to_guess: String) -> GameInfo
	{
		let mut string_discovered = Vec::new();
		for c in word_to_guess.chars()
		{
			string_discovered.push((c, false));
		}

        GameInfo { word_to_guess, string_discovered, chars_used: Vec::new(), nb_failure_left: 10, hangman: extract_hangman() }
    }

	fn is_game_won(&self) -> bool
	{
		for i in &self.string_discovered
		{
			if i.1 == false
			{
				return false;
			}
		}
		true	
	}

	fn game_state(&self) -> GameState
	{
		if self.nb_failure_left < 0
		{
			GameState::LOST
		}
		else if self.is_game_won()
		{
			GameState::WIN
		}
		else
		{
			GameState::RUNNING
		}
	}

	fn print_discovery(&self)
	{
		println!("\nYou can fail only {} times before dying", self.nb_failure_left);

		let mut tmp_str = String::new();
		for (c, i) in &self.string_discovered
		{
			match i
			{
				true	=> tmp_str.push(*c),
				false	=> tmp_str.push('_'),
			}
		}
		println!("For the moment you have: {}", tmp_str);
	}

	fn show_hangman(&self)
	{
		let coeff = 10 - self.nb_failure_left;
		let nb_line_to_show = (coeff as usize) * 2 + 1;

		for i in 0 .. nb_line_to_show
		{
			println!("{}", self.hangman[i]);
		}
	}

	fn add_failure(&mut self)
	{
		self.nb_failure_left -= 1;
		self.show_hangman();
	}

	fn add_char(&mut self, c: char) -> bool
	{
		if self.chars_used.contains(&c)
		{
			self.add_failure();
			false
		}
		else
		{
			self.chars_used.push(c);

			let mut char_is_found = false;
			for mut i in &mut self.string_discovered
			{
				if i.0 == c
				{
					char_is_found = true;
					i.1 = true;
				}
			}

			if !char_is_found
			{
				self.add_failure();
				false
			}
			else
			{
				true
			}
		}
	}
}

fn extract_hangman() -> Vec<String>
{
	let mut file = File::open("ascii_art_hangman.txt").expect("File not found.");
	let mut content = String::new();
	file.read_to_string(&mut content).expect("Can't read the file.");

	let mut hangman = Vec::new();
	for l in content.lines()
	{
		hangman.push(l.to_string());
	}
	hangman
}

fn get_input_char() -> char
{
	println!("Enter a character you think is in the word.");

	let mut guess = String::new();

	std::io::stdin().read_line(&mut guess)
		.expect("Failed to read line");
	guess.chars().next().unwrap()
}

fn get_word_to_guess() -> String
{
	println!("Enter the word you want the player to guess. (case sensitive)");

	let mut guess = String::new();

	std::io::stdin().read_line(&mut guess)
		.expect("Failed to read line");
	guess.trim().to_string()
}

fn main()
{
	let mut game_info = GameInfo::new(get_word_to_guess());

	loop
	{
		game_info.add_char(get_input_char());

		match game_info.game_state()
		{
			GameState::WIN		=>
			{
				println!("You won! The word was: {}", game_info.word_to_guess);
				break;
			}
			GameState::LOST		=>
			{
				println!("You are dead! The word was: {}", game_info.word_to_guess);
				break;
			}
			GameState::RUNNING	=> game_info.print_discovery(),
		}
	}
}
