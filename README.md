# Flip Ship
Think Tetris crossed with F-Zero; 2D would be easier, 3D would be nore fun!

## Description 
Dodge enemies and catch ship parts; rotate your ship to utilize the different modules on each side. The bigger the ship, the more parts it can hold - but you'll also be a bigger target with more to manage.

## Levels

### 1. LEO (Low Earth Orbit)
- the "mother-ship" blew up, so it's time to assemble a new one
- usefull for low risk tutorial
- stages
  1. build a hull: gather `hull` pieces (learn translation)
  2. need power: gather `battery` pieces (learn rotation)
- movement: `6-axis snap`
  - translation: 3-axis
  - rotation: 3-axis
- win steps
  - gather "enough" hull pieces
  - gather "enough" batteries 

### 2. Descent
- the ship is falling toward the "planet"
- a challenge of manuverability
- stages
  1. uh oh: dodge debris, collect shields (learn new movement) 
  2. whats cooking: avoid overheating (shield charge)
  3. still falling: dodge debris, collect thrusters
  4. look out below: decelerate with bursts (thruster cooldown)
- movement: `constant forward`
  - translation: 2-axis (orthogonal to `FORWARD`) (constant `FORWARD`)
  - rotation: 3-axis
- win steps
  - collect shields to slow speed
  - dont overheating 
  - collect thrusters to land
  - avoid crashing 

### 3. Roll Around
- need to build a land based configuration
- definitely NOT katamari
- stages
  1. get big: collect `hull` pieces (learn new movement)
  2. arms and armor: collect `shields` and `weapons`
  3. need wheels: collect `wheels`
- movement: `roly poly`
  - translation: NONE
  - rotation: 2-axis (orthogonal to `UP`)
- win steps
  - recollect `hull` pieces
  - collect "enough" (3?) tank tracks
  - collect armor
  - collect weapons

