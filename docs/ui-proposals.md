# git-all-status TUI UI案

全ローカルリポジトリの未コミット状況を一括把握するRatatui TUI。
**前提**: TUI必須、多数リポジトリ（50〜200+）対象。

## 決定

**案B: マスター・ディテール（2ペイン）** を採用。lazygit/k9s風の左一覧+右詳細レイアウト。

---

## 案A: フラットテーブル

htop風の密な一覧テーブル。1リポジトリ=1行。スクロール・ソート・フィルタで大量リポジトリを捌く。

```
 git-all-status                              4/87 dirty  ⟳ 5s
 ─────────────────────────────────────────────────────────────────
  Repository        Branch   St  S  U  ?  ↑  ↓  Last commit
 ─────────────────────────────────────────────────────────────────
  my-app            main     ✗   2  3  1  ·  ·  5m ago
  api-server        dev      ✓   ·  ·  ·  2  ·  2h ago
  dotfiles          main     ✗   ·  1  ·  ·  ·  1d ago
  website           main     ✓   ·  ·  ·  ·  ·  3d ago
  auth-service      main     ✗   1  ·  ·  ·  ·  10m ago
  mobile-app        feature  ✗   ·  5  2  1  3  30m ago
  ...
 ─────────────────────────────────────────────────────────────────
  [/] search  [s] sort  [f] dirty only  [Enter] detail  [q] quit
```

**長所**: 情報密度最大、スクロールだけで全体把握、ソート切替で優先度判断しやすい
**短所**: 各リポジトリの変更ファイル詳細は別画面が必要
**参考**: htop, lazydocker の一覧画面

---

## 案B: マスター・ディテール（2ペイン）

左にリポジトリ一覧、右に選択中リポジトリの詳細。一覧と詳細を同時に見られる。

```
 git-all-status                                         ⟳ 5s
 ──────────────────────────┬──────────────────────────────────
  Repository     St  S U ? │  my-app  [main]  ✗ Dirty
 ──────────────────────────│
 ▶ my-app        ✗  2 3 1 │  Staged (2):
   api-server    ✓  · · · │    M src/main.rs
   dotfiles      ✗  · 1 · │    A src/config.rs
   website       ✓  · · · │
   auth-service  ✗  1 · · │  Unstaged (3):
   mobile-app    ✗  · 5 2 │    M README.md
   cli-tool      ✓  · · · │    M Cargo.toml
   infra-tf      ✗  · 2 · │    M src/lib.rs
   blog          ✓  · · · │
   sdk-python    ✓  · · · │  Untracked (1):
   monorepo      ✗  3 7 4 │    ? tmp/debug.log
   ...           ...       │
                           │  Last commit: 5 min ago
                           │  "fix: resolve config parsing"
 ──────────────────────────┴──────────────────────────────────
  4/12 dirty │ [Tab] pane │ [f] filter │ [Enter] lazygit
```

**長所**: 一覧性と詳細の両立、ペイン比率を端末幅に応じて調整可能
**短所**: 横幅が狭い端末では窮屈、ペイン操作がやや複雑
**参考**: lazygit, k9s のリスト+詳細レイアウト

---

## 案C: グループ化ツリー

ディレクトリ構造やタグでリポジトリをグループ化し、ツリー表示。折りたたみ可能。

```
 git-all-status                              4/87 dirty  ⟳ 5s
 ─────────────────────────────────────────────────────────────────
  ▼ C:\Main\Project  (3 dirty / 8 repos)
     my-app            main     ✗   S:2 U:3 ?:1     5m ago
     api-server        dev      ✓   ↑2               2h ago
     website           main     ✓                     3d ago
     auth-service      main     ✗   S:1               10m ago
     mobile-app        feature  ✗   U:5 ?:2  ↑1 ↓3   30m ago
     cli-tool          main     ✓                     1w ago
     sdk-python        main     ✓                     2d ago
     monorepo          main     ✗   S:3 U:7 ?:4       15m ago

  ▶ C:\Main\Work  (1 dirty / 4 repos)

  ▶ C:\Main\OSS  (0 dirty / 3 repos)

 ─────────────────────────────────────────────────────────────────
  [Space] toggle group  [f] dirty only  [Enter] detail  [q] quit
```

