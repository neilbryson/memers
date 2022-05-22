# memers

Meme generator

## Memes

### Drake
```bash
memers drake <TOP_TEXT> <BOTTOM_TEXT> <OUTPUT_PATH>
```

#### Example
```bash
memers drake "TOP TEXT" "Hello world" drake.jpg
```
![](docs_assets/drake.jpg)

### GigaChad (GIF)
```bash
memers gigachad <TEXT> <OUTPUT_PATH>
```

### Example
```bash
memers gigachad "Average Rust Fan" giga.gif
```

### Average Fan vs. Average Enjoyer (GigaChad) [GIF]
```bash
memers fve <FAN_TEXT> <ENJOYER_TEXT> <OUTPUT_PATH>
```

### Example
```bash
memers fve "Average Iron Oxide Fan" "Average Rust Fan" giga.gif
```

## Building locally

### Debug mode

```bash
cargo run
```

### Release build

```bash
cargo build --release
```

## Fonts
Using [Bebas Neue](https://github.com/dharmatype/Bebas-Neue).

## License
MIT