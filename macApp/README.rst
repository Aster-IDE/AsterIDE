macOS building stuff
====================

This isn't actually needed, it's just to keep nicely integrated
with the Apple ecosystem, and also implement building the .app
easier to work with.

Process of building
===================

This is a just a wrapper subproject. You can still build via
`cargo r`.

This actual subproject will wrap around that, but allow it
to be dunbled for macOS correctly, and allow for better DX
for things like entitlements (app intents), and changing called
resources like an app icon.
