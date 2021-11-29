# Euclidean Algorithm

Computer science is (almost by definition) a science about computers -- a device first conceptualized in the 1800's. Computers have become so revolutionary, that it is difficult to think of our lives today without them. That said, _algorithms_ are much older and have existed in the world for millennia. Incredibly, a few of the algorithms created before the Common Era (AD) are still in use today. One such algorithm was first described in Euclid's _Elements_ (~ 300 BC) and has come to be known as the _Euclidean Algorithm_.

The algorithm is a simple way to find the _greatest common divisor_ (GCD) of two numbers, which is useful for a number of different applications (like reducing fractions). The first method (envisioned by Euclid) uses simple subtraction:

```vim
{{#include code/viml/euclidean.vim:14:27}}
```

```c
{{#include code/c/euclidean_example.c:17:30}}
```

```csharp
{{#include code/csharp/EuclideanAlgorithm.cs:8:23}}
```

```clojure
{{#include code/clojure/euclidean_example.clj:2:8}}
```

```cpp
{{#include code/cpp/euclidean.cpp:18:31}}
```

```java
{{#include code/java/EuclideanAlgo.java:3:16}}
```

```kotlin
{{#include code/kotlin/Euclidean.kt:3:13}}
```

```javascript
{{#include code/javascript/euclidean_example.js:15:29}}
```

```lisp
{{#include code/clisp/euclidean.lisp:3:12}}
```

```python
{{#include code/python/euclidean_example.py:11:27}}
```

```haskell
{{#include code/haskell/euclidean_algorithm.hs:3:13}}
```

```rust
{{#include code/rust/euclidean_example.rs:3:15}}
```

```ocaml
{{#include code/ocaml/euclidean_example.ml:9:17}}
```

```go
{{#include code/go/euclidean.go:25:38}}
```

```swift
{{#include code/swift/euclidean_algorithm.swift:1:14}}
```

```matlab
{{#include code/matlab/euclidean.m:3:17}}
```

```lua
{{#include code/lua/euclidean.lua:1:14}}
```

```julia
{{#include code/julia/euclidean.jl:12:25}}
```

```nim
{{#include code/nim/euclid_algorithm.nim:13:24}}
```

```asm-x64
{{#include code/asm-x64/euclidean_example.s:35:56}}
```

```fortran
{{#include code/fortran/euclidean.f90:1:19}}
```

```php
{{#include code/php/euclidean.php:4:18}}
```

```factor
{{#include code/factor/euclid.factor:1:13}}
```

```whitespace
{{#include code/whitespace/euclidian_sub.ws}}
```

```scala
{{#include code/scala/euclidean.scala:3:8}}
```

```racket
{{#include code/racket/euclidean_algorithm.rkt:3:14}}
```

```ruby
{{#include code/ruby/euclidean.rb:8:19}}
```

```smalltalk
{{#include code/smalltalk/euclid.st:1:13}}
```

```emojicode
{{#include code/emojicode/euclidean_algorithm.emojic:2:17}}
```

```LOLCODE
{{#include code/lolcode/euclid.lol:25:40}}
```

```bash
{{#include code/bash/euclid.bash:24:38}}
```

```d
{{#include code/d/euclidean_algorithm.d:19:33}}
```

> ![](code/piet/subtract/euclidian_algorithm_subtract_large.png) ![](code/piet/subtract/euclidian_algorithm_subtract.png)

```scheme
{{#include code/scheme/euclidalg.ss:1:7}}
```

<!-- {% sample lang="scratch" %} -->

<p>
  <img  class="center" src="code/scratch/euclid_sub.svg" style="width:30%" />
</p>
# leave one line empty:

```powershell
{{#include code/powershell/euclidean_algorithm.ps1:1:14}}
```

```coconut
{{#include code/coconut/euclidean.coco:1:9}}
```

Here, we simply line the two numbers up every step and subtract the lower value from the higher one every timestep. Once the two values are equal, we call that value the greatest common divisor. A graph of `a` and `b` as they change every step would look something like this:

<p>
    <img  class="center" src="res/subtraction.png" style="width:70%" />
</p>

Modern implementations, though, often use the modulus operator (%) like so

