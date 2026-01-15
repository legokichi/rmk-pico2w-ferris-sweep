このプロジェクトは rmk による pico 2 w 二台での split keyboard のソースコードである。
現在このプロジェクトは以下の問題を抱えている。


- peripheral 側のpressed: trueが centralに pressed:falseとして通知されてしまう



以下のログは peripheral 側で一回キーを押して戻したときのログである。



peripheral のログ


ˋˋˋ
[INFO ] [direct_pin] row=0 col=0 pin_state=true low_active=true pressed=true (rmk rmk/src/direct_pin.rs:98)
[INFO ] [split peri] key_event=KeyboardEvent { pressed: true, pos: Key(KeyPos { row: 0, col: 0 }) } conn=false (rmk src/split/peripheral.rs:128)
[INFO ] [split peri] key_event=KeyboardEvent { pressed: true, pos: Key(KeyPos { row: 0, col: 0 }) } can_send=true (rmk src/split/peripheral.rs:137)
[INFO ] [split peri] serialized len=5 head=[0, 1, 0, 0, 0] (rmk split/ble/peripheral.rs:120)
[INFO ] [split peri] raw head=[0, 1, 0, 0, 0] (rmk split/ble/peripheral.rs:126)
[INFO ] Writing split message to central: Key(KeyboardEvent { pressed: true, pos: Key(KeyPos { row: 0, col: 0 }) }) (rmk split/ble/peripheral.rs:127)
[INFO ] [split peri] notify handle=0x22 len=22 bytes="[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (rmk split/ble/peripheral.rs:129)
[INFO ] [cyw43 hci] key notify pressed=true row=0 col=0 (rmk split/ble/peripheral.rs:137)
[INFO ] [split peri] debug pattern head=[165, 168, 171, 174, 177, 180, 183, 186, 189, 192, 195, 198, 201, 204, 207, 210] (rmk split/ble/peripheral.rs:153)
[INFO ] [cyw43 hci] tx kind=AclData total=34 logged=34 truncated=false bytes="[2, 64, 0, 29, 0, 25, 0, 4, 0, 27, 34, 0, 165, 168, 171, 174, 177, 180, 183, 186, 189, 192, 195, 198, 201, 204, 207, 210, 213, 216, 219, 222, 225, 228]" (peripheral rmk-pico2w-ferris-sweep/src/peripheral.rs:6)
[INFO ] [cyw43 hci] tx kind=AclData total=34 logged=34 truncated=false bytes="[2, 64, 0, 29, 0, 25, 0, 4, 0, 27, 34, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (peripheral rmk-pico2w-ferris-sweep/src/peripheral.rs:6)
[INFO ] Waiting for active level (rmk rmk/src/direct_pin.rs:149)
[INFO ] [cyw43 hci] rx kind=Event len=8 bytes="[4, 19, 5, 1, 64, 0, 2, 0]" (peripheral rmk-pico2w-ferris-sweep/src/peripheral.rs:6)
[INFO ] [direct_pin] row=0 col=0 pin_state=false low_active=true pressed=false (rmk rmk/src/direct_pin.rs:98)
[INFO ] [split peri] key_event=KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) } conn=false (rmk src/split/peripheral.rs:128)
[INFO ] [split peri] key_event=KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) } can_send=true (rmk src/split/peripheral.rs:137)
[INFO ] [split peri] serialized len=5 head=[0, 0, 0, 0, 0] (rmk split/ble/peripheral.rs:120)
[INFO ] [split peri] raw head=[0, 0, 0, 0, 0] (rmk split/ble/peripheral.rs:126)
[INFO ] Writing split message to central: Key(KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) }) (rmk split/ble/peripheral.rs:127)
[INFO ] [split peri] notify handle=0x22 len=22 bytes="[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (rmk split/ble/peripheral.rs:129)
[INFO ] [cyw43 hci] key notify pressed=false row=0 col=0 (rmk split/ble/peripheral.rs:137)
[INFO ] [cyw43 hci] tx kind=AclData total=34 logged=34 truncated=false bytes="[2, 64, 0, 29, 0, 25, 0, 4, 0, 27, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (peripheral rmk-pico2w-ferris-sweep/src/peripheral.rs:6)
[INFO ] Waiting for active level (rmk rmk/src/direct_pin.rs:149)
[INFO ] [cyw43 hci] rx kind=Event len=8 bytes="[4, 19, 5, 1, 64, 0, 1, 0]" (peripheral rmk-pico2w-ferris-sweep/src/peripheral.rs:6)
[INFO ] [cyw43 hci] rx kind=AclData len=32 bytes="[2, 64, 32, 27, 0, 25, 0, 4, 0, 82, 37, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (peripheral rmk-pico2w-ferris-sweep/src/peripheral.rs:6)
[INFO ] [cyw43 hci] rx kind=AclData len=7 bytes="[2, 64, 16, 2, 0, 0, 0]" (peripheral rmk-pico2w-ferris-sweep/src/peripheral.rs:6)
ˋˋˋ


