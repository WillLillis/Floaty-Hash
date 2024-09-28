# Floaty Hash

Ever wanted to use a hash set with floats but ALSO wanted to treat values within a given error tolerance as the *same* number in this case? (Why would you ever try to do this?)

*DISCLAIMER*: Please don't use this in any actual project. It's just a proof of concept of a Friday afternoon "What if?"

```
Let's try a quick demo of this cursed idea!
Inserting 42 and 42.00002...
Number of items in the hash set: 2
Ok well that makes sense, let's clear out those values
How about this?
Inserting 42 and 42.000004...
(Note that |42 - 42.000004| = 0.0000038146973 < 0.00001)
Number of items in the hash set: 1 (I'm so sorry)
```
