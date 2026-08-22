# NLP — Complete Theory & NLTK Implementation Guide

A ground-up reference: the linguistics, the math, why each technique exists, when to reach for it over the alternatives, and how to implement it with NLTK. Read top to bottom to build a full mental model, or jump to a section before an interview or before writing code.

---

## Table of Contents

1. The NLP Pipeline — Big Picture
2. Corpora, Datasets & NLTK Resources
3. Text Preprocessing: Cleaning & Normalization
4. Tokenization
5. Stopword Removal
6. Stemming vs Lemmatization
7. Text Representation — BoW, TF-IDF, N-grams
8. Word Embeddings (Word2Vec, GloVe, FastText) — Theory
9. POS Tagging
10. Chunking, Chinking & Shallow Parsing
11. Named Entity Recognition (NER)
12. Parsing — Constituency & Dependency
13. WordNet & Lexical Semantics
14. Language Models & N-gram Smoothing
15. Text Classification (Naive Bayes, MaxEnt, SVM)
16. Sentiment Analysis (Lexicon-based & VADER)
17. Similarity & Distance Metrics
18. Sequence Labeling & the IOB Scheme
19. Topic Modeling (brief, outside NLTK)
20. Classical NLP vs Modern Transformer NLP
21. End-to-End NLTK Pipeline Example
22. "Which Algorithm Do I Use?" Cheat Sheet
23. Common Interview Questions & Answers

---

## 1. The NLP Pipeline — Big Picture

Every NLP system, classical or neural, is some subset of this pipeline:

```
Raw text
   │
   ▼
1. Cleaning/normalization (lowercase, remove noise, unicode fix)
   │
   ▼
2. Tokenization (split into words/sentences/subwords)
   │
   ▼
3. Stopword removal / stemming / lemmatization (optional, task-dependent)
   │
   ▼
4. Feature extraction / representation
     - Sparse: BoW, TF-IDF, n-grams
     - Dense: Word2Vec, GloVe, contextual embeddings (BERT-style)
   │
   ▼
5. Modeling
     - Classification: Naive Bayes, SVM, Logistic Regression, MaxEnt
     - Sequence labeling: HMM, CRF, BiLSTM-CRF
     - Parsing: CFG, dependency parsers
     - Generation: n-gram LM, seq2seq, Transformer
   │
   ▼
6. Evaluation (accuracy, F1, BLEU, perplexity, ...)
```

**Key mental model:** NLP is fundamentally about turning discrete, ambiguous, variable-length symbolic sequences into structured representations a model can compute over. Every technique in this doc is either (a) cleaning/normalizing the symbols, (b) converting symbols to numbers, or (c) modeling structure/relationships between symbols.

**Why NLTK specifically:** NLTK (Natural Language Toolkit) is a teaching- and research-oriented Python library — it's the best tool for understanding classical NLP algorithms explicitly (you can see the HMM transition matrix, the CFG grammar rules, the Naive Bayes feature counts). In production you'd typically use spaCy (speed, pretrained pipelines) or Hugging Face transformers (SOTA accuracy), but NLTK is unmatched for *learning why things work* — which is exactly what makes it valuable for interviews and building intuition.

```python
import nltk
# One-time resource downloads (interactive):
nltk.download('punkt')            # tokenizer models
nltk.download('punkt_tab')        # newer NLTK versions
nltk.download('stopwords')
nltk.download('wordnet')
nltk.download('omw-1.4')
nltk.download('averaged_perceptron_tagger')
nltk.download('averaged_perceptron_tagger_eng')
nltk.download('maxent_ne_chunker')
nltk.download('maxent_ne_chunker_tab')
nltk.download('words')
nltk.download('vader_lexicon')
nltk.download('conll2000')
nltk.download('treebank')
nltk.download('gutenberg')
nltk.download('brown')
```

---

## 2. Corpora, Datasets & NLTK Resources

NLTK bundles dozens of annotated corpora — invaluable because most classical algorithms (POS taggers, NER chunkers, PCFG parsers) need labeled data to train on.

| Corpus | Contains | Used for |
|---|---|---|
| `brown` | 500 tagged text samples, 15 genres | POS tagging, genre classification |
| `treebank` | Penn Treebank sample, POS + parse trees | Parsing, POS tagging |
| `conll2000` | Chunked (IOB) sentences | Chunking/shallow parsing training |
| `gutenberg` | Full public-domain books | Language modeling, general text |
| `movie_reviews` | 2000 labeled pos/neg reviews | Sentiment classification |
| `names` | Male/female first names | Classic Naive Bayes gender-classifier demo |
| `wordnet` | Lexical database (synsets, relations) | Synonyms, similarity, WSD |
| `stopwords` | Stopword lists, 20+ languages | Preprocessing |

```python
from nltk.corpus import brown, movie_reviews, treebank

print(brown.categories())                 # ['adventure', 'belles_lettres', ...]
print(brown.words(categories='news')[:10])
print(brown.tagged_words(categories='news')[:5])   # [('The','AT'), ('Fulton','NP-TL'), ...]

print(movie_reviews.fileids()[:3])
print(movie_reviews.categories())         # ['neg', 'pos']
```

**Interview angle:** know that corpora are the "training data" for classical statistical NLP — HMM taggers, PCFG parsers, and Naive Bayes classifiers all learn their parameters (probabilities) by counting occurrences in a labeled corpus. This is fundamentally different from how a Transformer learns (gradient descent on a loss over unlabeled/self-supervised objectives at massive scale), but the *concept* of learning distributional statistics from data is the throughline.

---

## 3. Text Preprocessing: Cleaning & Normalization

Goal: reduce noise and non-meaningful variation before the model ever sees the text.

Typical steps, roughly in order:

1. **Unicode normalization** — canonicalize characters (NFC/NFKC) so visually-identical characters compare equal.
2. **Lowercasing** — collapses `"Apple"` and `"apple"`; skip for case-sensitive tasks like NER (capitalization is a strong entity signal) or sentiment (ALL CAPS often signals intensity).
3. **Noise removal** — strip HTML tags, URLs, emails, extra whitespace, control characters.
4. **Punctuation handling** — remove or isolate, depending on downstream tokenizer.
5. **Number handling** — remove, replace with a placeholder token (`<NUM>`), or keep, depending on task.
6. **Spelling/contraction normalization** — `"don't"` → `"do not"`, `"u"` → `"you"` for noisy social text.

```python
import re
import unicodedata

def clean_text(text: str) -> str:
    text = unicodedata.normalize("NFKC", text)
    text = re.sub(r"http\S+|www\.\S+", " ", text)      # URLs
    text = re.sub(r"\S+@\S+", " ", text)                # emails
    text = re.sub(r"<[^>]+>", " ", text)                # HTML tags
    text = re.sub(r"[^a-zA-Z0-9\s']", " ", text)        # keep alnum, space, apostrophe
    text = re.sub(r"\s+", " ", text).strip()
    return text.lower()

clean_text("Check THIS out: <b>http://example.com</b> — email me@x.com!!")
# "check this out"
```

**Why order matters:** strip URLs/HTML *before* stripping punctuation, or you'll leave garbage fragments (`httpexamplecom`). Lowercase *after* NER-relevant steps if you need casing for entity detection.

**Interview trap:** "just remove all punctuation" breaks negation contractions (`"don't"` → `"dont"`, which some tokenizers then treat as one token, losing the negation signal that matters a lot for sentiment). Always ask what the downstream task needs before you normalize aggressively.

---

## 4. Tokenization

Tokenization splits text into meaningful units: sentences, words, or subwords. It looks trivial but is full of edge cases: `"Dr. Smith paid $3.5 million to Yahoo! Inc."` — periods, dollar signs, and exclamation marks aren't always boundaries.

### Types

- **Sentence tokenization** — split on sentence boundaries (handles abbreviations like "Dr.", "e.g.").
- **Word tokenization** — split into words, separating punctuation as its own tokens (`"don't"` → `["do", "n't"]` in Penn Treebank convention).
- **Subword tokenization** (BPE, WordPiece, SentencePiece) — used by Transformers, not classical NLTK, but important to know: splits rare/unknown words into known sub-units (`"unhappiness"` → `["un", "happiness"]`), which solves the out-of-vocabulary (OOV) problem that plagues word-level tokenizers.

