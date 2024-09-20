## What is that
It's basic hashmap implementation. I didn't work much on hash functions, so there can be some collisions. The workaround i chose is a linked list. 
Also i wanted to make it using values as an array, but in the end i used a vector with an imitation of auto expansion (because you don't need to do it in the vector).
I've tried to provide type K for keys of hashmap, but it turned out kinda hard to do (also it's not so compatible with my hashers), so i decided to stay with type only for values. 
So, my hashmap can take only &str type for keys.
## Note
It is a library package, so you can't start it, but you can run tests via ```cargo test```. 
## My opinion
It was interesting task to do, but implementing a really good hashmap is a hard task. Hovewer, i'm happy that i tried to make a small working hashmap. 
