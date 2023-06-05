# rscenes

Rscene is a scene manager for [Raylib](https://crates.io/crates/raylib).

## Installation

```sh
cargo add rscenes
```

## Sample

You don’t need to include `raylib`, the following line alone is enough:

```rust
use rscene::prelude::*
```

Then, in your function, instantiate the builder and the manager:

```rust
let mut builder = raylib::init();
builder.title("my-game"); // this sets WM_CLASS
let font: Option<Font> = None;
let mut manager = SceneManager::new(builder, font);
manager.config(|handle, thread, font| {
    // Here you set the window title, otherwise it’s gonna be the same as
    // the WM_CLASS.
    handle.set_window_title(thread, "My Game");
    // You can call any handle method you need here.
    // For instance, the default framerate is 60fps, you can change it here:
    handle.set_target_fps(30);
    // Or load a font:
    font.insert(handle.load_font(thread, "font.ttf").unwrap());
});
manager.add_first_scene(Box::new(MyScene::default()));
manager.start()?;
```

The scene should be implemented like:

```rust
#[derive(Debug, Default)]
struct MyScene;

impl Scene<Option<Font>> for MyScene {
    fn init(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) -> anyhow::Result<()> {
        // Perform any initialisation you need here
        Ok(())
    }

    fn update(
        &mut self,
        (handle, thread): (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        resources: &mut Option<Font>,
    ) -> anyhow::Result<State<Option<Font>>> {
        // Per frame update:
        // dt is time since last frame in seconds.
        Ok(State::Keep)
    }

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        resources: &Option<Font>,
    ) -> anyhow::Result<()> {
        // Instantiate your RaylibMode2D or RaylibMode3D and draw here.
        // This is rendered once per frame.
        Ok(())
    }
}
```

The main resources are:

- [`Scene`](https://docs.rs/rscenes/latest/rscenes/prelude/trait.Scene.html)
- [`SceneManager`](https://docs.rs/rscenes/latest/rscenes/prelude/struct.SceneManager.html)
- [`State`](https://docs.rs/rscenes/latest/rscenes/prelude/enum.State.html)
- [`colors`](https://docs.rs/rscenes/latest/rscenes/prelude/colors/)
- [`raylib`](https://docs.rs/raylib/3.7.0/raylib) (Raylib itself)

Everything else is exposed from
[`raylib::prelude`](https://docs.rs/raylib/3.7.0/raylib/prelude/).

## License

- [The 3-Clause BSD License](https://opensource.org/license/bsd-3-clause/)
- [`COPYING`](https://github.com/cacilhas/rscenes/blob/master/COPYING)

License: BSD-3-Clause