### NLTK implementations

```python
from nltk.tokenize import (
    word_tokenize, sent_tokenize,
    RegexpTokenizer, TreebankWordTokenizer,
    WhitespaceTokenizer, TweetTokenizer
)

text = "Dr. Smith isn't happy. He said, \"NLTK's great!\" #excited @nlp_fan"

# Sentence tokenizer (Punkt — unsupervised, learns abbreviation patterns)
print(sent_tokenize(text))
# ["Dr. Smith isn't happy.", 'He said, "NLTK\'s great!"', '#excited @nlp_fan']

# Word tokenizer (Penn Treebank conventions: splits contractions, quotes)
print(word_tokenize(text))
# ['Dr.', 'Smith', 'is', "n't", 'happy', '.', 'He', 'said', ',', '``', 'NLTK', "'s", 'great', '!', "''", '#', 'excited', '@', 'nlp_fan']

# Regex tokenizer — full control, e.g. words-only, no punctuation
tokenizer = RegexpTokenizer(r"\w+")
print(tokenizer.tokenize(text))
# ['Dr', 'Smith', 'isn', 't', 'happy', 'He', 'said', 'NLTK', 's', 'great', 'excited', 'nlp_fan']

# Whitespace-only tokenizer — fastest, no linguistic awareness
print(WhitespaceTokenizer().tokenize(text))

# Tweet tokenizer — preserves hashtags, mentions, emoticons as single tokens
tweet_tok = TweetTokenizer()
print(tweet_tok.tokenize("OMG this is SO cool!! :) #NLTK @user"))
# ['OMG', 'this', 'is', 'SO', 'cool', '!', '!', ':)', '#NLTK', '@user']
```

**How Punkt's sentence tokenizer actually works:** it's an *unsupervised* algorithm (Kiss & Strunk, 2006) that builds a statistical model of which tokens are likely abbreviations by looking at collocation frequency, word length, and internal periods — it does **not** use a hardcoded abbreviation list. That's why it generalizes to new domains without retraining, and why you can train your own `PunktSentenceTokenizer` on domain text.

```python
from nltk.tokenize import PunktSentenceTokenizer
custom_tokenizer = PunktSentenceTokenizer(train_text)   # unsupervised training
custom_tokenizer.tokenize(new_text)
```

**Choosing a tokenizer:**

| Situation | Use |
|---|---|
| General English text | `word_tokenize` (Treebank conventions) |
| Social media / tweets | `TweetTokenizer` (keeps `#tag`, `@user`, emoticons intact) |
| Need only alphanumeric words, no punctuation | `RegexpTokenizer(r"\w+")` |
| Domain-specific abbreviations (legal, medical) | Train a custom `PunktSentenceTokenizer` |
| Feeding a Transformer model | Use the model's own subword tokenizer (BPE/WordPiece), not NLTK |

---

## 5. Stopword Removal

Stopwords are high-frequency, low-information words (`"the"`, `"is"`, `"and"`) that add little discriminative signal for tasks like classification or search, but carry real weight in others.

```python
from nltk.corpus import stopwords
from nltk.tokenize import word_tokenize

stop_words = set(stopwords.words('english'))
tokens = word_tokenize("This is not the movie I wanted to watch")
filtered = [w for w in tokens if w.lower() not in stop_words]
print(filtered)   # ['movie', 'wanted', 'watch']
```

**When to skip stopword removal:**
- **Sentiment analysis / negation-sensitive tasks:** removing `"not"`, `"no"`, `"never"` destroys the polarity signal (`"not good"` → `"good"` is a disaster). Use a custom stopword list that excludes negators, or don't remove stopwords at all.
- **Phrase/n-gram search, machine translation, POS tagging, parsing:** stopwords carry syntactic structure — removing them breaks grammar entirely.
- **Neural/Transformer pipelines:** almost never remove stopwords — attention mechanisms use them for structure, and subword tokenizers already handle frequency implicitly via BPE merge counts.

**When it helps:** classical bag-of-words / TF-IDF pipelines for topic classification, information retrieval indexing, and keyword extraction, where stopwords are pure noise that dilutes the vector's discriminative signal.

---

## 6. Stemming vs Lemmatization

Both reduce inflected word forms to a base form, but by fundamentally different mechanisms.

**Stemming** — crude, rule-based suffix stripping. Fast, no dictionary lookup, can produce non-words.

```python
from nltk.stem import PorterStemmer, SnowballStemmer, LancasterStemmer

porter = PorterStemmer()
snowball = SnowballStemmer("english")   # improved, multi-language Porter2
lancaster = LancasterStemmer()          # very aggressive

words = ["running", "runs", "ran", "easily", "fairly", "studies", "university"]
print([porter.stem(w) for w in words])
# ['run', 'run', 'ran', 'easili', 'fairli', 'studi', 'univers']
print([lancaster.stem(w) for w in words])
# ['run', 'run', 'ran', 'easy', 'fair', 'study', 'univers']  (more aggressive, more errors)
```

Note `"ran"` stays `"ran"` — Porter is purely suffix-based and has no knowledge that "ran" is the past tense of "run." And `"easili"` isn't a real word — that's the tradeoff of speed over correctness.

**Lemmatization** — dictionary/morphology-aware, returns a real base word (the *lemma*), but needs to know the POS to disambiguate (e.g., "meeting" as noun vs. verb lemmatizes differently).

```python
from nltk.stem import WordNetLemmatizer
from nltk.corpus import wordnet

lemmatizer = WordNetLemmatizer()

print(lemmatizer.lemmatize("running"))            # 'running' (default pos='n', wrong!)
print(lemmatizer.lemmatize("running", pos='v'))    # 'run'     (correct with POS hint)
print(lemmatizer.lemmatize("better", pos='a'))      # 'good'    (adjective, uses WordNet)
print(lemmatizer.lemmatize("studies", pos='v'))     # 'study'
print(lemmatizer.lemmatize("ran", pos='v'))         # 'run'     (knows irregular verbs)

# Map Treebank POS tags -> WordNet POS tags to lemmatize correctly in a pipeline
def get_wordnet_pos(treebank_tag):
    if treebank_tag.startswith('J'):
        return wordnet.ADJ
    elif treebank_tag.startswith('V'):
        return wordnet.VERB
    elif treebank_tag.startswith('N'):
        return wordnet.NOUN
    elif treebank_tag.startswith('R'):
        return wordnet.ADV
    return wordnet.NOUN   # default
```

**Comparison:**

| | Stemming | Lemmatization |
|---|---|---|
| Method | Rule-based suffix stripping | Dictionary + morphological analysis |
| Speed | Very fast | Slower (dictionary lookups) |
| Output | May be a non-word ("studi") | Always a real word |
| Needs POS? | No | Yes, for accuracy |
| Accuracy | Lower, more collisions | Higher |
| Use when | Search engines, IR indexing, speed-critical | Chatbots, QA, anything user-facing or meaning-sensitive |

**Interview answer, condensed:** *"Stemming is fast and crude — it chops suffixes with fixed rules and can produce non-words; use it for search/IR where you just need consistent bucketing. Lemmatization uses vocabulary and morphology (via WordNet in NLTK) to return a real dictionary base form, but needs the POS tag to disambiguate correctly, so it's slower but more accurate — use it whenever output is user-facing or semantic precision matters."*

---

## 7. Text Representation — BoW, TF-IDF, N-grams

Models need numbers, not strings. These are the classical ("sparse") ways to vectorize text, as opposed to dense embeddings (Section 8).

### Bag of Words (BoW)

Represents a document as a vector of word counts, ignoring order and grammar entirely.