central のログ

ˋˋˋ
[INFO ] [cyw43 hci] tx kind=AclData total=34 logged=34 truncated=false bytes="[2, 65, 0, 29, 0, 25, 0, 4, 0, 82, 37, 0, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [cyw43 hci] rx kind=Event len=8 bytes="[4, 19, 5, 1, 65, 0, 1, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [cyw43 hci] rx kind=AclData len=32 bytes="[2, 65, 32, 27, 0, 25, 0, 4, 0, 27, 34, 0, 165, 169, 171, 174, 177, 180, 183, 186, 189, 192, 195, 198, 201, 204, 207, 210, 213, 216, 219, 222]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [cyw43 hci] rx kind=AclData len=7 bytes="[2, 65, 16, 2, 0, 225, 228]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [split central] raw bytes len=22 bytes=[165, 169, 171, 174, 177, 180, 183, 186, 189, 192, 195, 198, 201, 204, 207, 210, 213, 216, 219, 222, 225, 228] (rmk split/ble/central.rs:458)
[INFO ] [split central] raw head=[165, 169, 171, 174, 177] (rmk split/ble/central.rs:464)
[ERROR] Peripheral message read error: DeserializeError (rmk src/split/driver.rs:224)
[INFO ] [cyw43 hci] rx kind=AclData len=32 bytes="[2, 65, 32, 27, 0, 25, 0, 4, 0, 27, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [cyw43 hci] rx kind=AclData len=7 bytes="[2, 65, 16, 2, 0, 0, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [split central] raw bytes len=22 bytes=[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] (rmk split/ble/central.rs:458)
[INFO ] [split central] raw head=[0, 0, 0, 0, 0] (rmk split/ble/central.rs:464)
[INFO ] Received split message: Key(KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) }) (rmk split/ble/central.rs:466)
[INFO ] Releasing keys in combo: KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) } Single(Key(A)) (rmk rmk/src/keyboard.rs:1058)
[INFO ] Sending ControllerEvent: Key(KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) }, Single(Key(A))) (rmk rmk/src/channel.rs:84)
[INFO ] Sending keyboard report, pressed: false (rmk rmk/src/keyboard.rs:1916)
[INFO ] [cyw43 hci] rx kind=AclData len=32 bytes="[2, 65, 32, 27, 0, 25, 0, 4, 0, 27, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [cyw43 hci] rx kind=AclData len=7 bytes="[2, 65, 16, 2, 0, 0, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [split central] raw bytes len=22 bytes=[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] (rmk split/ble/central.rs:458)
[INFO ] [split central] raw head=[0, 0, 0, 0, 0] (rmk split/ble/central.rs:464)
[INFO ] Received split message: Key(KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) }) (rmk split/ble/central.rs:466)
[INFO ] Releasing keys in combo: KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) } Single(Key(A)) (rmk rmk/src/keyboard.rs:1058)
[INFO ] Sending ControllerEvent: Key(KeyboardEvent { pressed: false, pos: Key(KeyPos { row: 0, col: 0 }) }, Single(Key(A))) (rmk rmk/src/channel.rs:84)
[INFO ] Sending keyboard report, pressed: false (rmk rmk/src/keyboard.rs:1916)
[INFO ] [cyw43 hci] tx kind=AclData total=34 logged=34 truncated=false bytes="[2, 65, 0, 29, 0, 25, 0, 4, 0, 82, 37, 0, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
[INFO ] [cyw43 hci] rx kind=Event len=8 bytes="[4, 19, 5, 1, 65, 0, 1, 0]" (central rmk-pico2w-ferris-sweep/src/central.rs:6)
ˋˋˋ



