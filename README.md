# sial
A tool to stymie the use of adversarial stylometry on oneself. It analyzes metadata from texts produced under an 'official' handle and a pseudonym, and returns statistics concerning each. Interpretation of the results is left to the user.

## Usage
./sial *file1* *file2*

## What is Adversarial Stylometry?
According to [Brennan et al.](https://www.cs.drexel.edu/~sa499/papers/adversarial_stylometry.pdf):
> The use of stylometry, authorship recognition through purely linguistic means, has contributed to literary, historical, and criminal investigation breakthroughs. Existing stylometry research assumes that authors have not attempted to disguise their linguistic writing style. We challenge this basic assumption of existing stylometry methodologies and present a new area of research: adversarial stylometry.

With the proliferation and permanence of online media, there is an increasing possibility of the opinions of one's pseudonym to be linked to a more 'official' presence, if their linguistic styles are similar. The purpose of this tool, at least initially, is to compare the similarities between text from an 'official' source and text that will be shared under a pseudonym. Features will be added/removed/modified in the future.

## Metadata Analyzed
Length:
+ Words
+ Sentences
+ Paragraphs

Frequency of Punctuation:
+ Commas
+ Semicolons
+ Exclamation marks
+ Hyphens

Frequency of Function Words:
+ *and*
+ *but*
+ *however*
+ *if*
+ *that*
+ *more*
+ *must*
+ *might*
+ *this*
+ *very*

All of these tests are referenced from [Ramyaa and Rasheed](http://www2.tcs.ifi.lmu.de/~ramyaa/publications/stylometry.pdf).

## TODO
+ Given a body of work, output similarity score to pseudonymous communication