```python
from sklearn.feature_extraction.text import CountVectorizer
# (NLTK doesn't ship a vectorizer; sklearn is the standard pairing — very common combo)

docs = ["I love NLP", "NLP is great", "I love great movies"]
vectorizer = CountVectorizer()
X = vectorizer.fit_transform(docs)
print(vectorizer.get_feature_names_out())   # ['great' 'is' 'love' 'movies' 'nlp']
print(X.toarray())
# [[0 0 1 0 1]
#  [1 1 0 0 1]
#  [1 0 1 1 0]]
```

You can build BoW manually with NLTK's `FreqDist`:

```python
from nltk import FreqDist
from nltk.tokenize import word_tokenize

fd = FreqDist(word_tokenize("the cat sat on the mat the cat ran"))
print(fd.most_common(3))   # [('the', 3), ('cat', 2), ('sat', 1)]
fd.plot(5)                 # frequency plot (needs matplotlib)
```

**Weakness:** loses word order (`"dog bites man"` == `"man bites dog"` as vectors), and treats every word as equally important — `"the"` gets the same weight as `"excellent"`.

### TF-IDF (Term Frequency – Inverse Document Frequency)

Fixes BoW's "every word equally important" problem by down-weighting words common across *all* documents (uninformative) and up-weighting words rare across documents but frequent in *this* one (informative/discriminative).

```
TF(t, d)  = (count of t in d) / (total terms in d)
IDF(t)    = log( N / (1 + df(t)) )     # N = total docs, df(t) = docs containing t
TF-IDF(t, d) = TF(t, d) * IDF(t)
```

```python
from sklearn.feature_extraction.text import TfidfVectorizer

tfidf = TfidfVectorizer()
X = tfidf.fit_transform(docs)
print(tfidf.get_feature_names_out())
print(X.toarray().round(2))
```

**Intuition:** IDF is the key idea — a word appearing in every document (like "the") gives `log(N / N) ≈ 0`, killing its weight; a word appearing in only one document out of a thousand gives a large IDF, boosting it. This is exactly why TF-IDF is still the default baseline for search engines and document similarity/keyword extraction — it's cheap, interpretable, and surprisingly hard to beat for pure keyword-matching tasks.

### N-grams

Captures local word order by grouping *n* consecutive tokens — a cheap way to partially recover the ordering info that BoW throws away.

```python
from nltk import ngrams, bigrams, trigrams
from nltk.tokenize import word_tokenize

tokens = word_tokenize("I love natural language processing")
print(list(bigrams(tokens)))
# [('I', 'love'), ('love', 'natural'), ('natural', 'language'), ('language', 'processing')]
print(list(ngrams(tokens, 3)))   # trigrams
# [('I', 'love', 'natural'), ('love', 'natural', 'language'), ...]

# Feed n-grams into CountVectorizer/TfidfVectorizer directly:
bigram_vectorizer = CountVectorizer(ngram_range=(1, 2))   # unigrams + bigrams
```

**Why n-grams matter:** `"not good"` as a bigram carries opposite sentiment to `"good"` alone — unigram BoW can't capture that, bigram BoW can. Tradeoff: vocabulary size explodes combinatorially with n (sparsity, memory), so in practice n=1–3 is the practical ceiling for classical pipelines; beyond that, dense embeddings/contextual models win decisively.

---

## 8. Word Embeddings (Word2Vec, GloVe, FastText) — Theory

NLTK itself doesn't train embeddings (that's `gensim`'s job), but embeddings are core NLP theory and interviewers expect you to explain them precisely.

**Core idea — the distributional hypothesis:** *"You shall know a word by the company it keeps"* (Firth, 1957). Words that appear in similar contexts have similar meanings, so if you train a model to predict a word from its context (or vice versa), the model's internal weights become a dense vector representation where semantically similar words end up close together.

### Word2Vec (Mikolov et al., 2013) — two architectures

- **CBOW (Continuous Bag of Words):** predict the center word from surrounding context words. Faster to train, works better for frequent words.
  ```
  context = [w(t-2), w(t-1), w(t+1), w(t+2)]  →  predict w(t)
  ```
- **Skip-gram:** predict surrounding context words from the center word. Slower, but works better for rare words and small datasets.
  ```
  w(t)  →  predict [w(t-2), w(t-1), w(t+1), w(t+2)]
  ```

Trained with **negative sampling** (turn the huge softmax over the whole vocabulary into a cheap binary classification: "is this word-context pair real or a randomly sampled fake?") for tractability.

### GloVe (Global Vectors, Pennington et al., 2014)

Instead of a sliding-window predictive objective, GloVe factorizes a **global word-word co-occurrence matrix** directly — it explicitly optimizes so that the dot product of two word vectors approximates the log of their co-occurrence probability ratio. Captures global corpus statistics rather than only local context windows.

### FastText (Facebook, 2016)

Extends Word2Vec by representing each word as a bag of **character n-grams** (e.g., `"where"` → `<wh, whe, her, ere, re>`) and summing their vectors. This means it can produce a reasonable vector for **out-of-vocabulary words** (typos, rare morphological forms) by composing from known subword pieces — a real weakness of vanilla Word2Vec/GloVe, which assign OOV words nothing.

### Key emergent property: vector arithmetic captures analogies

```
vector("king") - vector("man") + vector("woman") ≈ vector("queen")
```

This works because the *offset direction* between related word pairs (gender, in this case) is roughly consistent across the vector space — a strong empirical signal that these embeddings capture real semantic/syntactic relationships, not just co-occurrence noise.

```python
# Using gensim alongside NLTK-tokenized corpus (the standard real-world pairing)
from gensim.models import Word2Vec
from nltk.tokenize import word_tokenize, sent_tokenize
from nltk.corpus import gutenberg

sentences = [word_tokenize(s.lower()) for s in sent_tokenize(gutenberg.raw('austen-emma.txt'))]
model = Word2Vec(sentences, vector_size=100, window=5, min_count=2, sg=1)  # sg=1 -> skip-gram
print(model.wv.most_similar("marriage"))
print(model.wv.similarity("man", "woman"))
```

**Classical (sparse: BoW/TF-IDF) vs Dense (Word2Vec/GloVe) vs Contextual (BERT):**

| | Sparse (TF-IDF) | Static dense (Word2Vec/GloVe) | Contextual (BERT/Transformers) |
|---|---|---|---|
| Vector size | Vocabulary-sized (huge, sparse) | Fixed (e.g. 100–300 dims) | Fixed (e.g. 768+ dims) |
| Captures meaning? | No (just frequency) | Yes, but one vector per word regardless of context | Yes, vector changes per sentence context |
| Handles polysemy? | No | No — "bank" (river) and "bank" (finance) share one vector | Yes — different vectors per usage |
| Training cost | ~Free (counting) | Cheap (shallow network) | Very expensive (deep network, huge data) |
| OOV handling | N/A (vocab-fixed) | Poor (FastText improves this) | Good (subword tokenization) |

**Interview angle:** the single most important distinction is *static vs. contextual*. Word2Vec/GloVe give `"bank"` one fixed vector no matter the sentence; BERT gives `"bank"` a different vector in `"river bank"` vs. `"bank account"` because the representation is computed through self-attention over the full sentence at inference time. This is *the* reason Transformers displaced static embeddings for anything meaning-sensitive.

---

## 9. POS Tagging

Assigns a grammatical category (noun, verb, adjective, ...) to each token. Foundational — POS tags feed into chunking, NER, parsing, and lemmatization.

### Tagset

NLTK defaults to the **Penn Treebank tagset** (`NN`=noun, `VB`=verb base, `VBD`=verb past tense, `JJ`=adjective, `RB`=adverb, `DT`=determiner, `IN`=preposition, ...) — 36 fine-grained tags. A simplified **Universal POS tagset** (`NOUN`, `VERB`, `ADJ`, ...) has ~12 tags and is more portable across languages.

