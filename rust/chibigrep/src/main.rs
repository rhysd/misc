use grep_regex::RegexMatcherBuilder;
use grep_searcher::{BinaryDetection, MmapChoice, Searcher, SearcherBuilder, Sink, SinkMatch};
use ignore::{self, WalkBuilder, WalkState};
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender};

type MyError = String;
type Result<T> = std::result::Result<T, MyError>;

struct Match {
    path: PathBuf,
    lnum: u64,
    line: Vec<u8>,
}

struct SearchSink<'a> {
    tx: &'a Sender<Result<Match>>,
    path: &'a Path,
}

// 結果を集めるためのコールバックを Sink で実装．マッチ箇所ごとに `matched` が呼ばれる
impl<'a> Sink for SearchSink<'a> {
    type Error = io::Error;

    // `SinkMatch` にマッチ情報が入っている
    fn matched(
        &mut self,
        _searcher: &Searcher,
        mat: &SinkMatch<'_>,
    ) -> std::result::Result<bool, Self::Error> {
        let m = Match {
            path: self.path.to_owned(),
            lnum: mat.line_number().unwrap_or(0),
            line: mat.bytes().to_vec(),
        };
        self.tx.send(Ok(m)).unwrap(); // マッチ結果を返す
        Ok(true)
    }
}

fn grep_file(pat: &str, path: PathBuf, tx: &Sender<Result<Match>>) {
    let mut builder = RegexMatcherBuilder::new();
    builder
        .case_smart(true) // smart case を有効に
        .unicode(true); // unicode 対応

    // Matcher を生成．今回は regex crate を使った RegexMatcher を使う．これ以外にも pcre2 を使ったものもある
    let matcher = match builder.build(pat) {
        Ok(m) => m,
        Err(err) => {
            tx.send(Err(format!("{}", err))).unwrap();
            return;
        }
    };

    // Searcher を生成
    let mut builder = SearcherBuilder::new();
    builder
        .binary_detection(BinaryDetection::quit(0)) // バイナリファイルだと判明したら検索をやめる
        .line_number(true)
        .memory_map(unsafe { MmapChoice::auto() }); // mmap を有効にする
    let mut searcher = builder.build();

    // ここでファイルを検索．マッチごとに sink の matched メソッドが呼ばれる
    let mut sink = SearchSink { tx, path: &path };
    if let Err(err) = searcher.search_path(&matcher, &path, &mut sink) {
        tx.send(Err(format!("{}", err))).unwrap();
    }
}

fn grep(pat: &str, path: &str, rest: impl Iterator<Item = String>) -> Result<Vec<Match>> {
    // Path を再帰的に辿る walker を生成．今回は WalkParallel でマルチスレッドでパスを辿る．
    // スレッドプールは自動で生成される（スレッド数は指定もできるが，デフォルトで良い感じに決めてくれる）
    let mut builder = WalkBuilder::new(path);
    for path in rest {
        builder.add(path);
    }
    builder
        .hidden(false) // 隠しファイルを検索
        .ignore(true) // ignore されたファイルを無視
        .parents(true); // 親ディレクトリを辿って .gitignore を探す
    let walker = builder.build_parallel();

    // walker.run はマルチスレッドで呼ばれるので値の受け渡しを channel でやる
    let (tx, rx) = mpsc::channel();

    walker.run(|| {
        // 初期化関数．ここはスレッドプールのスレッドごとに呼ばれる
        let tx = tx.clone();
        Box::new(move |result| match result {
            // この内側のコールバックはファイルパスごとに呼ばれる
            Ok(entry) if entry.file_type().map(|t| t.is_file()).unwrap_or(false) => {
                // `entry` は `ignore::DirEntry`
                grep_file(pat, entry.into_path(), &tx); // ファイルの時．ファイル内を検索
                WalkState::Continue // 検索を続ける
            }
            Ok(_) => WalkState::Continue, // ディレクトリの時．検索を続ける
            Err(err) => {
                tx.send(Err(format!("{}", err))).unwrap();
                WalkState::Quit // 検索を中止する
            }
        })
    });

    drop(tx);
    rx.into_iter().collect()
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let pat = args.next().expect("pattern argument was not given");
    let path = args.next().unwrap_or_else(|| ".".to_string());
    for mat in grep(&pat, &path, args)? {
        let path = mat.path.to_string_lossy();
        let line = String::from_utf8_lossy(&mat.line);
        print!("{}:{}: {}", path, mat.lnum, line);
    }
    Ok(())
}
