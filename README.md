# Tic-Tac-Toe 🦀

A tic-tac-toe game where **all the game logic and the AI are written in Rust**,
compiled to **WebAssembly**, and played in the browser. The computer opponent
uses the **minimax** algorithm, which makes it *unbeatable* — the best you can do
is draw.

▶️ **Play:** `https://<your-username>.github.io/tic-tac-toe/`
(replace `<your-username>` with your GitHub username)

## How it works

- `src/lib.rs` — the whole game: the board, win detection, and the minimax AI.
  It is exposed to JavaScript with `wasm-bindgen`.
- `index.html` — a small web page that loads the compiled WebAssembly and draws
  the board. It calls into Rust for every move.
- `.github/workflows/deploy.yml` — a GitHub Action that compiles the Rust to
  WebAssembly and publishes it to GitHub Pages automatically on every push.

**You do not need to install Rust on your computer.** GitHub compiles everything
for you in the cloud.

## Put it on GitHub and play it (no local setup)

1. Create a new repository on GitHub named **`tic-tac-toe`**.
2. Upload these files to it (drag-and-drop in the browser, or `git push`).
3. In the repo, go to **Settings → Pages** and set **Source** to
   **"GitHub Actions"**.
4. Go to the **Actions** tab. The "Deploy to GitHub Pages" workflow runs
   automatically. Wait for the green check ✅ (about 1–2 minutes).
5. Open `https://<your-username>.github.io/tic-tac-toe/` and play!

Every time you push a change, the site rebuilds and redeploys by itself.

### Uploading with git (optional)

```bash
git init
git add .
git commit -m "Tic-tac-toe in Rust + WASM"
git branch -M main
git remote add origin https://github.com/<your-username>/tic-tac-toe.git
git push -u origin main
```

## Running it on your own computer (optional)

Only needed if you want to develop locally. Requires
[Rust](https://rustup.rs) and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/).

```bash
# Compile the Rust to WebAssembly
wasm-pack build --target web --out-dir pkg

# Serve the folder (a plain file:// open will NOT work — browsers block WASM modules)
python3 -m http.server 8080
# then open http://localhost:8080
```

## Running the tests

```bash
cargo test
```

This includes an exhaustive proof that the AI cannot be beaten.