```python
from nltk import pos_tag, word_tokenize

tokens = word_tokenize("The quick brown fox jumps over the lazy dog")
print(pos_tag(tokens))
# [('The','DT'), ('quick','JJ'), ('brown','JJ'), ('fox','NN'), ('jumps','VBZ'),
#  ('over','IN'), ('the','DT'), ('lazy','JJ'), ('dog','NN')]

print(pos_tag(tokens, tagset='universal'))
# [('The','DET'), ('quick','ADJ'), ('brown','ADJ'), ('fox','NOUN'), ('jumps','VERB'), ...]
```

NLTK's default `pos_tag` uses a pretrained **Averaged Perceptron Tagger** — fast, ~97% accurate on standard English, learned via structured perceptron updates over labeled Treebank data.

### Algorithm choices, in increasing sophistication

1. **Rule-based (e.g., Brill tagger):** starts with the most-frequent-tag baseline, then applies learned transformation rules ("change tag from X to Y when previous word is Z") iteratively. Interpretable, but fragile to unseen patterns.
2. **Hidden Markov Model (HMM):** models tagging as finding the most probable tag sequence given the word sequence, using the **Viterbi algorithm** for efficient search:
   ```
   P(tags | words) ∝ P(words | tags) * P(tags)
                    = Π P(word_i | tag_i) * Π P(tag_i | tag_{i-1})   [bigram HMM]
   ```
   Emission probabilities `P(word|tag)` and transition probabilities `P(tag_i|tag_{i-1})` are estimated by counting frequencies in a tagged corpus.
3. **Maximum Entropy / Averaged Perceptron:** discriminative models using rich, overlapping features (word shape, prefix/suffix, surrounding words/tags, capitalization) rather than the strict Markov independence assumptions of HMMs — this is what NLTK's default tagger uses, and why it beats plain HMMs in accuracy.
4. **CRF (Conditional Random Field):** globally normalizes over the whole tag sequence (rather than greedily/locally like a perceptron), avoiding label-bias problems. The go-to classical model for sequence labeling before neural nets.
5. **BiLSTM-CRF / Transformer:** neural contextual features feeding a CRF (or straight softmax) output layer — modern SOTA, outside NLTK's scope.

```python
# Training your own HMM tagger on the Brown corpus with NLTK
from nltk.corpus import brown
from nltk.tag import hmm

train_data = brown.tagged_sents(categories='news')[:3000]
trainer = hmm.HiddenMarkovModelTrainer()
hmm_tagger = trainer.train_supervised(train_data)
print(hmm_tagger.tag(word_tokenize("The dog runs fast")))
print(hmm_tagger.evaluate(brown.tagged_sents(categories='news')[3000:3500]))  # older NLTK
# newer NLTK: nltk.tag.accuracy(hmm_tagger, test_data)

# A simple backoff chain: Unigram -> Bigram -> Default tagger (classic NLTK pattern)
from nltk.tag import UnigramTagger, BigramTagger, DefaultTagger

train_sents = brown.tagged_sents(categories='news')[:3000]
test_sents = brown.tagged_sents(categories='news')[3000:3500]

default_tagger = DefaultTagger('NN')                       # fallback: guess noun
unigram_tagger = UnigramTagger(train_sents, backoff=default_tagger)
bigram_tagger = BigramTagger(train_sents, backoff=unigram_tagger)
print(bigram_tagger.evaluate(test_sents))   # or nltk.tag.accuracy(bigram_tagger, test_sents)
```

**The backoff pattern is a core NLTK idiom worth internalizing:** try the most context-sensitive tagger first (bigram — looks at previous tag), fall back to less context-sensitive (unigram — looks at word frequency alone) when the bigram tagger has no data for that context, fall back further to a dumb default. This gracefully handles data sparsity — exactly the same idea behind smoothing in language models (Section 14).

