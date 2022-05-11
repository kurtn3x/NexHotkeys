**Windows only.

***Example usage in src/main.rs.
Uses a slightly modified version of inputbot:
https://github.com/obv-mikhail/InputBot

**What it does
If you initialize a hotkey with exactly 1 keyboard key:
event will return true once the key is released, so 1 keypress returns exactly 1 true value.

If you initialize a hotkey with more than 2 keyboard keys: event will return true, once all hotkeys are pressed at the same time. It will be able to return true again, once at least 1 key has been lifted.

