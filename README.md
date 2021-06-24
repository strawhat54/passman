## Plan change:

1. we need to move match statements to main and logic bas dusre file m rhega.
2. Vec (u8) jo hai wo store karne m problem ho rha hai so we need to use serde like:
```
{
    master = salt,
    store = {
        name: ...
        salt: ...
        encrypted: ...
        desc = ...
    }
}
``` 

3. Ring acha hai but thora complicated hai...merko ek password manager mil hai usme wo use kiya hai to usse help le skte hai: https://github.com/defund/pw/tree/master/src

4. rpassword ek module hai jo password type krte wqt hiddend rkhega input ko

mai o'reilly ka ek rust pe book padh rha hu kaafi indepth hai to isiliye kyuch kiya nhi do din se.
Sath me start karenge :)