
# Celeste character physics

- Celeste seemingly uses purely time for time limits and not frames
  - like rather then subtracting 1 every frame, it subtracts the delta time every frame
  - Maybe this isn't that weird, but it could explain why the game feels so smooth
- `MoveH` and `MoveV` (`MoveH/V(int, cb)`)
  - moves player on the x/y axis by the given amount. calls cb on collision
  - [How `MoveH` and `MoveV` works](https://maddymakesgames.com/articles/celeste_and_towerfall_physics/index.html)
- `MoveHExact` and `MoveVExact` (`MoveH/VExact(int)`)
  - moves player on the x/y axis by the given amount. I believe it ignores collision
- I believe Maddy said that all of the physics engine using ints and not floats which is helpful

## Code locations

- `NormalUpdate`: Gravity, Jumping, and Friction
- `Update`: All encompassing update function for things that aren't a state
  - `Update` calls none of the state functions. They are all controlled by the `StateMachine` class. ([StateMachine source](https://github.com/JamesMcMahon/monocle-engine/blob/master/Monocle/Components/Logic/StateMachine.cs))
  - I don't really know where the state machine is updated, but its a part of monocle's ECS so its probably through that.
    - Im assuming its using some reflection stuff in the backend so thats probably where its called
    - ok yea looking into the ctor Add is called on the state machine
    - the state `Update` fns return an int which is the state to switch to next. Returns the current state if no change

## Rust reimplementation

- `MoveX` and `MoveY` should prob the condensed into a `self.move` method
  - You could pretty much get the function of `Movex` and `MoveY` with a macro
    - `fn move(&mut self, vec: Vec2, cb: impl Fn(&mut Self, Vec2) -> ())`
    - `x(3) = Vec2::new(3,0)`
    - `self.move(x!(self.speed), |_, _| {})` or something
