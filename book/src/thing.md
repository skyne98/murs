# Thing
## Overview
Every object (a "thing") has a set of properties that describes its state, material and physical behaviors. Characters, enemies, items and environment props all are considered "things" and have above mentioned properties.

Those properties are minimal, allowing for fast prop creation.
```
Rock:
    (properties)
    material: Granite
    (state)
    weight: 27.5 kg
    volume: 0.01 m3
    max hit-points: round(1.5 hp)
    hit-points: 1 hp
```

## Materials
Materials of items describe their volume in relation to their weight (mass), their hit points in relation to their weight (mass) and their ability to withstand temperatures.
```
Diamond:
    density: 3539 kg/m3
    strength: 28000 hp/m3
    max temperature: 4000 °c
```
```
Granite:
    density: 2750 kg/m3
    strength: 150 hp/m3
    max temperature: 1260 °c
```

### Creating a new material
To create a new material find or make up its properties. First of all, take its density in kg/m3 which allows calculating volume and weight. Second of all, find this material's ultimate tensile strength and multiply it by 10. Now you can use this value as hit-points/m3.