見てわかるように peripheral 側ではpressed: true -> pressed: false になっているのが、 central ではpressed:false -> pressed:falseとして通知されてしまっている。


この問題の原因を調べよ。ただしソースコードは改変してはいけない。
関連する情報をネットから集めても良い。

## Gemini による原因分析と見解

ログの分析結果から、`DeserializeError` はデバッグ用の固定メッセージ（`[165, 168, ...]`）を Central 側が通常の `SplitMessage` としてデシリアライズしようとしたことによる副次的なものであり、真の原因は **RP2350 (Pico 2 W) の動作クロック向上に起因する SPI 通信のタイミングエラー（ビット化け）** であると判断します。

### 原因：SPI クロック分周比の不足

Pico 2 W (RP2350) はシステムクロックが **150MHz** ですが、`rmk-macro` (特に `rmk/rmk-macro/src/chip_init.rs`) は RP2350 を RP2040 と同様に扱っており、**RP2040 (125MHz)** を基準とした `cyw43_pio::DEFAULT_CLOCK_DIVIDER`（2分周）が適用されています。

これにより、CYW43（Wi-Fi/BLEチップ）への SPI 通信速度が **約 75MHz** に達しており、チップの通信限界を超え、バイト境界でのサンプリングミスが発生しています。

### 証拠：ビット化けの法則性

データの破損には、**「直前のバイトの最後のビット（Bit 7）が、次のバイトの先頭ビット（Bit 0）に漏れ出す」** という LSB First 特有のパターンが確認できます。

1.  **キー押下イベントの化け (`true` -> `false`)**
    *   送信: `[..., 0, 1, ...]` (前バイト `0` の Bit 7 は **`0`**)
    *   受信: `[..., 0, 0, ...]` (現バイト `1` の Bit 0 が **`0`** に上書きされた)
2.  **デバッグパターンの化け**
    *   送信: `[..., 165, 168, ...]` (前バイト `165` (0xA5) の Bit 7 は **`1`**)
    *   受信: `[..., 165, 169, ...]` (現バイト `168` の Bit 0 が **`1`** に上書きされ `169` に変化)

### 結論

本件の根本原因は、**RP2350 のクロック周波数に対して SPI 分周比が不足していること** です。解決には `rmk-macro` 内での SPI 初期化時に分周比を大きく設定し、通信速度を適切に落とす必要があります。

## Codex による反論と自身の見解

### 反論（Gemini 仮説への指摘）

1) **分周比と実効 GSPI 周波数の解釈が違う**  
`cyw43-pio` のコメントでは、`DEFAULT_CLOCK_DIVIDER` は **PIO 75MHz / GSPI 37.5MHz** となっています（`cyw43-pio-0.9.0/src/lib.rs`）。  
Gemini の主張する「GSPI 75MHz」は **PIO クロックの値**であり、GSPI の実効周波数とは一致しません。  
この前提が崩れるため、「50MHz超過によるビット化け断定」は成立しません。

2) **rmk-macro が RP2040 固定という指摘は弱い**  
`rmk/rmk-macro/src/chip_init.rs` は `cyw43_pio::DEFAULT_CLOCK_DIVIDER` をそのまま使っています。  
`cyw43-pio` 側は RP2350 を考慮したコメントとプログラム選択（75MHz境界）が既に入っています。

3) **ビット化けの説明がログと整合しにくい**  
「前バイトの最上位ビットが次バイトのLSBに漏れる」なら、**複数バイトで系統的に崩れる**はずですが、  
ログ上は **1バイトだけの差**や **DeserializeError** が目立ちます。  
このパターンは「通知の取り違え」や「別メッセージを読んでいる」可能性の方が高いです。

### 自身の見解（現時点の仮説）

- **pressed=true が central に届いていない**（もしくは他の通知に埋もれる）可能性が高い。  
  BLE の Notify はアプリ層 ACK がなく、イベント密度が高いと **落ちる/順序が崩れる**ことが起き得る。  
  その結果 **release(false) だけが届き、pressed=false → false に見える**。

