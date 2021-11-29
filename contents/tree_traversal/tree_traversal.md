# Tree Traversal

Trees are naturally recursive data structures, and because of this, we cannot access their elements like we might access the elements of a vector or array. Instead, we need to use more interesting methods to work through each element. This is often called *Tree Traversal*, and there are many different ways to do this. For now, we will restrict the discussion to two common and related methods of tree traversal: *Depth-First* and *Breadth-First Search*. Note that trees vary greatly in shape and size depending on how they are used; however, they are composed primarily of nodes that house other, children nodes, like so:


```julia
{{#include code/julia/Tree.jl:3:7}}
``````julia
{{#include code/julia/Tree.jl:3:7}}
``````julia
{{#include code/julia/Tree.jl:3:7}}
```c```julia
{{#include code/julia/Tree.jl:3:7}}
``````julia
{{#include code/julia/Tree.jl:3:7}}
``````julia
{{#include code/julia/Tree.jl:3:7}}
```
[import:7-11, lang:"c"](code/c/tree_traversal.c)
{% sample lang="java" %}
[import:112-128, lang:"java"](code/java/Tree.java)
{% sample lang="js" %}
[import:1-10, lang:"javascript"](code/javascript/tree.js)
As a note, a `node` struct is not necessary in javascript, so this is an example of how a tree might be constructed.
```python
{{#include code/python/tree_traversal.py:1:4}}
```
{% sample lang="scratch" %}
<p>
  <img  class="center" src="code/scratch/struct.svg" style="width:25%" />
</p>
```rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
```v```rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
```.```rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
``````rust
{{#include code/rust/tree.rs:4:7}}
```t```rust
{{#include code/rust/tree.rs:4:7}}
```
[import:5-8, lang:"go"](code/go/treetraversal.go)
{% sample lang="asm-x64" %}
[import:24-27, lang:"asm-x64"](code/asm-x64/tree_traversal.s)
{% sample lang="emojic" %}
[import:1-3, lang:"emojicode"](code/emojicode/tree_traversal.emojic)
{% sample lang="lisp" %}
[import:3-3, lang:"lisp"](code/clisp/tree-traversal.lisp)
{% sample lang="m" %}
[import:6-6, lang:"matlab"](code/matlab/tree.m)
{% sample lang="coco" %}
[import:3-3, lang:"coconut"](code/coconut/tree_traversal.coco)


Because of this, the most straightforward way to traverse the tree might be recursive. This naturally leads us to the Depth-First Search (DFS) method:


```julia
{{#include code/julia/Tree.jl:9:16}}
``````julia
{{#include code/julia/Tree.jl:9:16}}
``````julia
{{#include code/julia/Tree.jl:9:16}}
```p```julia
{{#include code/julia/Tree.jl:9:16}}
``````julia
{{#include code/julia/Tree.jl:9:16}}
```s```julia
{{#include code/julia/Tree.jl:9:16}}
``````julia
{{#include code/julia/Tree.jl:9:16}}
```
{% sample lang="java" %}
[import:20-26, lang:"java"](code/java/Tree.java)
{% sample lang="js" %}
[import:12-19, lang:"javascript"](code/javascript/tree.js)
{% sample lang="py" %}
[import:17-22, lang:"python"](code/python/tree_traversal.py)
{% sample lang="scratch" %}
<p>
  <img  class="center" src="code/scratch/dfs-pre.svg" style="width:40%" />
</p>
{% sample lang="rs" %}
[import:9-15 lang:"rust"](code/rust/tree.rs)
```haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
```r```haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
``````haskell
{{#include code/haskell/TreeTraversal.hs:7:8}}
```
{% sample lang="asm-x64" %}
[import:290-314, lang:"asm-x64"](code/asm-x64/tree_traversal.s)
{% sample lang="emojic" %}
[import:27-34, lang:"emojicode"](code/emojicode/tree_traversal.emojic)
{% sample lang="lisp" %}
[import:5-10, lang:"lisp"](code/clisp/tree-traversal.lisp)
{% sample lang="m" %}
[import:31-45, lang:"matlab"](code/matlab/tree.m)
{% sample lang="coco" %}
[import:5-9, lang:"coconut"](code/coconut/tree_traversal.coco)