**長所**: 大量リポジトリを論理的にまとめて把握、グループ単位で折りたたみ可能
**短所**: ツリー展開状態の管理が必要、全展開すると案Aと大差なし
**参考**: ファイルマネージャのツリービュー, VS Code Explorer

---

## 案D: ダッシュボード（サマリー + テーブル）

画面上部にサマリー統計、下部にテーブル一覧。全体の健全性を一目で把握。

```
 git-all-status                                              ⟳ 5s
 ─────────────────────────────────────────────────────────────────
  Total: 87    Clean: 83    Dirty: 4    Ahead: 3    Behind: 1

  ██████████████████████████████████████████████░░  95% clean
 ─────────────────────────────────────────────────────────────────
  ▼ Dirty (4)
  Repository        Branch   St  S  U  ?  ↑  ↓  Last commit
  my-app            main     ✗   2  3  1  ·  ·  5m ago
  dotfiles          main     ✗   ·  1  ·  ·  ·  1d ago
  auth-service      main     ✗   1  ·  ·  ·  ·  10m ago
  monorepo          main     ✗   3  7  4  ·  ·  15m ago

  ▶ Clean (83)
 ─────────────────────────────────────────────────────────────────
  [Tab] section  [f] show all  [Enter] detail  [q] quit
```

**長所**: サマリーで全体状況を即座に把握、dirty優先表示で注意すべきリポジトリに集中
**短所**: サマリー領域がテーブルの表示行数を圧迫
**参考**: Grafanaダッシュボード, GitHub Actions summary

---

## 案E: ハイブリッド（A + B の切替）

デフォルトは案Aのフラットテーブル。Enterでインラインに詳細を展開（案Bのディテールをテーブル内に埋め込む）。

```
 git-all-status                              4/87 dirty  ⟳ 5s
 ─────────────────────────────────────────────────────────────────
  Repository        Branch   St  S  U  ?  ↑  ↓  Last commit
 ─────────────────────────────────────────────────────────────────
  my-app            main     ✗   2  3  1  ·  ·  5m ago
  ┆  Staged:    M src/main.rs, A src/config.rs
  ┆  Unstaged:  M README.md, M Cargo.toml, M src/lib.rs
  ┆  Untracked: ? tmp/debug.log
  api-server        dev      ✓   ·  ·  ·  2  ·  2h ago
  dotfiles          main     ✗   ·  1  ·  ·  ·  1d ago
  website           main     ✓   ·  ·  ·  ·  ·  3d ago
  auth-service      main     ✗   1  ·  ·  ·  ·  10m ago
  mobile-app        feature  ✗   ·  5  2  1  3  30m ago
  ...
 ─────────────────────────────────────────────────────────────────
  [Enter] toggle detail  [s] sort  [f] dirty only  [q] quit
```

**長所**: 普段は密な一覧、必要な時だけ詳細展開。両方のメリットを享受
**短所**: 展開時にスクロール位置がずれる、展開行の描画がやや複雑
**参考**: Thunderbirdのメールスレッド展開、ファイルマネージャの詳細表示

---

## 比較表

| 項目                 | A: テーブル | B: 2ペイン | C: ツリー | D: ダッシュ | E: ハイブリッド |
|----------------------|:---------:|:---------:|:--------:|:----------:|:-------------:|
| 情報密度             | ◎        | ○        | ○       | ○         | ◎            |
| 詳細の確認しやすさ   | △        | ◎        | △       | △         | ○            |
| 大量リポジトリ対応   | ◎        | ◎        | ◎       | ◎         | ◎            |
| 全体状況の把握       | ○        | ○        | ◎       | ◎         | ○            |
| 実装コスト           | 低        | 中        | 中       | 中         | 中            |
| 操作のシンプルさ     | ◎        | ○        | ○       | ○         | ◎            |
| 常設モニター映え     | ○        | ◎        | ○       | ◎         | ○            |

---

## 補足: 組み合わせの可能性

これらは排他ではなく、要素を混ぜることもできる:

- **D + B**: ダッシュボードサマリー上部 + 下部で2ペインのマスター・ディテール
- **C + E**: ツリーグループ化 + インライン展開
- **A + D**: テーブル上部にサマリーバーだけ追加（1行で `87 repos │ 4 dirty │ 3 ahead`）
