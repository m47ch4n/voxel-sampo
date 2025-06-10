# voxel sampo

```
       _  _   __  _  _  ____  __      ____   __   _  _  ____   __        
      / )( \ /  \( \/ )(  __)(  )    / ___) / _\ ( \/ )(  _ \ /  \       
      \ \/ /(  O ))  (  ) _) / (_/\  \___ \/    \/ \/ \ ) __/(  O )      
       \__/  \__/(_/\_)(____)\____/  (____/\_/\_/\_)(_/(__)   \__/       
```

![screenshot](https://github.com/m47ch4n/voxel-sampo/blob/main/etc/screenshot.png?raw=true)

A voxel-based 3D scene viewer built with the Bevy engine featuring physics-based player movement with jumping mechanics. 

## Requirements

- Rust 1.70.0 or higher
- Cargo

## Run

```bash
cargo run
```

## Controls

- `WASD` to move
- `Space` to jump
- `Q` to rotate camera counter-clockwise
- `E` to rotate camera clockwise
- `F3` to toggle debug mode

## Features

- **Realistic Physics**: Natural jumping with ballistic trajectories
- **Dynamic Damping**: Different air resistance when grounded vs airborne
- **Voxel Collision**: Automatic collision mesh generation from .vox files
- **Camera Controls**: Smooth rotation with snap-to-angle positioning

## Technologies & Libraries

- [Bevy](https://bevyengine.org/) 0.16.0 - Game engine
- [bevy_vox_scene](https://github.com/oliver-dew/bevy_vox_scene) - VOX file loading
- [bevy_rapier3d](https://github.com/dimforge/bevy_rapier) 0.30.0 - Physics simulation

## License

MIT License - See [LICENSE](LICENSE) file for details.