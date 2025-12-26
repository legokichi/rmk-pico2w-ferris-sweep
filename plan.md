rmk に Azoteq TPS65 トラックパッドのドライバを追加したい。


- TPS65の詳細
  - https://www.digikey.jp/ja/products/detail/azoteq-pty-ltd/TPS65-201A-S/7164942
  - https://holykeebs.com/products/touchpad-module

TPS65 は iqs5xx のチップを積んでおり、以下のドライバのサンプルが存在する。

参考実装として

- rust 独自実装のもの
  - https://www.reddit.com/r/ErgoMechKeyboards/comments/10hbco2/added_a_tps65_touchpad_to_eskarp/?tl=ja
  - https://github.com/legokichi/iqs5xx
- zmk のもの
  - https://www.reddit.com/r/ErgoMechKeyboards/comments/1mm73zi/zmk_driver_for_azoteq_trackpads/
  - https://github.com/legokichi/zmk-driver-azoteq-iqs5xx
  - https://github.com/legokichi/zmk-keyboard-iqs5xx-dev

を git submodule で用意している。


./rmk-driver-azoteq-iqs5xx フォルダを作り、そこに rmk 用の iqs5xx ドライバを実装せよ。

## 実装計画
1. 既存実装の把握: `iqs5xx` と `zmk-driver-azoteq-iqs5xx` の初期化手順・レポートフォーマット・ジェスチャー処理を整理する。
2. RMK 側の入力デバイス設計: `rmk/rmk/src/input_device/pmw3610.rs` を参考に、`InputDevice` + `InputProcessor` の責務を分離する。
3. 新規クレート設計: `./rmk-driver-azoteq-iqs5xx` に `no_std` クレートを作成し、I2C + RDY/RST ピン + 非同期待機を扱える API にする。
4. 変換ロジック実装: IQS5xx のレポート/ジェスチャーから RMK の `Event`（Joystick/AxisEventStream）を生成し、`MouseReport` への変換も用意する。
5. RMK との統合: `rmk`/`rmk-config`/`rmk-macro` に設定項目と初期化コードを追加し、`keyboard.toml` から設定可能にする。
6. 動作確認: `cargo build --release --bin central/peripheral` または `cargo make build` でビルド確認し、最低限のログ出力で初期化と入力イベントが流れることを確認する。

## 実装指示書
- `./rmk-driver-azoteq-iqs5xx` を新規作成し、`Cargo.toml`/`src/lib.rs` を用意する（`#![no_std]`、`embedded-hal-async` と `embassy-time` を利用）。
- IQS5xx のレジスタ/レポート定義は `iqs5xx` の実装を参照し、必要最小限を新クレート内に移植する（`no_main` は使わない）。
- `Iqs5xxConfig` を定義し、I2C アドレス、座標スケール、軸反転、タップ/スクロール設定など最低限の調整項目を持たせる。
- `Iqs5xxDevice` を実装して `InputDevice` を満たす（RDY ピン待機 → レポート取得 → `Event::Joystick` か `Event::AxisEventStream` を返す）。
- `Iqs5xxProcessor` を実装して `InputProcessor` を満たす（移動は `MouseReport` の `x/y`、二本指スクロールは `wheel/pan` を想定）。
- RMK 統合:
  - `rmk/rmk/src/input_device/mod.rs` にモジュールを追加。
  - `rmk/rmk-config/src/lib.rs` の `InputDeviceConfig` に `iqs5xx` 設定を追加。
  - `rmk/rmk-macro/src/input_device` に展開ロジックを追加し、RP2040/RP2350 の I2C と GPIO 初期化を生成。
- `keyboard.toml` に設定例を追加する（必要なら README にも簡潔な使用例を追記）。
- 生成物 (`pico2wh-*.uf2` 等) は追加しない。

## rmk-driver-azoteq-iqs5xx 実装内容（現状）
- 新規クレート `rmk-driver-azoteq-iqs5xx` を追加（`#![no_std]`、edition 2024）。
- `src/registers.rs` に IQS5xx の主要レジスタ/ビット定義を移植。
- `src/lib.rs` に非同期 I2C ドライバを実装（`embedded-hal-async` + `embassy-time`）。
- `Iqs5xxConfig` を追加（I2C アドレス、リセット/待機時間、ジェスチャー有効化、軸反転/XY 入替、感度系など）。
- `Report`/`Touch`/`Event` を定義し、レポート→イベント変換を実装。
- `rmk` feature で RMK 連携モジュール `rmk_support` を用意:
  - `Iqs5xxDevice` が `InputDevice` を実装（RDY 監視 + レポート取得 + タップ/スクロール/移動を `Event` 化）
  - `Iqs5xxProcessor` が `InputProcessor` を実装（移動を `MouseReport` に、スクロールは `wheel/pan` に変換）
  - タップ/ホールドは `Event::Custom` にエンコードしてボタン操作へ変換

## 未対応/今後の課題
- `rmk` 側（`rmk-config`/`rmk-macro`）への統合は未実施。
- `keyboard.toml` 設定からの自動生成は未対応。
- 実機での初期化/イベント動作確認は未実施。
