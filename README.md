# roll

Roll some dice.

Given arguments, `roll` will parse its arguments and roll any substring like `\d*[Dd]\d+`.  Without arguments, `roll` will read from stdin.

## Example usage:

```
$ echo '3d6 radiant damage and 2d12 fire damage' | roll
11 radiant damage and 17 fire damage
```

```
$ roll 3*d10 + 2d4 + 10 | bc
39
```
