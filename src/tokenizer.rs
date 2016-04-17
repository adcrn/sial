pub fn word_token(s: &String) -> Vec<&str> {
    
    let separators: &[char] = &[' ', '.', ','];
    
    let words: Vec<&str> = s.split(separators).collect();

    return words;
}

pub fn sent_token(s: &String) -> Vec<&str> {
	
	let separators: &[char] = &['.', '!', '?'];
	
	let sentences: Vec<&str> = s.split(separators).collect();

	return sentences;
}

pub fn para_token(s: &String) -> Vec<&str> {
	
	let separators: &[char] = &['\n'];

	let paragraphs: Vec<&str> = s.split(separators).collect();

	return paragraphs;
}

// Needs some thought.
/*pub fn punctuation(s: &String) {

	let temp_punc: String =  s.chars()
				.filter(|&x| !x.is_alphanumeric())
				.map(|x| x)
				.collect();

	
}*/