# Hachimi Mod Template
Cross-platform Unity IL2CPP modding template written in Rust, based on [Hachimi](https://github.com/Hachimi-Hachimi/Hachimi).

### Supported Platforms
- Windows
- Android (aarch64, armv7a, x86_64, i686)

# Usage
When using this template to make your own mod, remember to:
- Change the package name in `Cargo.toml`.
- Change `MODNAME`, `MODID` and `SONAME` in `tools/android/build_zygisk.sh` (if you want to use Zygisk).
- Replace `api.rs` and `types.rs` in `src/il2cpp` with appropriate modules generated for your game's Unity version, if needed. The included modules should work for Unity 6000.0.x. You can generate them with [hachimi-bindgen](https://github.com/LeadRDRK/hachimi-bindgen).
- Check `src/il2cpp/hook/UnityEngine_CoreModule` for some example hooks (remove them if you don't need them).
- Disclose your mod's source code under [GNU GPLv3](LICENSE).

### Windows
- Injecting the mod: Rename the dll to `winhttp.dll` or `version.dll` and put it in the game's directory.
- Logs are written to `OutputDebugStringW`. You can use [DebugView](https://learn.microsoft.com/en-us/sysinternals/downloads/debugview) to view them.

### Android
You may use the scripts included in [`tools/android`](tools/android) to build the mod. See the README file included in [`tools`](tools) and [`tools/android`](tools/android) for more info.

To inject your mod on Android:
1. Extract the APK file of the game. You might want to use [apktool](https://apktool.org/) for this.
2. Rename the `libmain.so` file in each of the folders inside `lib` to `libmain_orig.so`.
3. Copy the proxy libraries to their corresponding folders (e.g. `libmain-arm64-v8a.so` goes to `lib/arm64-v8a`). Rename them to `libmain.so`.
4. Build the APK file and install it.

You can view the logs through logcat. The log tag is your mod's package name.

# License
[GNU GPLv3](LICENSE)