At least to me, this makes a lot of sense. We fight recursion with recursion! First, we first output the node we are on and then we call `DFS_recursive(...)` on each of its children nodes. This method of tree traversal does what its name implies: it goes to the depths of the tree first before going through the rest of the branches. In this case, the ordering looks like:

<p>
    <img  class="center" src="res/DFS_pre.png" style="width:70%" />
</p>

Note that the in the code above, we are missing a crucial step: *checking to see if the node we are using actually exists!* Because we are using a vector to store all the nodes, we will be careful not to run into a case where we call `DFS_recursive(...)` on a node that has yet to be initialized; however, depending on the language we are using, we might need to be careful of this to avoid recursion errors!

Now, in this case the first element searched through is still the root of the tree. This type of tree traversal is known as *pre-order* DFS. We perform an action (output the ID) *before* searching through the children. If we shift the function around and place the data output at the end of the function, we can modify the order in which we search through the tree to be *post-order* and look something like this:



```julia
{{#include code/julia/Tree.jl:18:26}}
``````julia
{{#include code/julia/Tree.jl:18:26}}
``````julia
{{#include code/julia/Tree.jl:18:26}}
```p```julia
{{#include code/julia/Tree.jl:18:26}}
``````julia
{{#include code/julia/Tree.jl:18:26}}
```s```julia
{{#include code/julia/Tree.jl:18:26}}
``````julia
{{#include code/julia/Tree.jl:18:26}}
```
{% sample lang="java" %}
[import:33-40, lang:"java"](code/java/Tree.java)
{% sample lang="js" %}
[import:21-28, lang:"javascript"](code/javascript/tree.js)
{% sample lang="py" %}
[import:25-30, lang:"python"](code/python/tree_traversal.py)
{% sample lang="scratch" %}
<p>
  <img  class="center" src="code/scratch/dfs-post.svg" style="width:40%" />
</p>
```rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
```v```rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
```p```rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
``````rust
{{#include code/rust/tree.rs:17:24}}
```ree_traversal.st)
{% sample lang="go" %}
[import:17-22, lang:"go"](code/go/treetraversal.go)
{% sample lang="asm-x64" %}
[import:316-344, lang:"asm-x64"](code/asm-x64/tree_traversal.s)
{% sample lang="emojic" %}
[import:36-43, lang:"emojicode"](code/emojicode/tree_traversal.emojic)
{% sample lang="lisp" %}
[import:12-17, lang:"lisp"](code/clisp/tree-traversal.lisp)
{% sample lang="m" %}
[import:47-62, lang:"matlab"](code/matlab/tree.m)
{% sample lang="coco" %}
[import:11-15, lang:="coconut"](code/coconut/tree_traversal.coco)


<p>
    <img  class="center" src="res/DFS_post.png" style="width:70%" />
</p>

In this case, the first node visited is at the bottom of the tree and moves up the tree branch by branch. In addition to these two types, binary trees have an *in-order* traversal scheme that looks something like this:


```julia
{{#include code/julia/Tree.jl:28:43}}
```
{% sample lang="cpp" %}
[import:34-52 lang:"cpp"](code/cpp/tree_example.cpp)
```csharp
{{#include code/csharp/Tree.cs:59:83}}
``````csharp
{{#include code/csharp/Tree.cs:59:83}}
``````csharp
{{#include code/csharp/Tree.cs:59:83}}
```)```csharp
{{#include code/csharp/Tree.cs:59:83}}
``````csharp
{{#include code/csharp/Tree.cs:59:83}}
```
{% sample lang="js" %}
[import:30-51, lang:"javascript"](code/javascript/tree.js)
{% sample lang="py" %}
[import:34-45, lang:"python"](code/python/tree_traversal.py)
{% sample lang="scratch" %}
<p>
  <img  class="center" src="code/scratch/dfs-in.svg" style="width:40%" />
