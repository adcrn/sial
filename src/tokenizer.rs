pub fn word_token(s: String) -> f32 {
    let separators: &[char] = &[' ', '.', ','];
    let words = s.split(separators);

    let (mut word_length_total, mut word_total) = (0, 0);
    for w in words {
    	word_length_total += w.len();
    	word_total += 1;
    }

    let mean_word_length = (word_length_total / word_total) as f32;

    return mean_word_length;
}

pub fn sentence_token(s: String) -> f32 {
	let separators: &[char] = &['.', '!', '?'];
	let sentences = s.split(separators);

	let (mut sentence_length_total, mut sentence_total) = (0,0);

	for s in sentences {
		sentence_length_total += s.len();
		sentence_total +=1;
	}

	let mean_sentence_length = (sentence_length_total / sentence_total) as f32;

	return mean_sentence_length;
}