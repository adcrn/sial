pub fn num_words(words: &Vec<&str>) -> f32 {
    let mut num_words = 0;
    for word in words {
    	num_words += 1;
    }

    return num_words as f32;
}

pub fn word_mean(words: &Vec<&str>) -> f32 {
	let word_mean: f32;
	let mut word_length_total = 0;
	let word_total = num_words(words) as f32;
	
	for word in words {
		word_length_total += word.len();
	}

	let total_length = word_length_total as f32;

	word_mean = total_length / word_total;

	return word_mean;
}

pub fn sent_mean(word_total: &f32, sentences: &Vec<&str>) -> f32 {
	let sent_mean: f32;
	let mut sent_total = 0;

	for sentence in sentences {
		sent_total += 1;
	}

	let total_sent = sent_total as f32;

	sent_mean = word_total / total_sent;

	return sent_mean;
}

