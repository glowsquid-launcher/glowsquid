# copper

## A rust minecraft launcher

[![for the badge](https://forthebadge.com/images/badges/0-percent-optimized.svg)](https://forthebadge.com)
[![for the badge](https://forthebadge.com/images/badges/60-percent-of-the-time-works-every-time.svg)](https://forthebadge.com)
[![for the badge](https://forthebadge.com/images/badges/contains-tasty-spaghetti-code.svg)](https://forthebadge.com)
[![for the badge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
[![for the badge](https://forthebadge.com/images/badges/mom-made-pizza-rolls.svg)](https://forthebadge.com)
[![Wakatime](https://wakatime.com/badge/github/glowsquid-launcher/copper.svg?style=for-the-badge)](https://wakatime.com/badge/github/glowsquid-launcher/copper)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fglowsquid-launcher%2Fcopper.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fglowsquid-launcher%2Fcopper?ref=badge_shield)

This is meant to be a mid-level structural launcher. It works in 3 steps:

- authenticate
- download
- launch!

You get to decide how to authenticate, either internally (via the `auth` module) or externally (Just get the token and user info and you're good).

Downloading and launching, ditto.

**MICROSOFT ONLY** because Mojang is being removed in the future. Migrate now _or else_

Currently, this is being used as the launcher backend for the Glowsquid launcher.

## Features

- [x] can actually download minecraft
- [x] parses version manifests for you
- [x] proper error handling

## TODO

- Ability to refresh existing tokens
- Full tauri bindings

## Testing

TODO: refactor testing

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fglowsquid-launcher%2Fcopper.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fglowsquid-launcher%2Fcopper?ref=badge_large)
