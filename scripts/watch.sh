#! /bin/sh


set -x

(sleep 1 && touch src/main.rs) & # makes it go the first time

# no nead to rebuild, cargo is reliable
# -nickto97

# TODO use: https://github.com/schell/steeloverseer#readme
# I think it does this better
while inotifywait -r -e modify -e create -e attrib ./src ./shell.nix ./Cargo.toml;
do
    # since ghc needs to compile both backend, frontend and run the tests,
    # ghcjs should 'win' the race, I noticed some dangling ghcjs'es after a while
    make build
     # && \
    # make enter EXTRA="--run \"make haddock\""
done
