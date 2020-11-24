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

Class S (200-300 durability):
```
Meteorite Steel:
    density: 3730 kg/m3
    class: S
    durability: 200 hp/kg
```
Class A (100-199 durability):
```
Diamond:
    density: 3539 kg/m3
    class: A
    durability: 100 hp/kg
```
Class B (35-99 durability):
```
Steel:
    density: 8050 kg/m3
    class: B
    durability: 35 hp/kg
```
Class C (10-34 durability):
```
Granite:
    density: 2750 kg/m3
    class: C
    durability: 10 hp/kg
```
Class D (3-9 durability):
```
Bone:
    density: 1850 kg/m3
    class: D
    durability: 5 hp/kg
```
```
Oak:
    density: 2750 kg/m3
    class: D
    durability: 3 hp/kg
```
Class F (0-2 durability):
```
Meat:
    density: 1093 kg/m3
    class: D
    durability: 2 hp/kg
```
```
Glass:
    density: 2500 kg/m3
    class: D
    durability: 2 hp/kg
```

### Creating a new material
To create a new material find or make up its properties. First of all, take its density in kg/m3 which allows calculating volume and weight. Second of all, find this material's ultimate tensile strength and multiply it by 10. Now you can use this value as hit-points/m3.