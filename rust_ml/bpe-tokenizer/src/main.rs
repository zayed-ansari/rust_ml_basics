use std::collections::HashMap;

fn get_status(vocab: &HashMap<Vec<String>, usize>) -> HashMap<(String, String), usize> {
	let mut pairs: HashMap<(String, String), usize> = HashMap::new();
	
	for (word, freq) in vocab {
		// This will slide a window of 2 over each word's symbols
		for i in 0..word.len() - 1{
			let pair = (word[i].clone(), word[i+1].clone());
			*pairs.entry(pair).or_insert(0) += freq;
		}
	}
	pairs
}

fn merge_vocab(
	pair: &(String, String),
	vocab: &HashMap<Vec<String>, usize>
) -> HashMap<Vec<String>, usize> {
	let mut new_vocab: HashMap<Vec<String>, usize> = HashMap::new();
	for (word, freq) in vocab{
		let mut new_word: Vec<String> = Vec::new();	
		let mut i = 0;

		while i < word.len(){

			// if current and next match our pair merge them
			if i < word.len() - 1 && word[i] == pair.0 && word[i+1] == pair.1 {
				new_word.push(format!("{}{}", pair.0, pair.1)); //merge
				i+=2 ;
			
			} else{
				new_word.push(word[i].clone());
				i += 1; 
			}
		}
		*new_vocab.entry(new_word).or_insert(0) += freq;
	}
	new_vocab
 
}

fn tokenize(text: &str, merges: &Vec<(String, String)>) -> Vec<String> {
	let mut tokens: Vec<String> = text.chars().map(|c| c.to_string()).collect();
	tokens.push("</w>".to_string());

	for merge in merges {
		let mut i = 0;
		while i < tokens.len() - 1 {
			if tokens[i] == merge.0 && tokens[i+1] == merge.1 {
				tokens[i] = format!("{}{}", merge.0, merge.1);
				tokens.remove(i+1);

			}else{
				i+=1;
			}
		}
	}
	tokens
}
fn main() {
	// Test text -> small corpus
    let text = "low low low low low lower lower newest newest newest newest newest newest widest widest widest";
	
	// build initial vocab - each word split into  characters + end token
	    
	let mut vocab: HashMap<Vec<String>, usize> = HashMap::new();

	for word in text.split_whitespace(){
		let mut chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
		chars.push("</w>" .to_string()); 
		*vocab.entry(chars).or_insert(0) += 1;

} 
	let num_merges = 10;
	let mut merges: Vec<(String, String)> = Vec::new();

	for i in 0..num_merges{

		let pairs = get_status(&vocab);
		if pairs.is_empty() {break; }

		let best = pairs.iter().max_by_key(|x| x.1).unwrap().0.clone();
		println!("Merge {}: {:?}", i+1, best);
		vocab = merge_vocab(&best, &vocab);
		merges.push(best);
	}

	println!("\nFinal vocab:");
	for (word, freq) in &vocab {
		println!(" {:?} : {}", word, freq);

	}

	println!("\nLearned merges:");
	for (i, merge) in merges.iter().enumerate() {
		println!(" {}: {:?}", i+1, merge);
	}

	println!("\nTokenizing new words:");
	let test_words = vec!["low", "lowest", "newer"];
	for word in test_words {
		let tokens = tokenize(word, &merges);
		println!(" '{}' -> {:?}", word, tokens);
	}

}