**When to use which:** rule-based/backoff-taggers for teaching and lightweight cases; HMM when you specifically need a generative probabilistic model (e.g., you also want `P(sequence)`); Averaged Perceptron/MaxEnt (NLTK's default) for the best accuracy/speed tradeoff in classical pipelines; CRF/BiLSTM-CRF when tag interdependencies are strong and you have the data/compute for a neural pipeline.

---

## 10. Chunking, Chinking & Shallow Parsing

**Chunking** groups tagged tokens into higher-level, non-overlapping, non-recursive phrases — most commonly **noun phrases (NP)** — without building a full parse tree. This is "shallow parsing": cheaper than full syntactic parsing but gives you useful phrase-level structure (critical groundwork for NER, Section 11).

```python
from nltk import pos_tag, word_tokenize, RegexpParser

sentence = "The little yellow dog barked at the cat"
tagged = pos_tag(word_tokenize(sentence))

# Grammar rule: NP = optional determiner, any adjectives, then a noun
grammar = "NP: {<DT>?<JJ>*<NN>}"
chunk_parser = RegexpParser(grammar)
tree = chunk_parser.parse(tagged)
print(tree)
# (S (NP The/DT little/JJ yellow/JJ dog/NN) barked/VBD at/IN (NP the/DT cat/NN))
tree.draw()   # opens a GUI tree visualization
```

**Chinking** is the inverse — define what to *exclude* from a chunk rather than what to include, useful when the "include" pattern is harder to specify than the "exclude" pattern:

```python
grammar = r"""
  NP:
    {<.*>+}          # chunk everything
    }<VBD|IN>+{       # chink (exclude) verbs and prepositions
"""
```

### IOB tagging — the standard chunk representation

Chunks are conventionally encoded per-token as **I**nside, **O**utside, **B**egin:

```
The      B-NP
little   I-NP
yellow   I-NP
dog      I-NP
barked   O
at       O
the      B-NP
cat      I-NP
```

```python
from nltk.chunk import tree2conlltags, conlltags2tree
print(tree2conlltags(tree))
# [('The','DT','B-NP'), ('little','JJ','I-NP'), ('yellow','JJ','I-NP'),
#  ('dog','NN','I-NP'), ('barked','VBD','O'), ('at','IN','O'),
#  ('the','DT','B-NP'), ('cat','NN','I-NP')]
```

### Training a statistical chunker (instead of hand-written grammar rules)

```python
from nltk.corpus import conll2000
from nltk.chunk import ChunkParserI
from nltk.tag import UnigramTagger, BigramTagger

class BigramChunker(ChunkParserI):
    def __init__(self, train_sents):
        train_data = [[(t, c) for _, t, c in tree2conlltags(sent)] for sent in train_sents]
        self.tagger = BigramTagger(train_data, backoff=UnigramTagger(train_data))

    def parse(self, sentence):
        pos_tags = [t for _, t in sentence]
        tagged = self.tagger.tag(pos_tags)
        conlltags = [(w, t, c) for (w, t), (_, c) in zip(sentence, tagged)]
        return conlltags2tree(conlltags)

train_sents = conll2000.chunked_sents('train.txt', chunk_types=['NP'])
test_sents = conll2000.chunked_sents('test.txt', chunk_types=['NP'])
chunker = BigramChunker(train_sents)
print(chunker.evaluate(test_sents))   # precision/recall/F-measure/accuracy
```

This reframes chunking as a **sequence tagging problem over IOB labels** — exactly the same trick used for NER (Section 11) and, later, neural sequence labeling (BiLSTM-CRF). Recognizing "IOB tagging = sequence labeling" is a key unifying insight across NLTK's chunking, NER, and modern token-classification models.

---

## 11. Named Entity Recognition (NER)

Identifies and classifies spans of text into predefined categories: `PERSON`, `ORGANIZATION`, `GPE` (geo-political entity), `LOCATION`, `DATE`, `MONEY`, etc.

```python
from nltk import pos_tag, word_tokenize, ne_chunk

sentence = "Barack Obama was born in Hawaii and worked at Google."
tagged = pos_tag(word_tokenize(sentence))
tree = ne_chunk(tagged)
print(tree)
# (S (PERSON Barack/NNP) (PERSON Obama/NNP) was/VBD born/VBN in/IN
#    (GPE Hawaii/NNP) and/CC worked/VBD at/IN (ORGANIZATION Google/NNP) ./.)

for subtree in tree:
    if hasattr(subtree, 'label'):
        entity = " ".join(w for w, t in subtree.leaves())
        print(entity, "->", subtree.label())
# Barack Obama -> PERSON   (note: NLTK often splits multi-token PERSON entities — a known weakness)
# Hawaii -> GPE
# Google -> ORGANIZATION
```

NLTK's built-in `ne_chunk` uses a pretrained MaxEnt classifier trained on ACE corpus data, treating NER as **IOB sequence tagging** (Section 10) with rich features: word shape, capitalization, POS tag, gazetteer/lookup-list membership, and surrounding context.

### Why NER is hard

- **Ambiguity:** `"Washington"` could be a person, a state, or a city — resolved only by context.
- **Nested/overlapping entities:** `"Bank of America Tower"` — is the whole thing an ORG, or is "America" also a nested GPE?
- **Novel entities:** product names, new companies not seen in training data — this is why gazetteers (lookup lists) alone fail and contextual features/models matter.

**Practical note:** NLTK's built-in NER chunker is a solid teaching example but is noticeably less accurate than spaCy's pretrained NER or a fine-tuned Transformer (e.g., `bert-base-NER`) — in production, reach for those. NLTK is the right tool when you need to **train a custom chunker on domain-specific IOB-labeled data** and want full visibility into the feature engineering.

---

## 12. Parsing — Constituency & Dependency

Full syntactic parsing builds a complete structural tree over a sentence, going deeper than chunking's flat phrase groups.

### Constituency parsing (phrase-structure grammar)

Represents sentence structure as nested phrases (NP, VP, PP, ...) using a **Context-Free Grammar (CFG)**.

```python
import nltk

grammar = nltk.CFG.fromstring("""
  S -> NP VP
  NP -> Det N | Det N PP
  VP -> V NP | VP PP
  PP -> P NP
  Det -> 'the' | 'a'
  N -> 'dog' | 'cat' | 'park'
  V -> 'chased'
  P -> 'in'
""")

parser = nltk.ChartParser(grammar)
sentence = "the dog chased a cat in the park".split()
for tree in parser.parse(sentence):
    print(tree)
    tree.draw()
# (S
#   (NP (Det the) (N dog))
#   (VP
#     (VP (V chased) (NP (Det a) (N cat)))
#     (PP (P in) (NP (Det the) (N park)))))
```

Ambiguity is real: the PP `"in the park"` could attach to the VP (chasing happened in the park) or to the NP `"cat"` (a cat that is in the park) — a CFG parser can legitimately return **both** trees, and resolving which is intended requires semantics/statistics, not just grammar (this is the classic **PP-attachment ambiguity** problem, a favorite interview topic).

**PCFG (Probabilistic CFG)** attaches a probability to each production rule (learned from a treebank), letting the parser rank ambiguous parses by likelihood instead of returning all of them undifferentiated:

```python
pcfg_grammar = nltk.PCFG.fromstring("""
  S -> NP VP [1.0]
  NP -> Det N [0.5] | Det N PP [0.5]
  VP -> V NP [0.7] | VP PP [0.3]
  ...
""")
parser = nltk.ViterbiParser(pcfg_grammar)   # returns the single most probable parse
```

### Dependency parsing

Represents structure as directed binary relations between a **head** word and its **dependents**, rather than nested phrases — closer to how modern parsers (and Universal Dependencies, spaCy's default) work.

```python
from nltk.parse import DependencyGraph

# NLTK can parse CoNLL-style dependency notation, but doesn't ship a trainable
# statistical dependency parser out of the box — pair with spaCy for real dependency
# parsing; NLTK is best here for understanding the *representation*.
dep_tree_str = """
dog NN 2 nsubj
chased VBD 0 root
cat NN 2 dobj
"""
dg = DependencyGraph(dep_tree_str)
print(dg.tree())
```

**Constituency vs Dependency — the interview-ready distinction:**

| | Constituency | Dependency |
|---|---|---|
| Structure | Nested phrases (NP, VP, ...) | Direct head→dependent word relations |
| Grammar formalism | CFG / PCFG | Dependency grammar |
| Output | Tree with phrase-category internal nodes | Tree with words as *all* nodes, labeled edges |
| Better for | Understanding phrase structure, generation | Understanding "who did what to whom" — relation extraction, information extraction |
| Modern default | Less common now | Dominant (Universal Dependencies, spaCy, most Treebanks) |

---

## 13. WordNet & Lexical Semantics

WordNet is a large lexical database organizing English words into **synsets** (sets of cognitive synonyms), linked by semantic relations: hypernym (is-a, more general), hyponym (is-a, more specific), meronym (part-of), antonym, etc.

```python
from nltk.corpus import wordnet as wn

synsets = wn.synsets("dog")
print(synsets[0].definition())     # 'a member of the genus Canis...'
print(synsets[0].examples())

dog = wn.synset('dog.n.01')
print(dog.hypernyms())             # [Synset('canine.n.02')]        -- more general
print(dog.hyponyms()[:3])          # more specific breeds/types
print(dog.part_meronyms())         # parts of a dog (e.g., 'flag.n.07' -- tail)

# Synonyms / antonyms via lemmas
for lemma in wn.synset('good.a.01').lemmas():
    print(lemma.name(), lemma.antonyms())
```

### Semantic similarity between words

```python
dog = wn.synset('dog.n.01')
cat = wn.synset('cat.n.01')

print(dog.path_similarity(cat))      # 1 / (shortest path length in hypernym tree + 1)
print(dog.wup_similarity(cat))       # Wu-Palmer: based on depth of LCS (least common subsumer)
print(dog.lch_similarity(cat))       # Leacock-Chodorow: -log(path length / 2*max depth)
```

- **Path similarity:** inverse of the shortest hypernym-tree path between two synsets — simple, purely structural.
- **Wu-Palmer similarity:** uses the depth of the **least common subsumer (LCS)** relative to the depth of each synset — accounts for how deep/specific the shared ancestor is, generally more meaningful than raw path length.
- **Leacock-Chodorow similarity:** scales path length by the max taxonomy depth — normalizes for how "deep" the whole hierarchy is.

### Word Sense Disambiguation (WSD) — the Lesk algorithm

Given a word with multiple senses (`"bank"`: river edge vs. financial institution), pick the sense whose WordNet gloss (definition) has the most word overlap with the surrounding sentence context.

```python
from nltk.wsd import lesk
from nltk.tokenize import word_tokenize

sentence = word_tokenize("I went to the bank to deposit money")
sense = lesk(sentence, 'bank')
print(sense, sense.definition())
# Synset('bank.n.02') 'a financial institution...' -- picks correctly given "deposit money" context

sentence2 = word_tokenize("I sat by the river bank and fished")
sense2 = lesk(sentence2, 'bank')
print(sense2, sense2.definition())
```

**Why Lesk matters conceptually even though it's a weak baseline:** it's the simplest possible instance of "use context to disambiguate meaning" — the exact same problem contextual embeddings (BERT) solve far more powerfully by learning dense contextual representations instead of doing discrete dictionary-gloss overlap. Knowing Lesk gives you the classical baseline to contrast against when explaining *why* contextual embeddings were such a leap forward for WSD-dependent tasks.

---

## 14. Language Models & N-gram Smoothing

A language model assigns a probability to a sequence of words, `P(w1, w2, ..., wn)`. Classical (pre-neural) LMs are built on the **Markov assumption**: the probability of the next word depends only on the previous *n-1* words.

```
Bigram model:  P(w1...wn) ≈ Π P(w_i | w_{i-1})
Trigram model: P(w1...wn) ≈ Π P(w_i | w_{i-2}, w_{i-1})
```

Each conditional probability is estimated via **Maximum Likelihood Estimation** — just counting:

```
P(w_i | w_{i-1}) = count(w_{i-1}, w_i) / count(w_{i-1})
```

### The zero-probability problem, and smoothing

MLE assigns probability **zero** to any n-gram unseen in training — which then makes the probability of the *entire sentence* zero, even if only one n-gram in it was unseen. This is unacceptable, so smoothing techniques redistribute some probability mass to unseen events:

- **Laplace (add-one) smoothing:** add 1 to every count. Simple but crude — over-smooths, especially with large vocabularies.
  ```
  P(w_i | w_{i-1}) = (count(w_{i-1}, w_i) + 1) / (count(w_{i-1}) + V)     # V = vocab size
  ```
- **Add-k smoothing:** generalize to adding `k < 1` — less aggressive than add-one.
- **Backoff (Katz backoff):** if the trigram is unseen, "back off" to the bigram estimate; if that's also unseen, back off further to unigram — the same backoff idea as the POS tagger chain in Section 9.
- **Interpolation:** instead of an all-or-nothing backoff, take a **weighted combination** of unigram, bigram, and trigram estimates simultaneously — generally outperforms pure backoff.
- **Kneser-Ney smoothing:** the strongest classical technique — instead of just discounting raw counts, it models how many *distinct contexts* a word appears in (its "continuation probability"), which better captures words that are versatile across contexts vs. words that only ever follow one specific word (e.g., "Francisco" almost only follows "San" — Kneser-Ney correctly penalizes it as a poor generic backoff candidate despite decent raw frequency).

```python
from nltk.lm.preprocessing import padded_everygram_pipeline
from nltk.lm import MLE, Laplace, KneserNeyInterpolated
from nltk.tokenize import word_tokenize, sent_tokenize
from nltk.corpus import gutenberg

text = gutenberg.raw('austen-emma.txt')
sentences = [word_tokenize(s.lower()) for s in sent_tokenize(text)]

n = 3   # trigram model
train_data, padded_vocab = padded_everygram_pipeline(n, sentences)

model = KneserNeyInterpolated(n)
model.fit(train_data, padded_vocab)

print(model.score("emma", ["i", "am"]))          # P("emma" | "i", "am")
print(model.generate(10, text_seed=["i", "am"]))  # generate 10 words

# Perplexity -- the standard LM evaluation metric (lower = better)
test_ngrams = list(nltk.trigrams(word_tokenize("i am very happy today")))
print(model.perplexity(test_ngrams))
```

### Perplexity — how LMs are evaluated

```
Perplexity(W) = P(w1, w2, ..., wN) ^ (-1/N)  =  2 ^ (cross-entropy)
```

Interpreted as the **weighted average branching factor** — "on average, how many equally likely next-word choices was the model juggling at each step?" Lower perplexity = the model is less "surprised" by the test data = better fit. This is the direct classical ancestor of the loss functions used to train and evaluate modern LLMs (cross-entropy loss on next-token prediction is literally the same objective, just at neural scale).

**Interview framing:** *"N-gram models are the conceptual ancestor of GPT-style models — both are next-token predictors trained on the same objective (maximize likelihood of the observed sequence / minimize cross-entropy). The difference is n-gram models condition on a fixed, tiny window (Markov assumption) and estimate probabilities by counting, while Transformers condition on the full context via self-attention and estimate probabilities via a deep neural network — trading interpretability and cheap training for vastly better long-range coherence."*

---

## 15. Text Classification (Naive Bayes, MaxEnt, SVM)

The classical supervised-learning workflow: extract features from text, train a classifier on labeled examples.

### Naive Bayes — the NLTK-native default

Applies Bayes' theorem with the ("naive") assumption that features are conditionally independent given the class:

```
P(class | features) ∝ P(class) * Π P(feature_i | class)
```

```python
from nltk.corpus import movie_reviews
from nltk.classify import NaiveBayesClassifier, accuracy
import random

# Build (features, label) pairs
docs = [(list(movie_reviews.words(fileid)), category)
        for category in movie_reviews.categories()
        for fileid in movie_reviews.fileids(category)]
random.shuffle(docs)

all_words = nltk.FreqDist(w.lower() for w in movie_reviews.words())
word_features = list(all_words)[:2000]     # most frequent 2000 words as features

def document_features(document):
    doc_words = set(document)
    return {f"contains({w})": (w in doc_words) for w in word_features}

featuresets = [(document_features(d), c) for (d, c) in docs]
train_set, test_set = featuresets[100:], featuresets[:100]

classifier = NaiveBayesClassifier.train(train_set)
print(accuracy(classifier, test_set))
classifier.show_most_informative_features(10)
# e.g. contains(outstanding) = True -> pos : neg = 13.9 : 1.0
```

`show_most_informative_features` is one of NLTK's best teaching tools — it directly shows the **likelihood ratio** driving each feature's influence, making the model fully interpretable (a real edge over black-box neural classifiers for explainability-sensitive applications).

**Why Naive Bayes works well for text despite the independence assumption being clearly false** (words are obviously not independent — "New York" is a unit): text classification usually only needs the *relative ranking* of class probabilities to be right, not their exact calibrated values, and NB tends to get the ranking right even when the underlying probabilities are skewed by violated independence. It's also extremely fast to train (single pass, closed-form counting) and works well with small data — a strong, cheap baseline you should always try before reaching for something heavier.

### Maximum Entropy (MaxEnt) / Logistic Regression classifier

Unlike Naive Bayes (generative — models `P(features, class)`), MaxEnt is **discriminative** — directly models `P(class | features)` and doesn't assume feature independence, so it can use richly overlapping, correlated features without the naive-independence penalty. Usually more accurate than NB, at the cost of slower (iterative, e.g. gradient-based) training.

```python
from nltk.classify import MaxentClassifier
maxent_classifier = MaxentClassifier.train(train_set, max_iter=10)
print(accuracy(maxent_classifier, test_set))
```

### SVM via scikit-learn (common real-world combo: NLTK for features, sklearn for the classifier)

```python
from sklearn.svm import LinearSVC
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.pipeline import Pipeline

texts = [" ".join(movie_reviews.words(fid)) for fid in movie_reviews.fileids()]
labels = [movie_reviews.categories(fid)[0] for fid in movie_reviews.fileids()]

pipeline = Pipeline([
    ("tfidf", TfidfVectorizer(max_features=5000, stop_words="english")),
    ("svm", LinearSVC()),
])
pipeline.fit(texts[:1800], labels[:1800])
print(pipeline.score(texts[1800:], labels[1800:]))
```

SVMs with a linear kernel work exceptionally well on high-dimensional sparse text data (TF-IDF vectors) because they find the maximum-margin separating hyperplane robustly even when features vastly outnumber examples — a classic, still-competitive text classification baseline.

**Which classifier to reach for:**

| Situation | Use |
|---|---|
| Fast baseline, small data, need interpretability | Naive Bayes |
| Need best classical accuracy, features are correlated | MaxEnt / Logistic Regression |
| High-dimensional sparse TF-IDF vectors, clean margin | Linear SVM |
| Need SOTA accuracy, have GPU + enough data | Fine-tuned Transformer (BERT/RoBERTa) |

---

## 16. Sentiment Analysis (Lexicon-based & VADER)

### Lexicon-based approach

Maintain a dictionary mapping words to polarity scores; sum/average scores across the text. Simple, fast, no training data needed, but brittle to negation, sarcasm, and domain-specific vocabulary.

### VADER (Valence Aware Dictionary and sEntiment Reasoner)

Purpose-built for **social media text** — handles emoticons, slang, intensifiers ("very", "extremely"), punctuation emphasis ("!!!"), capitalization ("GREAT"), and crucially, **negation** ("not good") through hand-tuned heuristic rules layered on top of a sentiment lexicon. Ships directly with NLTK.

```python
from nltk.sentiment import SentimentIntensityAnalyzer

sia = SentimentIntensityAnalyzer()

print(sia.polarity_scores("This movie was great!"))
# {'neg': 0.0, 'neu': 0.406, 'pos': 0.594, 'compound': 0.6588}

print(sia.polarity_scores("This movie was NOT good at all."))
# strongly negative -- VADER correctly handles the negation + caps emphasis

print(sia.polarity_scores("This movie was okay, nothing special :("))
# mixed/slightly negative -- picks up the emoticon
```

- `pos`, `neu`, `neg` — proportion of text falling into each category (sum to 1.0).
- `compound` — single normalized score in `[-1, 1]`; the standard field to threshold for pos/neu/neg classification (common convention: `compound >= 0.05` → positive, `<= -0.05` → negative, else neutral).

**When to use VADER vs. training your own classifier:**

| Situation | Use |
|---|---|
| Social media, reviews, general English, no labeled data available | VADER (rule-based, zero training needed) |
| Domain-specific vocabulary (finance, medical) where general sentiment words mean something different | Train Naive Bayes/SVM on domain-labeled data |
| Need nuance: sarcasm, mixed sentiment, aspect-level ("food great, service terrible") | Fine-tuned Transformer sentiment model |
| Need fast, explainable, zero-shot baseline before investing in a trained model | VADER, always start here |

---

## 17. Similarity & Distance Metrics

Core primitives used across spell-checking, deduplication, search, clustering, and evaluation.

### Edit distance (Levenshtein distance)

Minimum number of single-character insertions, deletions, or substitutions to transform one string into another. Computed via dynamic programming.

```python
from nltk.metrics.distance import edit_distance

print(edit_distance("kitten", "sitting"))   # 3
print(edit_distance("colour", "color"))     # 1
```

**Used for:** spell correction (find the dictionary word with minimum edit distance to a typo), fuzzy matching, DNA sequence alignment.

### Jaccard distance/similarity

Set-based overlap — ignores frequency and order entirely, just membership.

```python
from nltk.metrics.distance import jaccard_distance
from nltk.util import ngrams

def jaccard_sim(a, b, n=2):
    a_ngrams = set(ngrams(a, n))
    b_ngrams = set(ngrams(b, n))
    return 1 - jaccard_distance(a_ngrams, b_ngrams)

print(jaccard_sim("night", "nacht"))   # character-bigram Jaccard similarity
```

`Jaccard(A, B) = |A ∩ B| / |A ∪ B|` — good for near-duplicate detection, plagiarism checks, comparing tag sets.

### Cosine similarity (dominant metric for vector-based text)

```python
from sklearn.metrics.pairwise import cosine_similarity
from sklearn.feature_extraction.text import TfidfVectorizer

docs = ["I love natural language processing", "I love machine learning"]
vecs = TfidfVectorizer().fit_transform(docs)
print(cosine_similarity(vecs[0], vecs[1]))
```

Measures the angle between two vectors, ignoring magnitude — critical for text because it makes the metric **insensitive to document length** (a long document repeating the same theme shouldn't be judged "more similar" just because its raw counts are bigger). This is why cosine similarity, not Euclidean distance, is the standard for comparing TF-IDF vectors and embeddings.

### Choosing a metric

| Situation | Metric |
|---|---|
| Comparing two strings for typos/near-matches | Edit (Levenshtein) distance |
| Comparing two sets (tags, n-gram sets) for overlap | Jaccard similarity |
| Comparing two dense/sparse vectors (TF-IDF, embeddings) | Cosine similarity |
| Comparing two probability distributions (e.g., topic distributions) | KL divergence / Jensen-Shannon divergence |
| Comparing WordNet senses | Path / Wu-Palmer / Leacock-Chodorow (Section 13) |

---

## 18. Sequence Labeling & the IOB Scheme

A unifying lens worth stating explicitly: **POS tagging, chunking, and NER are all the same underlying problem** — assign one label per token, where labels have strong sequential dependencies (a `B-NP` tag is far more likely to be followed by `I-NP` than by another `B-NP`). This is why the same algorithmic toolbox (HMM → MaxEnt/Perceptron → CRF → BiLSTM-CRF) reappears across all three tasks in NLTK.

```
Task           Input             Output labels
POS tagging    tokens            DT, NN, VB, JJ, ...
Chunking       (token, POS)      B-NP, I-NP, O, ...
NER            (token, POS)      B-PER, I-PER, B-ORG, I-ORG, O, ...
```

**The IOB (Inside-Outside-Begin) scheme** is what makes span-level labels (a "chunk" or "entity" can span multiple tokens) expressible as a flat per-token classification problem — necessary because most classifiers/sequence models predict one label per token, not variable-length spans directly.

```
B-X = first token of a span of type X
I-X = a token inside a span of type X (not the first)
O   = outside any span
```

**Why sequence labeling needs more than independent per-token classification:** if you classified each token's tag independently (ignore neighbors), you could predict an impossible sequence like `O, I-NP, O` (an "I" tag with no preceding "B"). HMMs, CRFs, and perceptron taggers all explicitly model transition likelihoods between adjacent labels to prevent this — the Viterbi algorithm's whole purpose is finding the single most probable *entire label sequence* jointly, not the best label per position independently.

---

## 19. Topic Modeling (brief, outside NLTK)

Not part of core NLTK, but essential NLP theory and commonly paired with NLTK preprocessing.

**LDA (Latent Dirichlet Allocation):** a generative probabilistic model assuming each document is a mixture of topics, and each topic is a distribution over words. Given only raw documents (unsupervised), LDA infers both the topic-word distributions and the document-topic distributions via Bayesian inference (typically Gibbs sampling or variational inference).

```python
import gensim
from gensim import corpora
from nltk.tokenize import word_tokenize
from nltk.corpus import stopwords

stop_words = set(stopwords.words('english'))
docs = ["cats and dogs are great pets", "stock markets rallied today", "my dog loves the vet"]
tokenized = [[w for w in word_tokenize(d.lower()) if w.isalpha() and w not in stop_words] for d in docs]

dictionary = corpora.Dictionary(tokenized)
corpus = [dictionary.doc2bow(d) for d in tokenized]

lda_model = gensim.models.LdaModel(corpus, num_topics=2, id2word=dictionary, passes=15)
for idx, topic in lda_model.print_topics(-1):
    print(f"Topic {idx}: {topic}")
```

**When to use:** exploratory analysis of large unlabeled document collections (what themes exist in this corpus?), document clustering/organization, feature reduction before classification. Not useful when you already have labels — use supervised classification instead (Section 15).

---

## 20. Classical NLP vs Modern Transformer NLP

Where NLTK-style classical NLP fits relative to the current state of the art — an interview staple ("why would you ever use this instead of an LLM?").

| Dimension | Classical (NLTK-style) | Modern (Transformer-based) |
|---|---|---|
| Representation | Sparse (BoW/TF-IDF) or static dense (Word2Vec) | Contextual dense embeddings via self-attention |
| Feature engineering | Manual (hand-crafted features, grammar rules) | Learned automatically end-to-end |
| Data needs | Works with small labeled datasets | Needs large data (or leverages pretraining + fine-tuning/few-shot) |
| Interpretability | High (can inspect probabilities, rules, feature weights) | Low (black-box, needs separate interpretability tooling) |
| Compute cost | Very cheap, runs on a laptop instantly | Expensive (GPU for training, often for inference too) |
| Handles long-range context | Poorly (Markov/window-limited) | Well (full-sequence self-attention) |
| Handles ambiguity/polysemy | Poorly (fixed rules/vectors) | Well (context-sensitive representations) |
| Typical accuracy ceiling | Good enough for constrained/simple tasks | SOTA across almost all benchmarks |
| When still the right choice | Tight latency/resource budgets, need full explainability, small/no training data, teaching/prototyping, strong keyword-matching needs (search/IR) | Everything else, especially open-domain, ambiguity-heavy, or accuracy-critical tasks |

**Good interview framing:** *"Classical NLP techniques aren't obsolete — they're the right tool when you need speed, interpretability, or you're resource/data-constrained, and they're the conceptual foundation (Markov assumption → attention, MLE counting → gradient descent, feature engineering → learned representations) that makes it possible to actually understand what a Transformer is doing instead of treating it as magic."*

---

## 21. End-to-End NLTK Pipeline Example

Putting it together — a full classical pipeline from raw text to a trained sentiment classifier, one representative pattern you could extend to any classification task.

```python
import nltk
import random
import re
from nltk.corpus import movie_reviews, stopwords
from nltk.tokenize import word_tokenize
from nltk.stem import WordNetLemmatizer
from nltk.classify import NaiveBayesClassifier, accuracy

# 1. Load labeled corpus
docs = [(movie_reviews.raw(fid), cat)
        for cat in movie_reviews.categories()
        for fid in movie_reviews.fileids(cat)]
random.seed(42)
random.shuffle(docs)

# 2. Preprocess: clean -> tokenize -> lowercase -> stopword removal -> lemmatize
stop_words = set(stopwords.words('english')) - {"not", "no", "never"}  # keep negators!
lemmatizer = WordNetLemmatizer()

def preprocess(text):
    text = re.sub(r"[^a-zA-Z\s]", " ", text)
    tokens = word_tokenize(text.lower())
    return [lemmatizer.lemmatize(t) for t in tokens if t not in stop_words and len(t) > 2]

processed = [(preprocess(text), label) for text, label in docs]

# 3. Feature extraction: most informative words as boolean presence features
all_words = nltk.FreqDist(w for tokens, _ in processed for w in tokens)
top_words = [w for w, _ in all_words.most_common(2000)]

def featurize(tokens):
    token_set = set(tokens)
    return {f"has({w})": (w in token_set) for w in top_words}

featuresets = [(featurize(tokens), label) for tokens, label in processed]

# 4. Train/test split
split = int(len(featuresets) * 0.9)
train_set, test_set = featuresets[:split], featuresets[split:]

# 5. Train and evaluate
classifier = NaiveBayesClassifier.train(train_set)
print("Accuracy:", accuracy(classifier, test_set))
classifier.show_most_informative_features(15)

# 6. Inference on new text
def predict(text):
    return classifier.classify(featurize(preprocess(text)))

print(predict("This film was an absolute masterpiece, brilliantly acted."))
print(predict("Terrible plot, wasted two hours of my life, not worth it."))
```

This is the pattern to reuse for **any** classical text classification task (spam detection, topic classification, intent detection) — swap the corpus and labels, the pipeline shape stays identical.

---

## 22. "Which Algorithm Do I Use?" Cheat Sheet

| Task | Go-to classical/NLTK tool | Reach for instead when... |
|---|---|---|
| Split text into sentences | `sent_tokenize` (Punkt) | Domain has unusual abbreviations → train custom Punkt |
| Split text into words | `word_tokenize` | Social text → `TweetTokenizer`; need subwords → BPE/WordPiece |
| Reduce words to base form, fast | `PorterStemmer` / `SnowballStemmer` | Need real words / semantic precision → `WordNetLemmatizer` |
| Vectorize documents, simple | `CountVectorizer` (BoW) | Need term importance weighting → `TfidfVectorizer` |
| Vectorize capturing word order | n-grams (`ngram_range=(1,2)` or higher) | Need true semantics → Word2Vec/GloVe/contextual embeddings |
| Represent word meaning densely | Word2Vec / GloVe | Need context-sensitive meaning (polysemy) → BERT/Transformer embeddings |
| Tag part of speech | `pos_tag` (Averaged Perceptron, default) | Need probabilistic sequence model → train an HMM tagger |
| Extract noun phrases | `RegexpParser` with grammar rules | Need learned/statistical chunking → train a `BigramChunker` on `conll2000` |
| Extract named entities | `ne_chunk` | Need higher accuracy → spaCy pretrained NER or fine-tuned Transformer |
| Full syntax tree | `ChartParser` + CFG, or `ViterbiParser` + PCFG | Need dependency relations → spaCy dependency parser |
| Word similarity/relatedness | WordNet (`path_similarity`, `wup_similarity`) | Need distributional/contextual similarity → embeddings + cosine similarity |
| Disambiguate word sense | `nltk.wsd.lesk` | Need higher accuracy → contextual embedding-based WSD |
| Predict next word / sequence probability | `nltk.lm` (`KneserNeyInterpolated`) | Need long-range coherence → neural LM/Transformer |
| Classify text (spam, topic, sentiment) | `NaiveBayesClassifier` (fast baseline) | Need better accuracy with correlated features → `MaxentClassifier` or `sklearn` SVM/LogReg; need SOTA → fine-tuned Transformer |
| Sentiment on social/short text, zero training | `SentimentIntensityAnalyzer` (VADER) | Domain-specific or nuanced sentiment → train a custom classifier |
| Fuzzy string matching / typo correction | `edit_distance` (Levenshtein) | Comparing sets/tags → Jaccard; comparing vectors → cosine similarity |
| Discover latent themes in unlabeled docs | LDA (via `gensim`, paired with NLTK preprocessing) | You have labels already → supervised classification instead |

---

## 23. Common Interview Questions & Answers

**Q: Why is tokenization harder than "just split on spaces"?**
A: Punctuation, contractions, abbreviations ("Dr.", "U.S."), hyphenated words, and multi-word expressions all break naive whitespace splitting. Sentence tokenization additionally has to distinguish sentence-final periods from abbreviation periods and decimal points — which is why NLTK's Punkt uses an unsupervised statistical model rather than a simple regex.

**Q: Stemming vs. lemmatization — when would each actually break your pipeline?**
A: Stemming can produce non-dictionary strings ("studi") that break any downstream step relying on real words (WordNet lookups, spell-checking, human-readable output). Lemmatization without the correct POS tag silently defaults to noun interpretation and can under-normalize verbs/adjectives — always pass POS tags through if lemmatizing in a real pipeline.

**Q: Why does TF-IDF outperform raw word counts for search/retrieval?**
A: Raw counts let ubiquitous, low-information words ("the", "is") dominate the vector, drowning out discriminative terms. IDF explicitly down-weights terms that appear in most documents, so the resulting vector is dominated by terms that actually distinguish this document from the rest of the corpus.

**Q: Explain the Markov assumption in n-gram language models and its main weakness.**
A: It assumes the next word depends only on the previous *n-1* words, not the full history — this makes probability estimation tractable via simple counting, but it means the model has no memory of anything outside its fixed window, so it can't capture long-range dependencies or discourse-level coherence (e.g., resolving a pronoun to a noun mentioned 20 words earlier). This is precisely the limitation self-attention in Transformers was designed to remove.

**Q: Why is Naive Bayes called "naive," and why does it still work well for text?**
A: It naively assumes all features (words) are conditionally independent given the class label, which is clearly false for language (word order and co-occurrence matter). It still works well because text classification usually only needs correct *relative* class ranking, not calibrated probabilities, and NB tends to preserve that ranking even when the independence assumption is violated — plus it's fast, needs little data, and is highly interpretable.

**Q: What's the difference between generative and discriminative classifiers, with NLP examples?**
A: Generative models (Naive Bayes, HMM) model the joint distribution `P(x, y)` — effectively learning how each class *generates* the data — and derive `P(y|x)` via Bayes' rule. Discriminative models (MaxEnt/Logistic Regression, CRF, SVM) directly model the decision boundary `P(y|x)` without modeling how the data was generated, which usually lets them use richer, overlapping, correlated features without a penalty, generally yielding higher accuracy at the cost of needing more data/compute to train.

**Q: Static embeddings (Word2Vec) vs. contextual embeddings (BERT) — what's the core architectural reason for the difference in capability?**
A: Word2Vec assigns exactly one fixed vector per word type, learned by predicting/being-predicted-by local context windows during training, then frozen at inference. BERT computes a word's vector at *inference time*, as a function of the entire input sequence via self-attention — so the same word gets a different vector depending on its sentence. This is what lets contextual embeddings resolve polysemy (bank/river vs. bank/finance) that static embeddings fundamentally cannot, since a static embedding must average/conflate all senses into one point in vector space.

**Q: How would you handle negation in a sentiment pipeline, and why is this a common bug?**
A: The common bug is applying generic stopword removal (which often strips "not", "no", "never") before sentiment scoring, silently flipping "not good" into "good." Fix: exclude negators from your stopword list, use a negation-aware lexicon method like VADER (which has explicit negation-handling rules), or use bigram features/contextual embeddings that naturally capture "not X" as a unit rather than treating "not" and "X" independently.
