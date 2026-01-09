# Issue: Pico 2 W BLE advertise not visible / debug logging confusion

## 現象
- BLEアドバタイズが確認できない。
- defmt RTTログが出ない／`probe-rs attach` で `locked up core` になることがある。

## 原因と解決（判明分）
### 1) BLE広告が「見えない」原因
- BLE広告自体は開始されていたが、USBスタックが `Device enabled` を出した時点で
  `USB_ENABLED` がシグナルされ、**USB優先モードへ切り替わって広告が止まる**挙動になっていた。
  - USB給電なしでも `Device enabled / suspended` が出るケースがあり、誤ってUSB優先になっていた。
- 解決: `rmk/rmk/src/usb/mod.rs` の `enabled()` で `USB_ENABLED.signal()` を行わず、
  **`configured()` でのみ `USB_ENABLED` をシグナル**するように変更。
  - これでUSB未接続時はBLE広告が継続する。

### 2) BLE初期化でパニックする原因
- `keyboard-nosplit.toml` の `name/product_name` が長すぎると
  `Device name is too long. Max length is 22 bytes` でパニック。
- 解決: デバイス名を22バイト以下（`rmk-p2w`）に短縮。

## 確認済み（ハードウェア）
- `../rust-sandbox/pi-pico-2-w` の `debug` バイナリは `cargo run` で RTT が流れる。
  - よって **debug probe / 配線 / ボード自体は正常**。

## 最小構成での検証（このリポジトリ内）
### USB-only（BLEなし）
- `keyboard-smoke.toml` + `src/bin/rmk_smoke.rs` を追加し、最小USB構成で起動確認。
- `cargo run --release --bin rmk_smoke` で defmt ログが流れることを確認。
  - 例: `[smoke] controller init`, `Device enabled`, `rmk alive: 0..`
- 結論: **RMK自体は Pico 2 W で動作可能（USB-only）**。

### BLEあり（smoke構成）
- `pico_w_ble` を有効化し、`keyboard-smoke.toml` の `[ble] enabled = true` に変更。
- `cyw43` 初期化ログ（CLMロード/BT init）が出ており、**BLE初期化自体は動作**。
- `Overwritten(ChipInit)` で BLE初期化の詳細ログを挿入してもロックアップは再現せず。
- 結論: **BLE初期化は Pico 2 W で動作する**（少なくとも smoke 構成ではOK）。

## 途中で出たエラー（再現条件に依存）
- `probe-rs attach` が `locked up core` になるケースがあった。
- `probe-rs run` 実行中に RTT が出ない場合があった。
  - ただし smoke 検証でログ出力は確認済み。
- `usb_enable = false` にすると `no usb info for the chip` でビルド失敗
  - `rmk` マクロが `usb_info` を必須としているため（現状はUSB無効化が困難）。

## 現在のリポジトリ内の変更点（要整理）
- `keyboard-smoke.toml` 追加（最小USB/BLE切り分け用）
- `src/bin/rmk_smoke.rs` 追加（最小RMK起動+defmtログ）
  - `Overwritten(ChipInit)` / `Overwritten(entry)` で詳細ログあり
- `src/cyw43-firmware` → `../cyw43-firmware` のシンボリックリンク追加
  - `src/bin/../cyw43-firmware/...` の include 参照対策
- `keyboard-nosplit.toml` 追加（分割を外したBLE広告確認用）
  - デバイス名を `rmk-p2w` に短縮
- `Cargo.toml` の依存を更新
  - `embassy-rp` 0.9 / `cyw43` 0.6 / `cyw43-pio` 0.9
  - `rmk` features: `pico_w_ble` + `async_matrix` + `storage` + `controller`
- `rmk/rmk/src/usb/mod.rs`
  - `enabled()` で `USB_ENABLED.signal()` を行わないよう変更（USB誤判定対策）
- `src/central.rs` は最小構成に戻している

## 次のアクション候補
1. 本命構成（`src/central.rs` + `keyboard.toml`）へ段階的に戻し、BLE広告の有無を再確認。
2. smoke用の `Overwritten` を除去して、本番相当の経路でBLE広告を確認。
3. 必要なら `probe-rs debug` で HardFault 状態の取得を再試行。