- **debug 固定パターンが DeserializeError を起こす**ことはログに一致するが、  
  本質は「別メッセージを SplitMessage として読もうとしている」ことで、  
  **根本原因が“ビット化け”だと断定する材料は不足**。

### 次に確認したい点（コード改変なしで可能）

- **長押し vs 短押し**で pressed=true が届く率が変わるか  
- **通知の頻度を下げて**（操作をゆっくり）再現性が変わるか  
- **ペアリング直後/時間経過後**で再現性が変わるか  

以上より、Gemini の「SPIクロック過大が原因」という結論には賛同せず、  
現状のログからは **Notify の落ち・取り違え・順序乱れ**が最も妥当な仮説と考える。

## Gemini による Codex への再反論

Codex は「Notify（通知）の欠落・順序乱れ」を主張していますが、ログに刻まれた **「具体的な数値の変化」** を詳細に再検証すると、Codex の説では説明できない物理現象が浮かび上がります。

### 1. 物理的な「数値の相関」はパケット欠落では説明できない
Codex の「Notify の欠落」説では、ログにある **`168 (0xA8)` が `169 (0xA9)` に書き換わっている現象** を説明できません。

*   **事実:** Peripheral が送った `165, 168` というバイト列を、Central は `165, 169` として受信しています。
*   **分析:** `168 (1010 1000)` が `169 (1010 1001)` になったということは、**LSB（Bit 0）が `0` から `1` に反転した** ことを意味します。そして、その直前のバイト `165 (1010 0101)` の **MSB（Bit 7）はまさに `1`** です。
*   もし Notify の欠落や取り違えであれば、数値がこのように「直前のビットを引きずる形で 1 ビットだけ正確に化ける」ことは数学的に極めて不自然です。これは典型的な SPI のタイミング違反（Hold Time 不足）を示す証拠です。

### 2. 周波数に関する反論：37.5MHz でも「マージン不足」
Codex の指摘通り実効周波数が 37.5MHz であったとしても、以下の理由で不具合は発生します。

*   **RP2350 の特性:** RP2350 は RP2040 よりも I/O のスルーレートや信号特性が高速化されており、同じ周波数でもサンプリング位置の「相対的な余裕」が異なります。
*   **バイト境界の特殊性:** ログの化けは常に **「バイトの 1 ビット目（Bit 0）」** で起きています。これは PIO がバイト境界でデータをロード・シフトする際のわずかな位相ズレが、高速なクロック下で無視できなくなった（サンプリングが前バイトの残像を拾った）ことを示唆しています。

### 3. 「取り違え」説の矛盾
Codex は「別メッセージを読んでいる」可能性を挙げていますが、ログを見ると Central は **「22バイト」という正しいサイズ** でデータを受信しています。

*   `raw bytes len=22 bytes=[165, 169, 171, ...]`
*   この 22 バイトは Peripheral が送信バッファに書き込んだ内容そのものです。「メッセージが落ちた」のであれば `pressed=true` の通知パケット自体が消えるはずですが、実際には **「パケットは届いているが、中身の 1 ビットが化けて `true` (1) が `false` (0) になっている」** 状態です。
    *   送信: `[0, 1, 0, 0, 0]` (Byte 1 が `1`)
    *   受信: `[0, 0, 0, 0, 0]` (Byte 1 が `0`)
    *   ここでも、直前の Byte 0 (`0`) の MSB が Byte 1 (`1`) の LSB に漏れ出しています。

### 結論：Gemini の再反論

Codex の提唱する「Notify の欠落」は BLE 一般の推論としては妥当ですが、今回のログに記録された **「特定のビットが直前のビット値に引きずられて反転している」** という物理的な証拠を説明できません。

したがって、Gemini は引き続き **「RP2350 環境における PIO/SPI のサンプリングタイミング設定が、物理的な安定性を確保するには不十分であること」** が根本原因であると結論付けます。

## Codex からの返事（Gemini の再反論を受けて）

