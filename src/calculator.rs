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

pub fn word_freq(words: &Vec<&str>) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) {
	let mut num_and = 0;
	let mut num_but = 0;
	let mut num_however = 0;
	let mut num_if = 0;
	let mut num_that = 0;
	let mut num_more = 0;
	let mut num_must = 0;
	let mut num_might = 0;
	let mut num_this = 0;
	let mut num_very = 0;

	for &word in words {
		match word {
		    "and" | "And" | "AND" => num_and +=1,
		    "but" | "But" | "BUT" => num_but +=1,
		    "however" | "However" | "HOWEVER" => num_however +=1,
		    "if" | "If" | "IF" => num_if +=1,
		    "that" | "That" | "THAT" => num_that +=1,
		    "more" | "More" | "MORE" => num_more +=1,
		    "must" | "Must" | "MUST" => num_must +=1,
		    "might" | "Might" | "MIGHT" => num_might +=1,
		    "this" | "This" | "THIS" => num_this +=1,
		    "very" | "Very" | "VERY" => num_very +=1,
		    _ => continue,
		}
	}

	return (num_and, num_but, num_however, num_if, num_that, num_more, num_must, num_might, num_this, num_very);
}