</p>
```rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
```v```rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
```p```rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
``````rust
{{#include code/rust/tree.rs:25:40}}
```r```rust
{{#include code/rust/tree.rs:25:40}}
```
[import:24-38, lang:"go"](code/go/treetraversal.go)
{% sample lang="asm-x64" %}
[import:346-396, lang:"asm-x64"](code/asm-x64/tree_traversal.s)
{% sample lang="emojic" %}
[import:45-62, lang:"emojicode"](code/emojicode/tree_traversal.emojic)
{% sample lang="lisp" %}
[import:19-32, lang:"lisp"](code/clisp/tree-traversal.lisp)
{% sample lang="m" %}
[import:64-82, lang:"matlab"](code/matlab/tree.m)
{% sample lang="coco" %}
[import:17-30, lang:"coconut"](code/coconut/tree_traversal.coco)


<p>
    <img  class="center" src="res/DFS_in.png" width="500" />
</p>

The order here seems to be some mix of the other 2 methods and works through the binary tree from left to right.

Now, at this point, it might seem that the only way to search through a recursive data structure is with recursion, but this is not necessarily the case! Rather surprisingly, we can perform a DFS non-recursively by using a stack, which are data structures that hold multiple elements, but only allow you to interact with the very last element you put in. The idea here is simple:

1. Put the root node in the stack
2. Take it out and put in its children
3. Pop the top of the stack and put its children in
4. Repeat 3 until the stack is empty

In code, it looks like this:


```julia
{{#include code/julia/Tree.jl:45:56}}
``````julia
{{#include code/julia/Tree.jl:45:56}}
``````julia
{{#include code/julia/Tree.jl:45:56}}
```p```julia
{{#include code/julia/Tree.jl:45:56}}
``````julia
{{#include code/julia/Tree.jl:45:56}}
```s```julia
{{#include code/julia/Tree.jl:45:56}}
``````julia
{{#include code/julia/Tree.jl:45:56}}
```
{% sample lang="java" %}
[import:67-81, lang:"java"](code/java/Tree.java)
{% sample lang="js" %}
[import:53-60, lang:"javascript"](code/javascript/tree.js)
{% sample lang="py" %}
[import:48-59, lang:"python"](code/python/tree_traversal.py)
{% sample lang="scratch" %}
<p>
  <img  class="center" src="code/scratch/dfs-stack.svg" style="width:70%" />
</p>
```rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
```v```rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
```p```rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
``````rust
{{#include code/rust/tree.rs:41:48}}
```e```rust
{{#include code/rust/tree.rs:41:48}}
```
[import:40-49, lang:"go"](code/go/treetraversal.go)
{% sample lang="asm-x64" %}
[import:398-445, lang:"asm-x64"](code/asm-x64/tree_traversal.s)
{% sample lang="emojic" %}
[import:64-79, lang:"emojicode"](code/emojicode/tree_traversal.emojic)
{% sample lang="lisp" %}
[import:34-43, lang:"lisp"](code/clisp/tree-traversal.lisp)
{% sample lang="m" %}
[import:84-106, lang:"matlab"](code/matlab/tree.m)
{% sample lang="coco" %}
[import:32-39, lang:"coconut"](code/coconut/tree_traversal.coco)


All this said, there are a few details about DFS that might not be ideal, depending on the situation. For example, if we use DFS on an incredibly long tree, we will spend a lot of time going further and further down a single branch without searching the rest of the data structure. In addition, it is not the natural way humans would order a tree if asked to number all the nodes from top to bottom. I would argue a more natural traversal order would look something like this:

<p>
    <img  class="center" src="res/BFS_simple.png" style="width:70%" />
</p>

And this is exactly what Breadth-First Search (BFS) does! On top of that, it can be implemented in the same way as the `DFS_stack(...)` function above, simply by swapping the `stack` for a `queue`, which is similar to a stack, except that it only allows you to interact with the very first element instead of the last. In code, this looks something like:


```julia
{{#include code/julia/Tree.jl:58:69}}
``````julia
{{#include code/julia/Tree.jl:58:69}}
``````julia
{{#include code/julia/Tree.jl:58:69}}
```)```julia
{{#include code/julia/Tree.jl:58:69}}
``````julia
{{#include code/julia/Tree.jl:58:69}}
```.```julia
{{#include code/julia/Tree.jl:58:69}}
``````julia
{{#include code/julia/Tree.jl:58:69}}
```
{% sample lang="java" %}
[import:83-97, lang:"java"](code/java/Tree.java)
{% sample lang="js" %}
[import:62-69, lang:"javascript"](code/javascript/tree.js)
{% sample lang="py" %}
[import:62-72, lang:"python"](code/python/tree_traversal.py)
{% sample lang="scratch" %}
<p>
  <img  class="center" src="code/scratch/bfs.svg" style="width:70%" />
