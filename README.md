# busmap

Rust で書かれた茨城交通のバス情報取得プログラムです。

## 概要

このプログラムは、茨城高専から勝田駅までのバス停間のバス情報を取得します。エントリーポイントは`src/main.rs`です。

作った理由は、遅延のせいで電車を逃しそうになって~~ムカついたからです~~ 。

## 使い方

1. プロジェクトをクローンします。

   ```
   git clone https://github.com/fukayatti0/busmap-nitic
   ```

2. プロジェクトディレクトリに移動します。

   ```
   cd busmap-nitic
   ```

3. 依存関係をインストールします。

   ```
   cargo build
   ```

4. アプリケーションを実行します。引数として高専から乗るときは `kosen` 勝田駅から乗るときは `station` を指定します。
   ```
   cargo run <kosen|station>
   ```
