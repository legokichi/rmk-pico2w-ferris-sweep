Ferris Sweep v2.2 (Pico 2 W) 向け配線反映タスク

目的: pin.md の配線情報を keyboard.toml / vial.json に反映し、Vial で正しい物理配置になるようにする。

1) pin.md を GPIO 番号に変換
- Pico 2 W の物理ピン → GPIO を確定
- 左右それぞれ 3x5 + 2（親指）の並びを「4行×5列」の direct pin 行列に整理
- 未使用セルは PIN_UNUSED にする

2) keyboard.toml の更新
- [layout] を rows=8, cols=5 に更新
- matrix_map を L(0-3行)/R(4-7行) に整理
- [split.central]/[split.peripheral] の rows/cols/offset を 4x5 前提に更新
- direct_pins を pin.md 反映版に置き換え
- トラックパッドが PIN_2/PIN_3 を使っている場合はピン競合を解消

3) vial.json の更新
- matrix.rows=8, matrix.cols=5 に更新
- layouts.keymap を Ferris Sweep 標準 (split_3x5_2) 配列に合わせる
- vendorId/productId/name は現状維持

4) ビルド/確認
- cargo make uf2 で左右 UF2 を生成
- Vial でレイアウト表示/キー位置が一致するか確認

5) 記録
- 変更点を README.md に簡単に追記（必要なら）
- 生成物 (pico2wh-*.uf2) はコミットしない

補足: pin 配線メモ（pin.md 抜粋）
- keyboard.toml の `PIN_XX` は Pico 2 W の `GPXX` に対応
- GND はファーム側には出てこない（配線の参照用）

左手（L）
| Phys Pin | GPIO (PIN_XX) | Key | Note |
| --- | --- | --- | --- |
| 32 | PIN_27 | G |  |
| 31 | PIN_26 | - |  |
| 30 | GND | GND |  |
| 29 | PIN_22 | GND |  |
| 28 | GND | Z | GND (7番と連結) |
| 27 | PIN_21 | X |  |
| 26 | PIN_20 | C |  |
| 25 | PIN_19 | V |  |
| 24 | PIN_18 | B |  |
| 23 | GND | Q | GND (5番と連結) |
| 22 | PIN_17 | 4 |  |
| 21 | PIN_16 | 3 |  |
| 4 | PIN_2 | T | 13番と連結 |
| 5 | PIN_3 | Q | 23番と連結 |
| 6 | PIN_4 | S | 18番と連結 |
| 7 | PIN_5 | Z | 28番と連結 |
| 13 | GND | T | GND |
| 14 | PIN_10 | R |  |
| 15 | PIN_11 | E |  |
| 16 | PIN_12 | W |  |
| 17 | PIN_13 | A |  |
| 18 | GND | S | GND |
| 19 | PIN_14 | D |  |
| 20 | PIN_15 | F |  |

右手（R）
| Phys Pin | GPIO (PIN_XX) | Key | Note |
| --- | --- | --- | --- |
| 32 | PIN_27 | H |  |
| 31 | PIN_26 | - |  |
| 30 | RUN | - |  |
| 29 | PIN_22 | - |  |
| 28 | GND | ? | GND (12番と連結) |
| 27 | PIN_21 | > |  |
| 26 | PIN_20 | < |  |
| 25 | PIN_19 | M |  |
| 24 | PIN_18 | N |  |
| 23 | GND | P | GND (10番と連結) |
| 22 | PIN_17 | 1 |  |
| 21 | PIN_16 | 2 |  |
| 9 | PIN_6 | Y | 13番と連結 |
| 10 | PIN_7 | P | 23番と連結 |
| 11 | PIN_8 | L | 18番と連結 |
| 12 | PIN_9 | ? | 28番と連結 |
| 13 | GND | Y | GND |
| 14 | PIN_10 | U |  |
| 15 | PIN_11 | I |  |
| 16 | PIN_12 | O |  |
| 17 | PIN_13 | : |  |
| 18 | GND | L | GND |
| 19 | PIN_14 | K |  |
| 20 | PIN_15 | J |  |
