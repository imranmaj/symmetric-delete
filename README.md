# Symmetric Delete

A simple demo of a Symmetric Delete spelling correction algorithm based on [the SymSpell algorithm](https://github.com/wolfgarbe/SymSpell).

## Running

```
$ cargo run
```

## Usage

Wait for the words to finish processing, then enter words that are either spelled correctly or slightly misspelled. Words with correct spellings should be found which are close to the input.

## Explanation

This demo is based on the Symmetric Delete algorithm.

A naive spelling correction algorithm might look for variations of an input word in a dictionary of correctly spelled words. A slightly more advanced algorithm might precompute variations of the dictionary so that inputs can be checked against the variations in O(1) time via a hash table.

Symmetric Delete takes the latter idea further by computing variations on both the dictionary and the input itself. Specifically, letters are deleted from both the input and all words in the dictionary.

Normally we might think of two strings that we're comparing as having an [edit distance](https://en.m.wikipedia.org/wiki/Edit_distance); for example, the [Damerau-Levenshtein distance](https://en.m.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance) is the number of operations required to transform one string into another with the least number of insertions, deletions, substitutions, or adjacent transpositions. A spelling correction algorithm will try to find correctly spelled words with the smallest edit distance from an input word.

Symmetric Delete sidesteps the issue of considering insertions, deletions, substitutions, and transpositions separately by only considering deletions. This is possible because any edit operation on an input string to transform it into another string can be modeled instead by simply deleting characters that are out of place.

### Example:

```
input: "tubr"

dictionary: ["tub", "tube", "tubes", "tuber"]
```

We can see how "tubr" might be transformed into the various words in the dictionary. If we delete the "r", we get "tub"; if we replace the "r" with an "e" we get "tube"; if we insert an "e" we get "tuber".

Symmetric Delete can detect that "tubr" is close to these dictionary words by calculating all combinations of deletions of characters from "tubr" as well as all combinations of deletions from words in the dictionary. If we find a subsequence of characters from the original input that matches a subsequence of letters from a word in the dictionary, then we know that the input is close to a dictionary word. In this case, deleting the "r" will give us "tub". "tub" is 0 deletions away from the dictionary word "tub", 1 deletion away from "tube" (deleting the "e"), and 2 deletions away from both "tubes" and "tuber" (deleting the last 2 lettes in both words).

## Further reading

- [SymSpell repository](https://github.com/wolfgarbe/SymSpell)
- [Blog post by the SymSpell author](https://wolfgarbe.medium.com/1000x-faster-spelling-correction-algorithm-2012-8701fcd87a5f)
- [Edit Distance](https://en.m.wikipedia.org/wiki/Edit_distance)

## License

See LICENSE.md for the license under which this software is provided.
