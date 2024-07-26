# anvil

a website for calculating the cheapest ways to combine items in anvils.

## building

you will need [trunk](https://trunkrs.dev/). the easiest way to get trunk is to do `cargo install --locked trunk`. otherwise, check their website.

### developer mode

do `trunk serve`. the code will automatically compile and the page will refresh after saving.

### building for release

do `trunk build --release`. if you're not putting this in the route of a server, you can also add `--public-url <folder>` to set the folder.