</p>
```rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
```v```rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
```.```rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
``````rust
{{#include code/rust/tree.rs:50:58}}
```e```rust
{{#include code/rust/tree.rs:50:58}}
```
[import:51-60, lang:"go"](code/go/treetraversal.go)
{% sample lang="asm-x64" %}
[import:447-498, lang:"asm-x64"](code/asm-x64/tree_traversal.s)
{% sample lang="emojic" %}
[import:81-96, lang:"emojicode"](code/emojicode/tree_traversal.emojic)
{% sample lang="lisp" %}
[import:45-56, lang:"lisp"](code/clisp/tree-traversal.lisp)
{% sample lang="m" %}
[import:108-129, lang:"matlab"](code/matlab/tree.m)
{% sample lang="coco" %}
[import:41-48, lang:"coconut"](code/coconut/tree_traversal.coco)


## Video Explanation

Here is a video describing tree traversal:

<div style="text-align:center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/cZPXfl_tUkA" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
</div>

## Example Code

```julia
{{#include code/julia/Tree.jl}}
``````julia
{{#include code/julia/Tree.jl}}
```
[import, lang:"cpp"](code/cpp/tree_example.cpp)
{% sample lang="cs" %}
##### Tree.cs
[import, lang:"csharp"](code/csharp/Tree.cs)
##### Program.cs
[import, lang:"csharp"](code/csharp/Program.cs)
{% sample lang="c" %}
##### utility.h
[import, lang:"c"](code/c/utility.h)
##### tree_traversal.c
[import, lang:"c"](code/c/tree_traversal.c)
{% sample lang="java" %}
##### Tree.java
[import, lang:"java"](code/java/Tree.java)
```javascript
{{#include code/javascript/tree.js}}
``````javascript
{{#include code/javascript/tree.js}}
```
[import, lang:"python"](code/python/tree_traversal.py)
{% sample lang="scratch" %}

The code snippets were taken from this [Scratch project](https://scratch.mit.edu/projects/174017753/)

<p>
  <img  class="center" src="code/scratch/all.svg" style="width:100%" />
</p>
```rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
```v```rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
```p```rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
``````rust
{{#include code/rust/tree.rs}}
```r```rust
{{#include code/rust/tree.rs}}
```
[import, lang:"go"](code/go/treetraversal.go)
{% sample lang="asm-x64" %}
[import, lang:"asm-x64"](code/asm-x64/tree_traversal.s)
{% sample lang="emojic" %}
[import, lang:"emojicode"](code/emojicode/tree_traversal.emojic)
{% sample lang="lisp" %}
[import, lang:"lisp"](code/clisp/tree-traversal.lisp)
{% sample lang="m" %}
[import, lang:"matlab"](code/matlab/tree.m)
{% sample lang="coco" %}
[import, lang:"coconut"](code/coconut/tree_traversal.coco)





## License

##### Code Examples

The code examples are licensed under the MIT license (found in [LICENSE.md](https://github.com/algorithm-archivists/algorithm-archive/blob/master/LICENSE.md)).

##### Text

The text of this chapter was written by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).

[<p><img  class="center" src="../cc/CC-BY-SA_icon.svg" /></p>](https://creativecommons.org/licenses/by-sa/4.0/)

##### Images/Graphics
- The image "[DFSpreorder](res/DFS_pre.png)" was created by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).
- The image "[DFSpostorder](res/DFS_post.png)" was created by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).
- The image "[DFSinorder](res/DFS_in.png)" was created by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).
- The image "[BFSsimple](res/BFS_simple.png)" was created by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).


##### Pull Requests

After initial licensing ([#560](https://github.com/algorithm-archivists/algorithm-archive/pull/560)), the following pull requests have modified the text or graphics of this chapter:
- none
