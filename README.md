# rscenes

[COPYING]: https://github.com/cacilhas/rscenes/blob/master/COPYING
[The 3-Clause BSD License]: https://opensource.org/license/bsd-3-clause/
[Raylib]: https://crates.io/crates/raylib
[raylib::prelude]: https://docs.rs/raylib/3.7.0/raylib/prelude/

## Rscene

Rscene is a scene manager for [Raylib][].

### Installation

```sh
cargo add rscenes
```

### Sample

You don’t need to include `raylib`, the following line alone is enough:

```rust
use rscene.prelude::*
```

Then, in your function, instantiate the builder and the manager:

```rust
let mut builder = raylib::init();
builder.title("my-game"); // this sets WM_CLASS
let mut manager = SceneManager::new(builder);
manager.config(|handle, thread| {
    // Here you set the window title, otherwise it’s gonna be the same as
    // the WM_CLASS.
    handle.set_window_title(thread, "My Game");
    // You can call any handle method you need here.
});
manager.add_first_scene(Box::new(MyScene::default()));
manager.start()?;
```

The scene should be implemented like:

```rust
#[derive(Debug, Default)]
struct MyScene;

impl Scene for MyScene {
    fn init(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> anyhow::Result<()> {
        // Perform any initialisation you need here
        Ok(())
    }

    fn update(
        &mut self,
        (handle, thread): (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        audio: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<State> {
        // Per frame update:
        // dt is time since last frame in seconds.
        Ok(State::Keep)
    }

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        font: Option<Rc<Font>>,
        audio: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<()> {
        // Instantiate your RaylibMode2D or RaylibMode3D and draw here.
        // This is rendered once per frame.
        Ok(())
    }
}
```

The main resources are:

- [`Scene`](./prelude/trait.Scene.html)
- [`SceneManager`](./prelude/struct.SceneManager.html)
- [`State`](./prelude/enum.State.html)
- [`colors`](./prelude/colors)

Everything else comes from [`raylib::prelude`][raylib::prelude].

### License

- [The 3-Clause BSD License][]
- [COPYING][]

License: BSD-3-Clause
