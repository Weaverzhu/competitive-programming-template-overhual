## Misc

### usize & u32 & u64

* usize overflow 在 oj 上用 release 是测不出来的
* `usize - x + y` 先下溢出，usize 应该先做加法