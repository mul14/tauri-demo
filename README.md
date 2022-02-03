# Tauri Demo

This project use Vite + Vue 3.

## How to run

```
npm install
npm run tauri dev
```

## How to build for production

```
npm install
npm run tauri build
```

Check file `tauri-demo` inside `./src-tauri/target/release`

## How to generate icon

Put PNG file (1024x1024) on root directory `./app-icon.png`.
Then run this command to generate icons.

```
npm run tauri icon
```

The icons will generated inside `./src-tauri/icons` directory.
