# Lucas's theorem

[Wikipedia ref](https://en.wikipedia.org/wiki/Lucas%27s_theorem)

$$
    {m \choose n} = \prod\limits_{i=0}^{k}{m_i \choose n_i} (\text{mod $p$}) \\
    m = \sum\limits_{i=0}^{k} m_i p^i, n = \sum\limits_{i=0}^{k} n_i p^i \\
    m < n \Rightarrow {m \choose n} = 0
$$

## When $p=2$

if $p=2$, ${m \choose n} > 0$ infers the binary representation of $n$ is the 'subset' of $m$, and let $S=2^x >= \max(n, m)$, $S-n$ contains $m$ and $S-m$ contains $n$.