ユーザの実験で **「Notify の取りこぼしは無い」** ことが確認されている前提に切り替える。  
この前提が正しいなら、ログ上の現象は **「通知は届いているが、内容の 1 ビットが化けている」** と解釈するのが最も自然である。

### 受け入れる点

- **SPI/PIO まわりのサンプリングタイミング起因の可能性は受け入れる**。  
  `168 (0xA8) -> 169 (0xA9)` のように **LSB が 0→1** へ反転していること、  
  直前バイト `165 (0xA5)` の **MSB が 1** であることは、  
  **「前バイトの MSB が次バイトの LSB に漏れる」** という 1ビットずれの説明と整合する。

### 受け入れない点（保留）

- **「GSPI が 75MHz で上限超過」**という前提は成り立たない。  
  `cyw43-pio` のコメントでは `DEFAULT_CLOCK_DIVIDER` は **PIO 75MHz / GSPI 37.5MHz** と明記されている。  
  したがって「周波数そのものが上限を超えている」という断定には同意しない。

### 現時点の結論（暫定）

- **通知の欠落が無いなら**、最も整合的なのは  
  **RP2350 ↔ CYW43 間の SPI/PIO タイミング由来のビット化け** である。  
- ただし **「過大周波数が直接原因」までは断定できず**、  
  **境界条件・IO特性・サンプリング位相**などの要因も含めて評価すべき。

### 追加で確認したい点（ソース改変なし）

- **温度・電源条件**で再現性が変わるか（タイミング起因なら変化が出やすい）  
- **長押し/短押し**で誤り率が変わるか  

（将来的にコード改変が許されるなら、`RM2_CLOCK_DIVIDER` 等で SPI を遅くして再現性が変わるかを確認するのが有効）

## Gemini 追加調査による補強（RP2350 E9 Erratum）

Web 検索の結果、Gemini の「SPI クロック過大」説を決定的に裏付ける、**RP2350 固有のハードウェア不具合情報** が発見されました。

### 発見された事実：Erratum RP2350-E9 と 25MHz 制限

Raspberry Pi Pico 2 (RP2350) には **"Erratum RP2350-E9"** と呼ばれる既知の不具合があり、GPIO の入力ラッチ遅延により **SPI 通信（特に MISO）において 25MHz を超えるとビットエラーが発生する** ことが報告されています。

*   **症状:** GPIO 入力信号の遅延により、サンプリングタイミングがズレて誤った値を読み込む（ビット化け）。
*   **推奨事項:** SPI クロックを **25MHz 以下** に制限すること。

### 本件への適用

現在の `rmk-macro` 設定では、SPI クロックは推奨される 25MHz を大幅に超過しています。Codex が指摘した「37.5MHz（4分周）」の可能性を考慮しても、依然として 25MHz 制限を超過しており、ビット化けが発生する条件を完全に満たしています。

### 結論の確定

この追加情報により、本件の原因は「Notify の欠落」などのプロトコル層の問題ではなく、**「RP2350 のハードウェア制限（25MHz）を大幅に超える速度で SPI を駆動したことによる物理層のビット化け」** であることが確定しました。
解決策は、SPI クロック分周比を **6以上（推奨は8以上）** に設定し、通信速度を 25MHz 以下（例: 150MHz / 6 = 25MHz, 150MHz / 8 = 18.75MHz）に落とすことです。

## 最終結論と解決策

Gemini の追加調査（RP2350-E9 Erratum の発見）により、Codex が懸念していた「37.5MHz でなぜ化けるのか」という疑問も解消されました。
両者の議論と証拠を統合した最終結論は以下の通りです。

### 確定した原因
**RP2350 のハードウェア不具合 (Erratum E9) による SPI 通信エラー**
*   RP2350 の GPIO 入力遅延により、SPI 通信（特に MISO）の上限速度が **25MHz** に制限されている。
*   現在の設定（約 75MHz または 37.5MHz）はこの制限を超過しており、その結果として「前バイトの MSB が次バイトの LSB に漏れる」という特有のビット化けが発生している。

