# LAST STAND: ZOMBIE APOCALYPSE

A first-person 3D zombie survival prototype written in **Rust** with **Macroquad**, targeting WebAssembly and GitHub Pages.

## Current prototype

- First-person camera and movement
- Procedurally arranged abandoned city blocks
- Hostile zombies that pursue the player
- Hitscan pistol with limited ammunition and reload
- Health, kills, score, death and restart
- Keyboard/mouse controls and an initial touch-friendly control layout

## Controls

| Input | Action |
|---|---|
| W / A / S / D | Move |
| Mouse | Look around |
| Left click / Space | Fire |
| R | Reload |
| Enter | Start / restart |

On narrow screens, the prototype displays touch control hints. Mobile browser input support is still being refined.

## Run locally

Install Rust, then:

```bash
cargo run
```

## Build for WebAssembly

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

The GitHub Actions workflow builds the WebAssembly binary and deploys the `web/` page to GitHub Pages.

## GitHub Pages

In the repository settings, set **Pages → Build and deployment → Source** to **GitHub Actions**. The workflow deploys automatically on pushes to `main`.

## Roadmap

- Improved mobile joystick and look controls
- Better 3D models, materials, lighting and atmosphere
- Weapon handling, reload animation and audio
- Loot, inventory, crafting and survival needs
- Larger explorable districts and save system

## License

License not yet selected.
