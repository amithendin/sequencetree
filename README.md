# Sequence Tree Collection
### a collection for fast read and right storage of sequential data
###### (as an alternative to hashtable)

#### Intro
Sequence trees are a new type of collection that allows to store data using a key that is a sequence (such as string) and
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
In come Sequence Trees! sequence trees break up a given key in to nodes with each node holding one element of the sequence (in case of strings a character),
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
I have currently only implemented SequenceTree for strings but in theory they could be implemented for any kind of sequential data
but for said data to also be used with binary search trees it must implement the compare function (less than, greater than, equal to).
In the future when I have time to get into rust generics I will implement a generic version. For now no other version in other
languages is planned.

#### How to use:
    
    let mut st = SequenceTree::new(); // initialize a new tree
    
    st.set("hello", "world"); // set a key to a value
    
    println!("value = {:?}", set.get("hello")); // fetch the value
    
#### Todo:
    - make generic implementation
    - expand README to include 
        - picture explantions, 
        - efficiency calculation 
        - preformance chart to compare with current solutions
    - comments in code