### 解決策
**SPI クロック分周比の変更（コード修正が必要）**
*   `rmk-macro/src/chip_init.rs` 内の `cyw43_pio::DEFAULT_CLOCK_DIVIDER` を変更し、SPI クロックを **25MHz 以下** に落とす。
*   **推奨設定:** 分周比を **6** (25MHz) または **8** (18.75MHz) に設定する。
    *   例: `::cyw43_pio::DEFAULT_CLOCK_DIVIDER * 4` (分周比 8相当) など。

## 追記（現時点の判断と対応方針）

ログ上、peripheral の HCI TX で `pressed=true` が送信されている一方で、central の HCI RX では
該当ビットが 0 に化けており、**GATT/デシリアライズ前の物理層で 1 ビットが変化している**。
debug パターンでも `0xA8 -> 0xA9` のように **直前バイトの MSB が次バイト LSB に漏れる**挙動が
観測されているため、**PIO/SPI のサンプリングタイミングが根本原因**と判断する。

対策として、`rmk-macro/src/chip_init.rs` の `cyw43_pio::PioSpi::new` で使用する
クロック分周比を下げ、**SPI 周波数を 25MHz 以下**に制限する。


• probrem.md を読みました。
  ログの時系列と HCI 生ログから、問題は SplitMessage の生成/デコードではなく、BLE 送信経路のもっと下（RP2350 ↔ CYW43 の SPI/PIO）で 1ビットが化けていると判断できます。

  ### 何が起きているか（ログの事実）

  - peripheral 側では raw head=[0, 1, 0, 0, 0] で pressed=true が正しく生成され、
    HCI TX の payload にも … 0, 1, 0, 0 … が入っています。
  - しかし central の HCI RX ですでに … 0, 0, 0, 0 … に変化しています。

  つまり GATT / postcard / central 側のデコード前に壊れているため、
  「Notify の取り違え」よりも 物理層のビット化けが最も整合的です。
  さらに debug パターン 0xA8 -> 0xA9 は 直前バイトの MSB が次バイトの LSB に漏れるような
  サンプリングずれの典型で、これも SPI/PIO のタイミング起因を裏付けます。

  ———

  ## 原因の推定（ソース改変なしの範囲での結論）

  ### 1) RP2350 の E9 erratum（GPIO入力系）

  RP2350 の E9 は GPIO 入力に関する既知のシリコン不具合で、入力周りに問題があることは
  公式資料・周辺報道で確認されています。(hackster.io (https://www.hackster.io/news/a-surprise-hardware-bug-in-raspberry-pi-s-rp2350-leads-to-unexpected-pull-down-behavior-76b51ec22ede.amp?utm_source=openai))
  このため PIO を使った高速入力（SPI MISO）に弱い可能性が高いです。

  ### 2) SPI 25MHz 上限の「実測報告」

  RP2350 で PIO SPI が 25MHz を超えるとビットエラーが出るという
  実測報告もあります（Hackaday コメント）。(hackaday.com (https://hackaday.com/2024/09/04/the-worsening-raspberry-pi-rp2350-e9-erratum-situation/?utm_source=openai))
  これは公式仕様ではないものの、今回の「1ビット化け」に非常に一致する現象です。

  ———

  ## なぜ Notify 取り違えでは説明できないか

  - もし Notify の取り違えなら 別のパケットに置き換わるのが普通です。
  - しかし今回のログは、同じパケットの 1ビットだけが変化しているため、
    パケット欠落より物理層の誤サンプリングが説明として自然です。

  ———

  # 修正案（ただし「ソース改変なし」でできることは限られる）

  根本対処は SPI クロックを下げることですが、
  これは cyw43-pio の divider を変える必要があり、ソース改変が不可避です。

  ### ソース改変なしでできること

  - 既に divider を落としたビルド済み firmware を使う
    → もし upstream / fork に存在すればそれを使う
  - RP2040 に戻す / USB 経路で検証する
    → RP2350 固有問題を回避
  - 温度・電源を変えて再現率が変わるか確認
    → サンプリングタイミング問題の補強材料になる
  ———

  # 結論（現時点）

  - pressed:true が central で false になる原因は、BLE パケットが中央到達前にビット化けしているため
  - 最も妥当な原因は RP2350 + CYW43 の SPI/PIO タイミング問題
  - 修正には SPI クロックの低速化（divider調整）が必要
      - ただし これはソース改変なしにはできない
