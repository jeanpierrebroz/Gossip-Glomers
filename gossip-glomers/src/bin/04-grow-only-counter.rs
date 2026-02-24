/*
Sequential consistency is a strong safety property for concurrent systems. 
Informally, sequential consistency implies that operations appear to take place in some total order, 
and that that order is consistent with the order of operations on each individual process.

Sequential consistency cannot be totally or sticky available; in the event of a network partition, 
some or all nodes will be unable to make progress.

A process in a sequentially consistent system may be far ahead of, or behind, other processes. 
For instance, they may read arbitrarily stale state. However, once a process A has observed some operation 
from process B, it can never observe a state prior to B. 
This, combined with the total ordering property, makes sequential consistency a surprisingly strong model 
for programmers.


*/


//create a key per node id?
//

pub fn main() {
    
}