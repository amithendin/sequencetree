# Sequence Tree Collection
### a collection for read and write storage of sequential data

This is an implementation in rust of the establish collection named [trie](https://en.wikipedia.org/wiki/Trie#:~:text=In%20computer%20science%2C%20a%20trie,from%20a%20dictionary%20or%20set).

#### Intro
This collection that allows to store data using a key that is a sequence (such as string) and
fetch then corresponding value to that key at a time that is relative to the length of the key and the amount of similar
keys that already exist in the tree

This collection is meant to provide an alternative to hashtables which relay on a hash functions to convert the sequence to
a place in an array.

#### Hashtables
The advantages of hashtables is the time in take to fetch or store a key is often dependent on the time is takes to hash
said key, which in most cases is rather quick, but they have a drawback because a hash function have collision, meaning two different
values passed to the hash function can generate the same hash. To solve this many work arounds have been developed and implemented
but they all require some computationally expensive operations. In addition to that, when as hashtable stores more and more values
resizing of the array in the backend of the hashtable is required which can be an expensive operation.

#### Sequence Trees
Sequence trees break up a given key in to nodes with each node holding one element of the sequence (in case of strings a character),
and each node linked to it's next neighbor (like a linked list), Then the very last value holds the actual value assigned to the key.
Now to fetch the value of a key one simply need to follow each node in the tree in accordance the sequence of the key and easily reach
the value, no hash function needed!. One advantage of this is that the keys are naturally compressed simply due to the nature of this collection,
but the challange to making the tree fast is in the way one finds the correct next node since one node (character) can link to multiple
other characters (up to as much as in your character encoding). Because characters are backed by numbers I was able to simply store the
next nodes of each node in a Binary Search Tree, but one can also use hashtables (which can be expensive memory wise since they reserve space),
or even just a plain array (but that would be rather slow).

Sequence trees are slower than hashtables since they need to follow each element in the sequence by finding the correct
next node in the many next nodes each node can potentially hold. for this reason also similarity of the keys provides a
speed boost.

#### Current State
Tree is implemented generically, for this implementation though type used for the key must have
the following traits `PartialOrd + PartialEq + Copy + Default` since a binary tree is used to find the next
node. Deletion of keys is currently supported but not thoroughly tested so use at your own risk and please
inform me if you find any issues.

The plan for the future is mainly performance testing and documentation

#### How to use:
    
    let mut st: SequenceTree<char, u64> = SequenceTree::new(); // initialize tree
    
    let key: Vec<char> = String::from("hello").chars().collect(); // create key

    st.set(key.to_owned(), 55); // set key

    println!("{:?}", st.get(key) ); //get value of key
    
#### Todo:
- expand README to include 
    - picture explantions, 
    - runtime and space complexity
    - preformance chart to compare with current solutions
