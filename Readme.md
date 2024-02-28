[slint#4699](https://github.com/slint-ui/slint/issues/4699) issue example project

### run project

```shell
cargo run
```

### issue description

1. i use TestGlobal to change data in async
2. .slint file rendering different components based on TestGlobal
3. but just change TestGlobal data and not change components, close PopupWindow components will be panic!

### steps

1. clicked button to show confirm component
2. clicked confirm/cancel button to close confirm component
3. if the TestGlobal data change now, the program will be panic!

### guess

this issue may be is not a bug, just my slint file logic bug