```vim
{{#include code/viml/euclidean.vim:1:12}}
```

```c
{{#include code/c/euclidean_example.c:4:16}}
```

```csharp
{{#include code/csharp/EuclideanAlgorithm.cs:25:39}}
```

```clojure
{{#include code/clojure/euclidean_example.clj:9:13}}
```

```cpp
{{#include code/cpp/euclidean.cpp:5:15}}
```

```java
{{#include code/java/EuclideanAlgo.java:18:26}}
```

```kotlin
{{#include code/kotlin/Euclidean.kt:15:26}}
```

```javascript
{{#include code/javascript/euclidean_example.js:1:13}}
```

```lisp
{{#include code/clisp/euclidean.lisp:14:18}}
```

```python
{{#include code/python/euclidean_example.py:1:9}}
```

```haskell
{{#include code/haskell/euclidean_algorithm.hs:18:25}}
```

```rust
{{#include code/rust/euclidean_example.rs:17:27}}
```

```ocaml
{{#include code/ocaml/euclidean_example.ml:3:7}}
```

```go
{{#include code/go/euclidean.go:14:23}}
```

```swift
{{#include code/swift/euclidean_algorithm.swift:16:27}}
```

```matlab
{{#include code/matlab/euclidean.m:19:31}}
```

```lua
{{#include code/lua/euclidean.lua:16:25}}
```

```julia
{{#include code/julia/euclidean.jl:1:10}}
```

```nim
{{#include code/nim/euclid_algorithm.nim:1:11}}
```

```asm-x64
{{#include code/asm-x64/euclidean_example.s:10:33}}
```

```fortran
{{#include code/fortran/euclidean.f90:21:34}}
```

```php
{{#include code/php/euclidean.php:20:30}}
```

```factor
{{#include code/factor/euclid.factor:15:25}}
```

```whitespace
{{#include code/whitespace/euclidian_mod.ws}}
```

```scala
{{#include code/scala/euclidean.scala:10:14}}
```

```racket
{{#include code/racket/euclidean_algorithm.rkt:16:24}}
```

```ruby
{{#include code/ruby/euclidean.rb:1:6}}
```

```smalltalk
{{#include code/smalltalk/euclid.st:15:25}}
```

```emojicode
{{#include code/emojicode/euclidean_algorithm.emojic:19:31}}
```

```LOLCODE
{{#include code/lolcode/euclid.lol:9:23}}
```

```bash
{{#include code/bash/euclid.bash:10:22}}
```

```d
{{#include code/d/euclidean_algorithm.d:4:17}}
```

<!-- {% sample lang="piet" %} -->

> ![](code/piet/mod/euclidian_algorithm_mod_large.png) ![](code/piet/mod/euclidian_algorithm_mod.png)

```scheme
{{#include code/scheme/euclidalg.ss:9:12}}
```

<!-- {% sample lang="scratch" %} -->

<p>
  <img  class="center" src="code/scratch/euclid_mod.svg" style="width:30%" />
</p>
# leave one line empty:

```powershell
{{#include code/powershell/euclidean_algorithm.ps1:16:27}}
```

```coconut
{{#include code/coconut/euclidean.coco:12:15}}
```

Here, we set `b` to be the remainder of `a%b` and `a` to be whatever `b` was last timestep. Because of how the modulus operator works, this will provide the same information as the subtraction-based implementation, but when we show `a` and `b` as they change with time, we can see that it might take many fewer steps:

<p>
    <img  class="center" src="res/modulus.png" style="width:70%" />
</p>

The Euclidean Algorithm is truly fundamental to many other algorithms throughout the history of computer science and will definitely be used again later. At least to me, it's amazing how such an ancient algorithm can still have modern use and appeal. That said, there are still other algorithms out there that can find the greatest common divisor of two numbers that are arguably better in certain cases than the Euclidean algorithm, but the fact that we are discussing Euclid two millennia after his death shows how timeless and universal mathematics truly is. I think that's pretty cool.

## Video Explanation

Here's a video on the Euclidean algorithm:

<div style="text-align:center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/h86RzlyHfUE" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
</div>

## Example Code

```vim
{{#include code/viml/euclidean.vim}}
```

```cpp
{{#include code/c/euclidean_example.c}}
```

