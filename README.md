# Rust LiveView

The goal of this project is too experiment a [Phoenix LiveView](https://github.com/phoenixframework/phoenix_live_view) like experience for rust (and learn rust at the same time).

This is a first start, totally not optimized and not usable at all

## Hacking on rust LiveView

1. `yarn install && yarn build`
2. `cargo run`

So far you will only a see a counter that you can increment or decrement. The entire "counter" component is overrided each times.


## TODOs

- [ ] study the [diff algorithm](https://github.com/phoenixframework/phoenix_live_view/blob/master/lib/phoenix_live_view/diff.ex) used by Phoenix LiveView
- [ ] find a way to determine statics and dynamics from a template (may require to hack on the template enine)
- [ ] have a real view attached to the socket (so we ccan handle more than one component per page)
- [ ] handle more LiveView event
- [ ] keep going to build a usable library
