fn num_words(v: &Vec<&str>) -> usize {
    let mut num_words = 0;
    for i in v {
    	num_words += 1;
    }

    return num_words;
}

pub fn word_mean(v: &Vec<&str>) -> (f32, f32) {
	let mut word_mean: f32;
	let mut word_length_total = 0;
	let word_total = num_words(v) as f32;
	println!("word_total: {}", word_total);
	
	for i in v {
		word_length_total += i.len();
	}

	let total_length = word_length_total as f32;

	word_mean = total_length / word_total;

	return (word_total, word_mean);
}

pub fn sent_mean(word_total: &f32, v: &Vec<&str>) -> f32 {
	let mut sent_mean: f32;
	let mut sent_total = 0;

	for i in v {
		sent_total += 1;
		println!("Sentence #{}", sent_total);
	}

	let total_sent = sent_total as f32;

	sent_mean = word_total / total_sent;

	return sent_mean;
}

