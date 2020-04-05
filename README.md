# binexpr

This is a binary infix expression parser. It uses recursive descent, and then fixes up the resulting tree according to operator precedence. Currently it just runs as unit tests, which you can see near the bottom of `main.rs`. Just run `cargo test`.

The algorithm is the point of this repo, but maybe someday I'll get around to making the executable do something useful, like emit an ASCII parse tree.

## Background

This implementation was inspired by a [conversation](https://youtu.be/MnctEW1oL-E?t=2922) between Jonathan Blow and Casey Muratori.

Muratori asks Blow the same question that comes to mind each time I've tried to write a parser: what's an intuitive way to handle infix operators with precedence?

I found Blow's answer, which he refers to as "the second-best way to do it", to be surprisingly simple: do recursive descent when you run into an infix operator, and then take a pass down the left edge of the tree you just created, looking for operators with higher precedence. If you find the latter, then reorganize the tree a bit, and you're done.

Blow goes on to describe the "best way" he knows how to do it, which is the [precedence climbing](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing) technique that appears frequently in the literature. Muratori makes an offhanded remark about his preference for the build-a-tree-and-fix-it technique, and I tend to agree. It has an easy elegance to it. Despite this, it doesn't seem to appear to be all that popular, or at least, I've had a hard time finding resources that mention it. I suspect maybe that has something to do with the perceived slowness of recursion and tree operations.

## Algorithm

### Recursive descent

Without any modification, running recursive descent over a stream of infix arithmetic expressions will give you the equivalent of the following:

```
(a + (b + (c + (d + ...))))
```

As a parse tree, it might look something like this:

```
  +
 / \
a   +
   / \
  b   +
     / \
    c   +
       / \
      d  ...
```

This is easy to express as code. Given a stream of tokens, your parse node is composed of three parts:

1. The left side of the node is the first token.
2. The operator is the second token.
3. The right side of the node is whatever happens when you run the parse operation on everything after the first two tokens. This might just be a leaf token, if there's only one token left in the stream.

### Cleanup

Naive recursive descent doesn't give us the right semantics if we're trying to follow precedence conventions for arithmetic operations. To wit, using recursive descent on `a * b + c` would yield the following:

```
  *
 / \
a   +
   / \
  b   c
```

Please excuse my dear Aunt Sally, but that's wrong! We should be multiplying `a` and `b` first, and then adding that value to `c`. In other words, we want a tree that looks like this:


```
    +
   / \
  *   c
 / \
a   b

```

Fortnately, we can actually construct the second tree from the first tree with a reasonably simple algorithm.

Let's say we've extracted `a` and `*` from our stream. Rather than construct a new node right away, we set them aside, and parse the rest of the stream. That gives use the `b + c` node. We compare the precedence of the operator from this node (`+`) to the precedence of the operator we originally set aside (`*`). Since the latter has higher precedence, we need to do some reorganization.

`+` should be the root of our end result, and we should keep in place whatever it had on the right side (`c`). But its left side needs to be updated to a node composed of the following parts:

- Left side: `a`, the token we set aside.
- Operator: `*`, the operator we set aside.
- Right side: `b`, which moves here from its original position at the left side of `+`.

Let's go through that step by step.

1. Set aside `a` and `*`.

2. Parse the remaining tokens:

    ```
      +
     / \
    b   c
    ```

3. Pull the left side (`b`) off of the `+` node:

    ```
      +
     / \
    ?   c
    ```

4. ...and use it as the right side of a new node, using the stuff from step 1:

    ```
      *
     / \
    a   b
    ```

5. ...and put that where `b` used to be:

    ```
        +
       / \
      *   c
     / \
    a   b
    ```

It's actually a bit more complicated when you have more than two levels of precedence. Consider what happens if try to parse the expression `a ^ b * c + d`. By the conventional rules, we'd be looking for a tree that looks like this:

```
      +
     / \
    *   d
   / \
  ^   c
 / \
a   b
```

Naive recursive descent gives us the following:

```
  ^
 / \
a   *
   / \
  b   +
     / \
    c   d
```

This looks like a weirdly-rotated reflection of what we want. Let's try to run the algorithm we tried above:

1. Set aside `a` and `^`.

2. Parse the rest. Since this is recursive, we assume we'll get back a subtree with the right precedence:

    ```
        +
       / \
      *   d
     / \
    b   c
    ```

4. Pull the left side (`b * c`) off of the `+` node:

    ```
      +
     / \
    ?   d
    ```

5. ...and use it as the right side of a new node, using the stuff from step 1:

    ```
      ^
     / \
    a   *
       / \
      b   c
    ```

6. ...and put that where (`b * c`) used to be:

    ```
        +
       / \
      ^   d
     / \
    a   *
       / \
      b   c
    ```

Unfortunately, that still doesn't look quite right. `a` is being raised to the power of `b * c`, but what we really want is to multiply `a ^ b` with `c`.

Here's the missing piece: we need to repeat the cleanup recursively, between step 4 and 5. In other words, not only do we need to compare the precedence of `^` to the `+` operator, we also need to perform the comparison recursively against whatever's on the left side of the `+` node (in this case, `*`), and so on and so forth.

This is what I meant earlier when I said "take a pass down the left edge of the tree you just created"--you keep going down the left edge until you either hit the end, or you hit an operator with higher precedence.

### Handling parentheses

Okay, but what should this algorithm do about parentheses?

In the first place, we need to make sure our parser can pull out ranges of tokens enclosed by matching braces, and parse those as their own subtree. That's a somewhat less interesting problem, but complex enough that I won't dive into it here.

So, let's assume we have a parser that can do recursive descent within ranges bounded by parentheses. As long as we have some notation that tells us whether a node comes from a parenthesized expression, we can simply have the cleanup algorithm treat those nodes like leaves, rather than subtrees. In other words, we should stop doing any left-edge cleanup if we hit a parenthesized node.

Let's try this with `a * (b + c) + d`. Parentheses-aware recursive descent gives us:

```
  *
 / \
a   +
   / \
 (+)  d
 / \
b   c
```

Even though `*` has higher precedence than both `+` operators, our recursive cleanup step will stop when it hits the `(+)` node, leaving us with the following result:

```
    +
   / \
  *   d
 / \
a  (+)
   / \
  b   c
```

One last little comment: when I wrote this in code, I expressed parenthetical enclosures as their own type of single-child node. Visually, it looks something like this:

```
    +
   / \
  *   d
 / \
a  ( )
    |
    +
   / \
  b   c
```

This definitely makes for much uglier diagrams. But to my eyes, the _code_ ends up looking a bit cleaner. If you don't express parentheticals with their own special node type, you're left with adding a boolean flag on your non-leaf nodes to differentiate between the parenthesized ones and the non-parenthesized ones. It felt like a lot of clutter.
