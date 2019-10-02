# this allows caching of instability.
# making switches less annoying but also less up to date
import (
    builtins.fetchGit (
    {
        url = "https://github.com/NixOS/nixpkgs";
        ref = "master";
        rev = "68d0a7114fe748958ec69096004902c6acfa2202";
    }
    ))