##### EuclideanAlgorithm.cs

```csharp
{{#include code/csharp/EuclideanAlgorithm.cs}}
```

##### Program.cs

```csharp
{{#include code/csharp/Program.cs}}
```

```clojure
{{#include code/clojure/euclidean_example.clj}}
```

```cpp
{{#include code/cpp/euclidean.cpp}}
```

```java
{{#include code/java/EuclideanAlgo.java}}
```

```kotlin
{{#include code/kotlin/Euclidean.kt}}
```

```javascript
{{#include code/javascript/euclidean_example.js}}
```

```lisp
{{#include code/clisp/euclidean.lisp}}
```

```python
{{#include code/python/euclidean_example.py}}
```

```haskell
{{#include code/haskell/euclidean_algorithm.hs}}
```

```rust
{{#include code/rust/euclidean_example.rs}}
```

```ocaml
{{#include code/ocaml/euclidean_example.ml}}
```

```go
{{#include code/go/euclidean.go}}
```

```swift
{{#include code/swift/euclidean_algorithm.swift}}
```

```matlab
{{#include code/matlab/euclidean.m}}
```

```lua
{{#include code/lua/euclidean.lua}}
```

```julia
{{#include code/julia/euclidean.jl}}
```

{% sample lang="nim" %}
[import, lang="nim" %](code/nim/euclid_algorithm.nim)

```asm-x64
{{#include code/asm-x64/euclidean_example.s}}
```

```fortran
{{#include code/fortran/euclidean.f90}}
```

```php
{{#include code/php/euclidean.php}}
```

```factor
{{#include code/factor/euclid.factor}}
```

```whitespace
{{#include code/whitespace/euclidian_sub_comments.ws}}
```

```whitespace
{{#include code/whitespace/euclidian_mod_comments.ws}}
```

```scala
{{#include code/scala/euclidean.scala}}
```

```racket
{{#include code/racket/euclidean_algorithm.rkt}}
```

```ruby
{{#include code/ruby/euclidean.rb}}
```

```smalltalk
{{#include code/smalltalk/euclid.st}}
```

```emojicode
{{#include code/emojicode/euclidean_algorithm.emojic}}
```

```LOLCODE
{{#include code/lolcode/euclid.lol}}
```

```bash
{{#include code/bash/euclid.bash}}
```

```d
{{#include code/d/euclidean_algorithm.d}}
```

A text version of the program is provided for both versions.

#### Subtraction

> ![](code/piet/subtract/euclidian_algorithm_subtract_large.png) ![](code/piet/subtract/euclidian_algorithm_subtract.png)

```piet
{{#include code/piet/euclidian_algorithm.piet:23:107}}
```
#### Modulo

> ![](code/piet/mod/euclidian_algorithm_mod_large.png) ![](code/piet/mod/euclidian_algorithm_mod.png)

```piet
{{#include code/piet/euclidian_algorithm.piet:126:146}}
```

```scheme
{{#include code/scheme/euclidalg.ss}}
```

<p>
The code snippets were taken from this [Scratch project](https://scratch.mit.edu/projects/278727055/)
</p>
<p>
  <img  class="center" src="code/scratch/main.svg" style="width:30%" />
</p>

```powershell
{{#include code/powershell/euclidean_algorithm.ps1}}
```

```coconut
{{#include code/coconut/euclidean.coco}}
```

## License

##### Code Examples

The code examples are licensed under the MIT license (found in [LICENSE.md](https://github.com/algorithm-archivists/algorithm-archive/blob/master/LICENSE.md)).

##### Text

The text of this chapter was written by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).

[<p><img  class="center" src="../cc/CC-BY-SA_icon.svg" /></p>](https://creativecommons.org/licenses/by-sa/4.0/)

##### Images/Graphics

- The image "[Euclidsub](res/subtraction.png)" was created by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).
- The image "[Euclidmod](res/modulus.png)" was created by [James Schloss](https://github.com/leios) and is licensed under the [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/legalcode).

##### Pull Requests

After initial licensing ([#560](https://github.com/algorithm-archivists/algorithm-archive/pull/560)), the following pull requests have modified the text or graphics of this chapter:

- none
