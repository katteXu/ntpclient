单例模式导致集成运行失败 原因未知

<!--
1. 构建
2. 代码签名 (可不签名)
3. 构建产物放到 cloudflare 项目下 src-tauri/bin 文件下
4. 修改对应平台名称(运行一个脚本)
5. 构建 cloudflare -->

Github Action 自动化构建编写

1.  远程拉去代码 仓库 (默认 https://github.com/katteXu/ntpclient.git 可自定义输入)
2.  判断是否为 rust 项目 不是则退出
3.  构建 不同平台的 rust 项目
4.  将构建产物(target/release/ntpclient windows: ntpclient.exe linux: ntpclient) 保存到 当前项目下(src-tauri/bin)
5.  将文件名修改成对应平台下的文件 例如 mac arm 下(ntpclient 修改成 ntpclient-aarch64-apple-darwin)
6.  构建本地项目
    构建逻辑大概是这样的
    jobs:
    publish-tauri:
    permissions:
    contents: write
    strategy:
    fail-fast: false
    matrix:
    include: - platform: "macos-latest" # for Arm based macs (M1 and above).
    args: "--target aarch64-apple-darwin" - platform: "macos-latest" # for Intel based macs.
    args: "--target x86_64-apple-darwin" - platform: "ubuntu-22.04" # for Tauri v1 you could replace this with ubuntu-20.04.
    args: "" - platform: "windows-latest"
    args: ""

        runs-on: ${{ matrix.platform }}
        steps:
          - uses: actions/checkout@v4

          - name: setup node
            uses: actions/setup-node@v4
            with:
              node-version: lts/*

          - name: install Rust stable
            uses: dtolnay/rust-toolchain@stable
            with:
              # Those targets are only used on macos runners so it's in an `if` to slightly speed up windows and linux builds.
              targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}

          - name: install dependencies (ubuntu only)
            if: matrix.platform == 'ubuntu-22.04' # This must match the platform value defined above.
            run: |
              sudo apt-get update
              sudo apt-get install -y libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
            # webkitgtk 4.0 is for Tauri v1 - webkitgtk 4.1 is for Tauri v2.
            # You can remove the one that doesn't apply to your app to speed up the workflow a bit.

          - name: install frontend dependencies
            run: yarn install # change this to npm, pnpm or bun depending on which one you use.
          - name: install bun
            run: yarn add bun -g
          - uses: tauri-apps/tauri-action@v0
            env:
              GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
              APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
              APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
              APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
            with:
              tagName: app-v__VERSION__ # the action automatically replaces \_\_VERSION\_\_ with the app version.
              releaseName: "App v__VERSION__"
              releaseBody: "See the assets to download this version and install."
              releaseDraft: true
              prerelease: true
              args: ${{ matrix.args }}
