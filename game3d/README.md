# Cube Collector 🎮

A simple 3D coin-collecting game built with **Rust** and **Macroquad**.

## Table of Contents

1. [About the Game](#about-the-game)
2. [Technologies](#technologies)
3. [Window Configuration](#window-configuration)
4. [Creating the Coin Mesh](#creating-the-coin-mesh)
5. [The Cube Struct](#the-cube-struct)
6. [The GameObject Enum](#the-gameobject-enum)
7. [Starting the Game](#starting-the-game)
8. [Generating Coins](#generating-coins)
9. [The Game Loop](#the-game-loop)
10. [Creating the Camera](#creating-the-camera)
11. [Drawing the Game Objects](#drawing-the-game-objects)
12. [Drawing the Grid](#drawing-the-grid)
13. [Creating the 2D UI](#creating-the-2d-ui)
14. [Collecting Coins](#collecting-coins)
15. [Player Movement](#player-movement)
16. [Closing the Game](#closing-the-game)
17. [Rust Concepts Learned](#rust-concepts-learned)

---

## 1. About the Game

Cube Collector is a 3D game where the player controls a cube and collects coins scattered around a grid.

The game includes:

* A 3D player cube.
* Ten coins placed at random positions.
* A 3D grid to represent the world.
* A camera that follows the player's position.
* Movement using WASD or the arrow keys.
* Collision detection to collect coins.
* A 2D user interface showing the title, instructions, and collection progress.
* A victory message when every coin has been collected.

## 2. Technologies

* **Rust:** The programming language used to build the game.
* **Macroquad:** The game framework responsible for the window, graphics, input, and rendering.
* **Rand:** The crate used to generate random coin positions.

The game uses a continuous game loop to update the player, check collisions, and draw each frame.

---

## 3. Window Configuration

```rust
use macroquad::prelude::*;
use ::rand::RngExt;
```

### `use macroquad::prelude::*;`

Imports commonly used Macroquad types, functions, and constants.

Examples used in this game include:

* `Vec3` for 3D positions.
* `Color` for colors.
* `Camera3D` for the camera.
* `draw_cube()` for drawing the player.
* `draw_text()` for displaying text.
* `KeyCode` for keyboard input.
* `WHITE`, `BLUE`, `GOLD`, and other color constants.

The `*` means to import all publicly available items from the specified module.

### `use ::rand::RngExt;`

Imports the random-number generation trait needed to use `random_range()` with the version of Rand used by this project.

### The `window_conf()` function

```rust
fn window_conf() -> Conf {
    Conf {
        window_title: "Cube Collector".to_owned(),
        sample_count: 10,
        ..Default::default()
    }
}
```

This function configures the game window.

* `fn` declares a function.
* `window_conf` is the function's name.
* `-> Conf` means it returns a Macroquad window configuration.
* `window_title` sets the window title.
* `.to_owned()` converts the string literal into an owned `String`.
* `sample_count: 10` requests 10 samples for multisample anti-aliasing, which can help smooth jagged edges.
* `..Default::default()` fills in the remaining configuration fields with their default values.

### The game entry point

```rust
#[macroquad::main(window_conf)]
async fn main() {
```

* `#[macroquad::main(window_conf)]` is a Macroquad attribute that sets up the game entry point and uses `window_conf()` to configure the window.
* `async fn main()` declares the asynchronous main function.
* The game runs inside this function.
* `async` allows the game to work with Macroquad's asynchronous frame loop, including `next_frame().await`.

---

## 4. Creating the Coin Mesh

The function `make_coin()` creates the 3D geometry for a coin.

```rust
fn make_coin(center_position: Vec3) -> Mesh {
```

* `make_coin` is the function's name.
* `center_position: Vec3` is the coin's center in 3D space.
* `-> Mesh` means the function returns a 3D mesh.

A mesh is geometry made from vertices and triangles.

### Setting the coin's dimensions

```rust
let radius = 0.5;
let height = 0.2;
let sides = 32;
```

* `radius` determines the coin's circular radius.
* `height` determines its thickness along the Z axis in this mesh.
* `sides` determines how many segments approximate the circle. With 32 segments, the coin looks approximately circular.

### Creating the vertex and index vectors

```rust
let mut verticies: Vec<Vertex> = Vec::new();
let mut indicies: Vec<u16> = Vec::new();
```

These vectors store the data needed to build the mesh.

* `Vec<Vertex>` stores the mesh's vertices.
* `Vec<u16>` stores indices that tell Macroquad which vertices form triangles.
* `Vec::new()` creates an empty vector.
* `mut` allows the vector to be modified.
* `u16` is an unsigned 16-bit integer type.

The variable names `verticies` and `indicies` work, but the conventional spellings are `vertices` and `indices`.

### Creating the back-face center

```rust
verticies.push(Vertex {
    color: GOLD.into(),
    position: vec3(
        center_position.x,
        center_position.y,
        center_position.z - height / 2.0,
    ),
    normal: vec4(0.0, 0.0, -1.0, 0.0),
    uv: vec2(0.0, 0.0),
});
```

This creates a vertex at the center of the coin's back face.

* `push()` adds a vertex to the vector.
* `color: GOLD.into()` assigns the gold color, converting it to the vertex color type.
* `position` specifies the vertex's location.
* Subtracting `height / 2.0` places it halfway through the coin's thickness, on the back side.
* `normal` specifies the direction associated with the face for lighting calculations.
* `uv` contains texture coordinates. Here, the coin does not use a texture, so these values are not important to its appearance.

### Creating the front-face center

```rust
verticies.push(Vertex {
    color: GOLD.into(),
    position: vec3(
        center_position.x,
        center_position.y,
        center_position.z + height / 2.0,
    ),
    normal: vec4(0.0, 0.0, 1.0, 0.0),
    uv: vec2(0.0, 0.0),
});
```

This creates the center of the front face.

The only important positional difference is `+ height / 2.0`, which places it on the opposite side of the coin.

The front and back centers allow the circular faces to be built from triangles.

### Generating the rim vertices

```rust
for i in 0..sides {
    let angle =
        i as f32 * 2.0 * std::f32::consts::PI / sides as f32;

    let x = center_position.x + radius * angle.cos();
    let y = center_position.y + radius * angle.sin();
```

This loop creates points around the circular edge.

* `0..sides` repeats 32 times, from 0 to 31.
* `i as f32` converts the integer index into a floating-point number.
* `PI` is the mathematical constant π.
* `cos()` calculates the horizontal component of the circle.
* `sin()` calculates the vertical component of the circle.

The formulas

`x = center_x + radius × cos(angle)`

`y = center_y + radius × sin(angle)`

place each point around a circle.

For each angle, the code creates two vertices:

1. A back-rim vertex at `center_position.z - height / 2.0`.
2. A front-rim vertex at `center_position.z + height / 2.0`.

This alternating arrangement is important because the triangle indices later rely on it.

### Connecting the vertices into triangles

```rust
for i in 0..sides {
    let current_back = (i * 2 + 2) as u16;
    let next_back = (((i + 1) % sides) * 2 + 2) as u16;

    let current_front = (i * 2 + 3) as u16;
    let next_front = (((i + 1) % sides) * 2 + 3) as u16;
```

This loop determines which vertices are used to form each segment of the coin.

The first two vertices in the mesh are the back and front centers, so the rim indices start at 2.

* `current_back` identifies the current back-rim vertex.
* `next_back` identifies the next back-rim vertex.
* `current_front` identifies the current front-rim vertex.
* `next_front` identifies the next front-rim vertex.
* `% sides` wraps the final segment back to the first segment, closing the circle.
* `as u16` converts the calculated index to the type required by the indices vector.

### Building the back face

```rust
indicies.extend_from_slice(&[0, next_back, current_back]);
```

This adds a triangle using the back center and two adjacent back-rim vertices.

`extend_from_slice()` appends several indices to the vector.

### Building the front face

```rust
indicies.extend_from_slice(&[1, current_front, next_front]);
```

This adds a triangle using the front center and two adjacent front-rim vertices.

Repeating these steps creates the complete circular faces.

### Building the side wall

```rust
indicies.extend_from_slice(&[
    current_back,
    next_back,
    current_front,
    next_back,
    next_front,
    current_front,
]);
```

Six indices create two triangles connecting adjacent back and front rim vertices.

Those triangles form a rectangular section of the coin's side wall.

Repeating this around the circle closes the thickness of the coin.

### Returning the mesh

```rust
Mesh {
    vertices: verticies,
    indices: indicies,
    texture: None,
}
```

This constructs and returns the completed mesh.

* `vertices` contains the points and their attributes.
* `indices` defines the triangles.
* `texture: None` means no texture image is assigned.

The result is a gold-colored, cylinder-like coin.

---

## 5. The Cube Struct

```rust
struct Cube<T> {
    cube_position: Vec3,
    cube_size: Vec3,
    texture: std::option::Option<T>,
    color: Color,
}
```

A `struct` groups related data together.

Instead of storing the cube's position, size, texture, and color separately, the game groups them into one `Cube` value.

### Understanding each field

* `cube_position: Vec3` stores the cube's position in 3D space.
* `cube_size: Vec3` stores its width, height, and depth.
* `texture: Option<T>` allows a texture to be present or absent.
* `color: Color` stores the cube's color.

### What does `<T>` mean?

`T` is a generic type parameter. It allows the struct to work with different texture types.

The field `Option<T>` can hold either:

* `Some(value)` — a value is present.
* `None` — no value is present.

In this game, the cube is initialized with `texture: None`.

---

## 6. The GameObject Enum

```rust
#[derive(Debug)]
#[derive(PartialEq)]
enum GameObject {
    Camera,
    Cube,
    Coin,
}
```

An `enum` defines a type that can represent one of several variants.

Here, a `GameObject` can be:

* `Camera`
* `Cube`
* `Coin`

The variants do not store the camera, cube, or coin's actual data. They identify which type of object an entry represents.

### `#[derive(Debug)]`

Allows values of `GameObject` to be formatted for debugging, for example with `println!("{:#?}", objects)`.

### `#[derive(PartialEq)]`

Allows values to be compared for equality using `==` and `!=`.

This is why the game can write:

```rust
if obj == &GameObject::Coin {
```

The enum makes it possible to store different kinds of object identifiers in one vector:

```rust
let mut objects: Vec<GameObject> = Vec::new();
```

The vector contains entries of one type, `GameObject`, even though those entries represent different variants.

---

## 7. Starting the Game

```rust
let cube_position = vec3(0.0, 0.5, 0.0);
```

Creates a 3D position at X = 0, Y = 0.5, and Z = 0.

The cube's Y coordinate is 0.5 because its height is 1.0, placing its bottom at ground level when the ground is Y = 0.

```rust
let mut objects: Vec<GameObject> = Vec::new();
```

Creates an empty vector that will contain the game's object identifiers.

```rust
let mut cube = Cube {
    cube_position: cube_position,
    cube_size: vec3(1.0, 1.0, 1.0),
    texture: None,
    color: BLUE,
};
```

Creates the player cube.

* Its position comes from `cube_position`.
* Its size is 1 unit along each axis.
* It has no texture.
* Its color is blue.

The code then adds the camera and cube identifiers:

```rust
objects.push(GameObject::Camera);
objects.push(GameObject::Cube);
```

The order matters because later code assumes the camera is at index 0 and the cube is at index 1.

---

## 8. Generating Coins

```rust
let mut coin_positions = Vec::new();

for _ in 0..10 {
    let mut rng = ::rand::rng();

    let x = rng.random_range(-9.0..9.0);
    let z = rng.random_range(-9.0..9.0);

    coin_positions.push(vec3(x, 0.5, z));
    objects.push(GameObject::Coin);
}
```

This generates ten coins.

### Understanding the loop

* `0..10` runs ten times.
* `_` means the loop index is not needed.
* `rand::rng()` obtains a random number generator.
* `random_range(-9.0..9.0)` chooses a random floating-point value in the range from -9 inclusive to 9 exclusive.
* `vec3(x, 0.5, z)` creates the coin's position.
* `coin_positions.push(...)` stores that position.
* `objects.push(GameObject::Coin)` adds a corresponding coin identifier.

The coins are generated before the main game loop. Therefore, their positions remain fixed during the current run.

The random generator could also be created once before the loop instead of once per coin.

### Printing the objects

```rust
println!("{:#?}", objects);
```

Prints the object vector to the terminal in a readable debug format. This is useful for checking the contents while developing.

---

## 9. The Game Loop

```rust
loop {
    clear_background(SKYBLUE);
```

The `loop` runs continuously until the game exits or the loop is broken.

`clear_background(SKYBLUE)` clears the previous frame and fills the window with a sky-blue background.

Each iteration represents one frame of the game.

The general structure is:

1. Clear the screen.
2. Create the camera.
3. Draw the 3D scene.
4. Draw the 2D interface.
5. Check for coin collection.
6. Process input and movement.
7. Wait for the next frame.

---

## 10. Creating the Camera

```rust
let camera = Camera3D {
    position: vec3(
        cube.cube_position.x,
        cube.cube_position.y + 2.0,
        cube.cube_position.z + 5.0
    ),
    target: cube.cube_position,
    up: vec3(0.0, 1.0, 0.0),
    ..Default::default()
};
```

This creates a 3D camera that follows the cube's position.

### `position`

The camera is placed:

* At the same X coordinate as the cube.
* Two units above the cube on the Y axis.
* Five units behind it on the Z axis.

This offset creates a view from above and behind the player.

### `target`

```rust
target: cube.cube_position
```

Tells the camera to look at the cube's position.

As the cube moves, the camera's position and target are recalculated in the next frame.

### `up`

```rust
up: vec3(0.0, 1.0, 0.0)
```

Defines the upward direction for the camera. Here, the Y axis is up.

### `..Default::default()`

Fills the remaining camera settings with their default values.

---

## 11. Drawing the Game Objects

```rust
let mut coin_id = 0;

for obj in &objects {
```

`coin_id` tracks which position in `coin_positions` belongs to the next coin being drawn.

`for obj in &objects` iterates through the object identifiers by reference.

### Activating the 3D camera

```rust
if obj == &GameObject::Camera {
    set_camera(&camera);
}
```

When the loop reaches the camera identifier, `set_camera()` activates the 3D camera.

This must happen before drawing the 3D objects.

### Drawing the cube

```rust
if obj == &GameObject::Cube {
    draw_cube(
        cube.cube_position,
        cube.cube_size,
        cube.texture,
        cube.color
    );
}
```

`draw_cube()` renders the player cube.

Its parameters are:

1. Position.
2. Size.
3. Optional texture.
4. Color.

### Drawing the coins

```rust
if obj == &GameObject::Coin {
    let coin = make_coin(coin_positions[coin_id]);

    draw_mesh(&coin);

    coin_id += 1;
}
```

For every coin identifier:

1. Retrieve the corresponding position from `coin_positions`.
2. Build a coin mesh at that position.
3. Draw the mesh with `draw_mesh()`.
4. Increment `coin_id` to prepare for the next coin.

This works because the code assumes the coin identifiers and coin positions are stored in the same order.

The mesh is rebuilt each frame in the current implementation. For a larger game, creating the meshes once and reusing them would be more efficient.

---

## 12. Drawing the Grid

```rust
draw_grid(20, 1.0, GRAY, DARKGRAY);
```

Draws the 3D grid.

* `20` controls the grid's divisions or extent as defined by Macroquad's function.
* `1.0` specifies the spacing between grid lines.
* `GRAY` and `DARKGRAY` specify the grid colors.

The grid is drawn after the object loop. Because the camera was activated by the first object in the vector, the 3D camera is still active when `draw_grid()` is called.

The ordering of camera changes and drawing commands is important: objects intended for the 3D world must be drawn while the 3D camera is active.

---

## 13. Creating the 2D UI

```rust
set_default_camera();
```

Switches from the 3D camera back to the default 2D screen coordinate system.

This lets the text remain fixed on the screen instead of being positioned in the 3D world.

### Game title

```rust
draw_text("Cube Collector", 20.0, 30.0, 30.0, WHITE);
```

The five parameters are:

1. Text to display.
2. X coordinate.
3. Y coordinate.
4. Font size.
5. Text color.

The X and Y coordinates are measured in screen pixels. The Y coordinate represents the text baseline.

### Instructions

```rust
draw_text("Collect all the coins!", 20.0, 60.0, 20.0, WHITE);
```

Displays the game's objective beneath the title.

### Collection progress

```rust
let progress = format!(
    "Coins collected: {}/10",
    10 - coin_positions.len()
);

draw_text(progress, 20.0, 90.0, 20.0, WHITE);
```

`coin_positions.len()` returns the number of coins remaining.

Since the game begins with ten coins, subtracting the remaining count from ten calculates how many have been collected.

`format!()` builds a string containing the current progress.

### Controls at the bottom-right

```rust
let font_size = 20.0;
let text = "Use W,A,S,D or Arrow Keys to move the character";

let text_width =
    measure_text(text, None, font_size as u16, 1.0).width;

draw_text(
    text,
    screen_width() - text_width - 20.0,
    screen_height() - 20.0,
    font_size,
    WHITE,
);
```

`measure_text()` calculates the width of the text in the selected font.

The horizontal coordinate is calculated by subtracting the text width and a 20-pixel margin from the screen width. This positions the text near the bottom-right corner.

`screen_width()` and `screen_height()` return the current screen dimensions.

### Victory message

```rust
let message = "You Collected All the coins!";
let font_size = 50.0;

let text_size =
    measure_text(message, None, font_size as u16, 1.0);
```

Measures the victory message so it can be centered.

```rust
if 10 - coin_positions.len() == 10 {
    draw_text(
        message,
        (screen_width() - text_size.width) / 2.0,
        (screen_height() + text_size.height) / 2.0,
        font_size,
        GREEN,
    );
}
```

The condition becomes true when all ten coins have been collected.

The horizontal coordinate subtracts the text width from the screen width and divides the remaining space equally.

The vertical coordinate accounts for the text's height and baseline, placing it around the center of the screen.

---

## 14. Collecting Coins

```rust
let mut coin_to_remove = None;
```

Creates an optional index to remember which coin should be removed.

Initially, no coin has been selected.

### Checking every coin

```rust
for (index, position) in coin_positions.iter().enumerate() {
    let cube_pos = cube.cube_position;
    let coin_pos = *position;

    let distance = cube_pos.distance(coin_pos);
```

* `iter()` iterates through the coin positions.
* `enumerate()` provides each position's index as well.
* `*position` dereferences the reference to obtain the `Vec3` value.
* `distance()` calculates the distance between the cube's center and the coin's center in 3D space.

### Detecting a collision

```rust
if distance <= 1.0 && objects.len() > index + 2 {
    coin_to_remove = Some(index);
    break;
}
```

If the distance is at most 1.0, the code treats the cube and coin as close enough to count as a collection.

The second condition checks that the object vector contains the expected coin index.

`Some(index)` stores the index of the coin to remove.

`break` stops the loop once a coin has been selected, so at most one coin is collected by this check in a frame.

### Removing the coin

```rust
if let Some(index) = coin_to_remove {
    objects.remove(index + 2);
    coin_positions.remove(index);
}
```

This code runs after the iteration finishes.

* `if let Some(index)` runs only when an index was selected.
* `objects.remove(index + 2)` removes the matching coin identifier. The `+ 2` accounts for the camera and cube at the start of the vector.
* `coin_positions.remove(index)` removes the corresponding position.

Removing the position means that coin will no longer be drawn, and the progress display will update.

Keeping the two vectors synchronized is essential because the drawing code uses one vector to identify coins and the other to find their positions.

---

## 15. Player Movement

The player moves by changing the cube's position in the game loop.

### Forward movement

```rust
if is_key_down(KeyCode::W) ||
   is_key_down(KeyCode::Up) && cube.cube_position.z > -10.0 {
    cube.cube_position.z -= 0.1;
}
```

When the relevant key is held, the Z coordinate decreases, moving the cube forward in this game's coordinate setup.

### Backward movement

```rust
if is_key_down(KeyCode::S) ||
   is_key_down(KeyCode::Down) && cube.cube_position.z < 10.0 {
    cube.cube_position.z += 0.1;
}
```

Increases the Z coordinate, moving the cube backward.

### Left movement

```rust
if is_key_down(KeyCode::A) ||
   is_key_down(KeyCode::Left) && cube.cube_position.x > -10.0 {
    cube.cube_position.x -= 0.1;
}
```

Decreases the X coordinate, moving the cube left.

### Right movement

```rust
if is_key_down(KeyCode::D) ||
   is_key_down(KeyCode::Right) && cube.cube_position.x < 10.0 {
    cube.cube_position.x += 0.1;
}
```

Increases the X coordinate, moving the cube right.

### `is_key_down()` versus `is_key_pressed()`

`is_key_down()` returns true for as long as the key is held down. This is useful for continuous movement.

`is_key_pressed()` returns true when the key is newly pressed. This is useful for actions that should happen once per press, such as collecting or removing a specific object with a button.

**Note about the movement conditions:** Rust evaluates `&&` before `||`. As currently written, the boundary condition is not applied to both keys in each pair. To enforce the limits for both keys, group the key checks in parentheses before applying the boundary condition.

For example:

```rust
if (is_key_down(KeyCode::W) || is_key_down(KeyCode::Up))
    && cube.cube_position.z > -10.0
{
    cube.cube_position.z -= 0.1;
}
```

The same principle applies to the other movement directions.

---

## 16. Closing the Game

```rust
if is_key_down(KeyCode::Escape) {
    break;
}
```

When Escape is held, `break` exits the game loop.

```rust
next_frame().await;
```

This tells Macroquad to proceed to the next frame.

The `await` pauses this asynchronous function until the next frame is ready. Without the frame wait, the loop would not follow Macroquad's normal frame-by-frame execution.

---

## 17. Rust Concepts Learned

Building Cube Collector introduced several important programming concepts.

| Concept                 | How the game uses it                                                            |
| ----------------------- | ------------------------------------------------------------------------------- |
| Functions               | `window_conf()` and `make_coin()`                                               |
| Structs                 | Storing the cube's position, size, texture, and color                           |
| Generics                | The `Cube<T>` type                                                              |
| Enums                   | Representing cameras, cubes, and coins                                          |
| Vectors                 | Storing object identifiers and coin positions                                   |
| Loops                   | Generating coins and drawing objects                                            |
| Conditions              | Handling input and detecting collisions                                         |
| `Option`                | Representing a missing texture or a coin that has not been selected for removal |
| References              | Iterating over objects and coin positions                                       |
| Random numbers          | Placing coins in different locations each run                                   |
| Geometry                | Building a coin from vertices and triangles                                     |
| Async functions         | Running the Macroquad game loop                                                 |
| Input handling          | Moving the player using the keyboard                                            |
| Rendering               | Drawing 3D objects and 2D text                                                  |
| String formatting       | Displaying collection progress                                                  |
| Borrowing and ownership | Iterating over a vector before removing an element                              |

## Final Thoughts

Cube Collector is a useful first step into 3D game development with Rust. It demonstrates how a game can combine custom geometry, object data, random placement, camera setup, keyboard input, collision detection, and a screen-space UI.

The game also shows an important programming lesson: when two data structures represent the same objects, they must remain synchronized. In a future version, keeping each coin's position directly inside its object representation could simplify collection and removal.

**Project status:** First 3D game completed. Ready to start the next game!
