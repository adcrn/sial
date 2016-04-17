pub fn num_words(words: &Vec<&str>) -> f32 {
    let num_words;
    
    num_words = words.len() as f32;

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
	let sent_total;

	sent_total = sentences.len() as f32;

	let total_sent = sent_total as f32;

	sent_mean = word_total / total_sent;

	return sent_mean;
}

pub fn word_freq(words: &Vec<&str>) {
	let mut num_and = 0;
	let mut num_but = 0;
	let mut num_however = 0;
	let mut num_if = 0;
	let mut num_what = 0;
	let mut num_more = 0;
	let mut num_must = 0;
	let mut num_might = 0;
	let mut num_this = 0;
	let mut num_very = 0;

	for &word in words {
		match word {
		    "and" => num_and +=1,
		    "but" => num_but +=1,
		    "however" => num_however +=1,
		    "if" => num_if +=1,
		    "what" => num_what +=1,
		    "more" => num_more +=1,
		    "must" => num_must +=1,
		    "might" => num_might +=1,
		    "this" => num_this +=1,
		    "very" => num_very +=1,
		    _ => continue,
		}
	}
}