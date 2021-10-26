# Pling

> Send notifications via Slack, Telegram, E-Mail, ...

The name of this Rust crate is inspired by the notification arrival sound.

This README is primarily meant for end users and which services are available when this crate is added in other software.
For ways to integrate this crate into your rust application programmatically check [its docs.rs entry](https://docs.rs/pling).

## Services

### Currently implemented
Keep in mind they depend on the feature flags this crate is compiled with.
Feature flags are also described in their documentation entry.

- [Command (on the same host)](docs/command.md)
- [Email](docs/email.md) (untested)
- [Matrix](docs/matrix.md) (untested)
- [Slack](docs/slack.md) (untested)
- [Telegram](docs/telegram.md)
- [Webhook](docs/webhook.md)

### Could be added in the future
- [Zulip](https://zulip.com/api/)
- feel free to create an Issue or PR to implement another one!

### Don't seem to have an API 😞
- Signal

## Contributions welcome!

This crate is fairly new, and I only have a limited view where this is useful.
There might be other use cases I haven't seen so far.
Hints like "This crate would need X for …" are welcome!
Implementations for other Platforms are a great way to contribute too.

Documentation always needs some improvements.
After all you are the one trying to get it working and know best what's missing in the docs.
