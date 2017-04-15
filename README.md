# SpellingCorrection
The purpose of correct is to find possible corrections for misspelled words. It consists of two phases: The first phase is a training module, which consumes a corpus of correctly spelled words and counts the number of occurrences of each word. The second phase uses the results of the first to check individual words. Specifically, it checks whether each word is spelled correctly according to the training module and, if not, whether “small edits” can reach a variant that is correctly spelled.

Given a word, an edit action is one of the following:
the deletion of one letter;

the transposition of two neighboring letters;

the replacement of one letter with another letter; and

the insertion of a letter at any position.

In this context, Norvig suggests that “small edits” means the application of one edit action possibly followed by the application of a second one to the result of the first.
Once the second part has generated all possible candidate for a potentially misspelled word, it picks the most frequently used one from the training corpus. If none of the candidates is a correct word, correct reports a failure.


