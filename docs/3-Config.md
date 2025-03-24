# Configuration Files

All configurations are stored as [JSON](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/JSON) in `F4SE\Plugins\Telekinesis2\` and structured as such:

```
    Devices.json            <- Device settings (enabled/disabled/body parts/actuator limits)
    Logging.json

    BoneTracking.json       <- Bone tracking sampling frequency, physical security limits, etc.
    DefaultRaceFemale.json
    DefaultRaceMale.json

    Actions/*
    Patterns/*
    Races/*
    Triggers/*
    Variables/*
```

Directories marked with wildcards `*` can contain multiple files, which are interpreted based on their directory name, i.e. every file in `Actions` must be some kind of `Actions.json`.

This structure allows to easily package and redistribute premade settings as "regular" archives consumed by a mod manager.

## Actions/*

Named actions that can be performed on phyiscal devices. These actions can be referenced in triggers.

## Patterns/*

Funscript files ending in `.funscript` or `.vibrator.funscript`

## Races/*

Race and gender-specific data used by the bone tracker. Each entry specifies a set of bone names and collision data, for a given combination of race (by form-id) and sex (male or female).

Humans, Robots or any other enemies can have different bodies with different bone names depending on which mods you run. For example, each female body mod (Fusion Girl, CBBE, etc) might require its own `DefaultRaceFemale.json` in this directory.

## Triggers/*