### 4. Tank Battle
- heading to enemy base (they're responsable for the mother-ship catastrophy)
- tank battle?
- stages
  1. move: get to checkpoint 
  2. shoot: defeat enemies 
  3. win: defeat big boss
- movement: `tank`
  - translation: 1-axis (controlled `FORWARD`)
  - rotation: 1-axis `UP`
- win steps
  - defeat enemies
  - defeat enemy boss guarding base

### 5. Trash
- approach enemy base interior for infiltration
- needs to be a trash level
- stages
  1. sneak: learn the movement in layer 1
  2. get the watch: obtain the watch
  3. hop the fence: infiltrate into layer 2
  4. get the gun: get "Walt's PP"
  5. skip the wall: infiltrate into layer 3
  6. get the fancy: get the bow-tie
  7. jump the dimentional rift: infiltrate into the base
- movement: `side slide`
  - translation: 2-axis (horizontal only)
  - rotation: NONE
- win steps
  - collect trash for power
  - infiltrate enemy base

### 6. Spy
- infiltrate enemy base for convert ops
- needs a suit
- stages
  1. main objective: shut down "The Grid"
  2. side objective: rescue hostages
  3. side objective: retrive secret files (silly photos)
  4. escape: before the base blows up
- movement: `wiggle walk`
  - transtion: NONE
  - rotation: 1-axis (pivot around `UP`)
- win steps
  - get secret documents
  - save hostages
  - sabatoge

### 7. Boats
- boats
- stages
  1. learn the ropes: learn the sailing mechanic (going with the wind)
  2. a-hoy: fight off the pirates
  3. booty: steal the treasure from the flag ship
  4. sail away: sail the rainbow to the next level
- movement: `sailing`
  - translation: 1-axis (controlled `FORWARD`)
  - rotation: 1-axis (pivot around `UP` - like a boat)

### 8. Western
- final land boss (player has never encountered and will never encounter again)
- ye-haw
- little voice
- stages
  1. enter saloon: walk around, and talk up the patroons
  2. blackjack: get the map
  3. poker: get the air clearance codes
  4. go fish: get grandma's coockies
  5. shootout: kill the boss at high noon
- movement: `wiggle-walk`
  - transtion: NONE
  - rotation: 1-axis (pivot around `UP`)

### 9. Flying
- gotta fly
- stages
  1. clear skies (`free flying`): learn the ropes of free-flying
  2. valley of focus (`forward flying`): infiltrate the 1st zone
  3. disable and defend (`free flying`): disable shield defence nodes, get trapped until defeating the retaliating forces
  4. threat canyon (`forward flying`): tighter run than that valley
  5. disable and defend 2 (`free flying`): defeating the retaliating forces, attacking node when possible (it moves around)
  6. tunnel of terror (`forward flying`): even tighter run than the canyon
  7. bosses fights (`free flying`): fight the flying saucer and the building transformer, each has a key to open the sky dome
  8. down the tubes (`tunnel twist`): avoid the obstacles, one shot the boss
- movement: `free flying` / `forward flying`
  - translation: NONE (constant, controlled `FORWARD`)
  - rotation: 3-axis (limited, dynamically-stable pitch (`RIGHT`) and yaw (`UP`) / roll (`FORWARD`) for fun)

### 10. Dance
- rythm game
- must blend in
- gotta get a `visa` to enter the "middle cloud layer"
- stages
  1. 2-step: simple movement
  2. 3-step: the waltz
  3. boogie: boogie
  4. free-style: combine sick moves
- movement: `wiggle-walk`

### 11. Lift-Off
- leave the planet to head home
- stages
  1. lift-off: controls practice (collect energy)
  2. customs: quiz time (answer the questions flawlessly)
  3. punch-it: leave the atmosphere (fight enemies)
  4. watch out: accelerate to orbial velocity (avoid debris)
- movement: `forward flying`
  - translation: 2-axis (orthogonal to `FORWARD`) (constant `FORWARD`)
  - rotation: 1-axis (`FORWARD`)
- win steps
  - defeat enemies
  - collect energy
  - avoid falling debris
  - reach orbit

### 12. Assemble New Mother Ship
- must build a hype-drive
- stages:
  1. `hull`
  2. `batteries`
  3. `hyper drive`
- movement: `6-axis snap`

### 13. Re-Final Ultimate Battle: Is It Improper for an Assembled Ship to Destroy God?
- fight god
- anime battle
- big voice
- stages: each is random, but they shift in two basic cycles until god is defeated
  - build up parts
  - fight god
- movement: `ALL`

## User Experience
- [ ] user can move the ship depending on the movement schema
  - [ ] 6-axis snap
    - [ ] pitch(X): controller | right-stick | vertical 
    - [ ] roll(Z): controller | right-stick | horizontal
    - [ ] yaw(Y): controller | triggers
    - [ ] frontback(Z): controller | left-stick | vertical 
    - [ ] sideways(X): controller | left-stick | horizontal 
    - [ ] updown(Y): controller | a/b-buttons
  - [ ] forward flying
    - [ ] pitch(X): controller | right-stick | vertical 
    - [ ] roll(Z): controller | right-stick | horizontal
    - [ ] yaw(Y): controller | triggers
    - [ ] updown(Y): controller | left-stick | vertical 
    - [ ] sideways(X): controller | left-stick | horizontal
  - [ ] side slide
    - [ ] frontback(Z): controller | left-stick | vertical 
    - [ ] sideways(X): controller | left-stick | horizontal 
    - [ ] updown(Y): controller | a/b-buttons
  - [ ] roly poly (pivots around ground contact)
    - [ ] pitch(X): controller | right-stick | vertical 
    - [ ] roll(Z): controller | right-stick | horizontal
  - [ ] wiggle walk:
    - [ ] rightpivot: controller | right-stick | directional 
    - [ ] leftpivot: controller | left-stick | directional 
    - [ ] rightlock: controller | right-trigger
    - [ ] leftlock: controller | left-lock
  - [ ] free flying
    - [ ] yaw(Y): controller | right-stick | horizontal
    - [ ] pitch(X): controller | right-stick | vertical 
    - [ ] boostbrake(Z): controller | a/b-buttons
  - [ ] sailing
    - [ ] sail(Y): controller | right-stick | directional
    - [ ] speed(Z): controller | a/b-buttons
  - [ ] tank
    - [ ] righttrack: controller | right-stick | vertical 
    - [ ] lefttrack: controller | left-stick | vertical
  - [ ] tunnel twist
    - [ ] pivot(XY): controller | right-stick | directional
- [ ] user can "catch" `hull` and `action` pieces
- [ ] user can extend the size (via adding surfaces) with the `hull` pieces
- [ ] user can add functionality with the `action` pieces
  - [ ] weapons
    - [ ] lazer
    - [ ] projectile
    - [ ] bombs
    - [ ] shockwave?
  - [ ] shields
    - [ ] deflector
    - [ ] funnel
  - [ ] batteries (for energy)
  - [ ] engines
    - [ ] thrusters
    - [ ] wings?
    - [ ] wheels?
    - [ ] springs?
- [ ] user ca be attacked by two types of weapons
  - [ ] damaging: tearing part the ship
  - [ ] shifting: inducing movement on the ship
    - [ ] translation 
    - [ ] rotation 
