# Bitmask DP

> This is a note from [reference on a codeforces blog](https://codeforces.com/blog/entry/72488)

### Sum over subset / Zeta transform
$$
    z(f(S)) = \sum\limits_{S' \subseteq S} f(S')
$$

If we know all $f(s)$, we can calculate any set of $s$, which is $S$ in $O(2^n)$ of both space complexity and time complexity.

See [rust code](../../rust/src/dp/sum_over_subset.rs)

### Odd-Negation transform

$$
    \sigma(f(S)) = (-1)^{|S|} f(S)
$$

Calcutate the parity of 1-bits after calculating zeta transform.

### Mobius transform

$$
    \mu(f(S)) = \sum\limits_{S'\subseteq S}(-1)^{|S \setminus S'|} f(S') = \sigma z \sigma (f(S))
$$

Proof is in the reference, here's some cpp code implementation from it.

```cpp
// Apply odd-negation transform
for(int mask = 0; mask < (1 << N); mask++) {
    if((__builtin_popcount(mask) % 2) == 1) {
        f[mask] *= -1;
    }
}

// Apply zeta transform
for(int i = 0; i < N; i++) {
    for(int mask = 0; mask < (1 << N); mask++) {
        if((mask & (1 << i)) != 0) {
            f[mask] += f[mask ^ (1 << i)];
        }
    }
}

// Apply odd-negation transform
for(int mask = 0; mask < (1 << N); mask++) {
    if((__builtin_popcount(mask) % 2) == 1) {
        f[mask] *= -1;
    }
}
for(int mask = 0; mask < (1 << N); mask++)  mu[mask] = f[mask];
```

$$
    z^{-1}f(S) = \mu(f(S))
$$

```cpp
for(int i = 0; i < N; i++) {
    for(int mask = 0; mask < (1 << N); mask++) {
        if((mask & (1 << i)) != 0) {
            f[mask] -= f[mask ^ (1 << i)]
        }
    }
}
for(int mask = 0; mask < (1 << N); mask++)  zinv[mask] = mu[mask] = f[mask]
```

### Subset convolution

I'll try to leave some intuitive ideas.

$$
    f \circ g (S) = \sum\limits_{S' \subseteq S} f(S)g(S \setminus S') \\
    = z^{-1}\big[ \sum\limits_{i=0}^{|S|} z(\hat f(i, S)) z(\hat g(|S| - i, S))\big]
$$

This can be achieved in time complexity $O(2^N * N^2)$, which should be close to $O(2^N)$.


First, some helper function is defined.

$$
    \hat f(i, s) = \begin{cases}
        f(s) &\text{if $|S| = i$} \\
        0 &\text{else}
    \end{cases} \\

    p(k, s) = \sum\limits_{i=0}^{k}\sum\limits_{a\subseteq s, |a|=i}\sum\limits_{b\subseteq s, |b|=|s|-i, a\cup b=s}f(a)g(b) \\
    p(|s|,s) = f \circ g(s) \ \ \ \ \   \text{$a\cap b$ must be $\emptyset$, b must be $s \setminus a$}
$$

The idea of the proof is to unroll the $z$

$$
    \sum\limits_{i=0}^{|S|} z( \hat f(i, S)) z( \hat g(|S|-i, S)) \\
    = \sum\limits_{i=0}^{|S|} \sum\limits_{a\subseteq S} \hat f(i, a) \sum\limits_{b\subseteq S} \hat g(|S|-i, b) \\
    =  \sum\limits_{i=0}^{|S|} \sum\limits_{a \subseteq S, |a| = i}f(a)  \sum\limits_{b \subseteq S, |b| = |S|-i}g(b)
$$

**Note that for every a, b, there is a $a \cup b$, we then enumerate this $a \cup b$ as $s'$**

$$
 = \sum\limits_{i=0}^{|S|} \sum\limits_{s' \subseteq S} \sum\limits_{a\subseteq s', b\subseteq s', a\cup b=s', |a|=i, |b|=|S|-i} f(a)g(b)
 = \sum\limits_{s' \subseteq S} p(|S|, s') = z(f \circ g(s))
$$

```cpp
// Make fhat[][] = {0} and ghat[][] = {0}
for(int mask = 0; mask < (1 << N); mask++) {
    fhat[__builtin_popcount(mask)][mask] = f[mask];
    ghat[__builtin_popcount(mask)][mask] = g[mask];
}

// Apply zeta transform on fhat[][] and ghat[][]
for(int i = 0; i < N; i++) {
    for(int j = 0; j < N; j++) {
        for(int mask = 0; mask < (1 << N); mask++) {
            if((mask & (1 << j)) != 0) {
                fhat[i][mask] += fhat[i][mask ^ (1 << j)];
                ghat[i][mask] += ghat[i][mask ^ (1 << j)];
            }
        }
    }
}

// Do the convolution and store into h[][] = {0}
for(int mask = 0; mask < (1 << N); mask++) {
    for(int i = 0; i < N; i++) {
        for(int j = 0; j <= i; j++) {
            h[i][mask] += fhat[j][mask] * ghat[i - j][mask];
        } 
    }
}

// Apply inverse SOS dp on h[][]
for(int i = 0; i < N; i++) {
    for(int j = 0; j < N; j++) {
        for(int mask = 0; mask < (1 << N); mask++) {
            if((mask & (1 << j)) != 0) {
                h[i][mask] -= h[i][mask ^ (1 << j)];
            }
        }
    }
}
for(int mask = 0; mask < (1 << N); mask++)  fog[mask] = h[__builtin_popcount(mask)][mask];
```