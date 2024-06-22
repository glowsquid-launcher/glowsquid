# Tauri Plugin state

## A Stupid simple state management plugin for Tauri

Let's say you want to manage state between your frontend and backend, but you want to keep it in sync and _completely_ managed from rust.
This plugin is for you.

## Panic guide

This plugin considers not being able to sync state as an invariant violation and will panic if it can't sync state. We always want to be in sync to make sure nothing goes wrong.
