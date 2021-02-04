# demver - Deterministic Version Manager
Pin the exact hashes of docker images while following a tag.
Think **git submodules for docker**.

## What is this?
Bob is a developer.
He likes to use docker in his projects.
Bob wants to be up to date, so he uses the `base-image:latest` tag in his Dockerfile.
Bob takes a vacation.
After he returns from his vacation, his application is not working anymore!
But he didn't change anything!
Turns out `base-image:latest` points to a different version now.
