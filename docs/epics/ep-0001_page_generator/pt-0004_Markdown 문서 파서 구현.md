# pt-0004 Markdown 문서 파서 구현

## 목표

Markdown 원문을 기본 문서 요소와 PAGE 블록 원문으로 구조화하고, Markdown 단계에서 판단할 수 있는 오류를 원본 위치와 함께 진단한다.

## 작업 내용

- Markdown AST 또는 이벤트 기반 파서를 선정한다.
- 제목, 문단, 표, 인용구, 목록, 링크, 이미지와 코드 등 MVP Markdown 요소를 구조화한다.
- `page:gantt` fenced block을 식별한다.
- raw HTML과 알 수 없는 `page:*` 블록을 진단한다.
- 첫 번째 level-1 제목으로 문서 제목을 결정한다. PAGE 블록 제목은 사용하지 않는다.
- 목록 항목이나 인용구 안의 `page:*` 블록을 `PAGE104`로 진단한다.

## 완료 조건

- [ ] 일반 Markdown 요소와 fenced block을 원문 순서 및 위치 정보와 함께 추출한다.
- [ ] PAGE 블록이 없는 일반 Markdown 문서가 정상적으로 파싱된다.
- [ ] 경계 및 오류 입력에 대한 단위 테스트가 통과한다.

## 결정 사항

- 파서는 원문을 pt-0003 문서 모델(`MarkdownBlock`, `Inline`)과 PAGE 블록 원문(`RawPageBlock`)으로 나눈다. YAML 해석은 pt-0005, HTML 생성은 pt-0007이 담당한다.
- 진단 메시지는 영어로 쓴다(`docs/cli.md`).
- Markdown 파서는 `pulldown-cmark`를 쓰고 실행 파일 크기를 최대한 줄인다. 기본 기능을 끄고 필요한 GFM 옵션만 켜며, workspace release profile에 크기 최적화를 적용한다.
- 위치 정보는 PAGE 블록 원문(`RawPageBlock`)에만 남긴다. Markdown 단계 오류는 파서가 읽는 즉시 위치와 함께 진단하므로 Markdown 블록은 위치 없이 pt-0003 모델을 그대로 쓴다.
- 진단 타입(`Diagnostic` 등)은 파서가 처음 쓰므로 pt-0004에서 design.md 계약대로 공개 타입으로 구현한다. 모든 단계의 진단을 모은 뒤에 하는 정렬과 100개 제한은 `validate`, `compile_html`을 만드는 pt-0007에서 구현한다.
- 링크 주소는 `SafeUrl::parse`로 검사해 `javascript:` 등 실행 가능한 주소를 `PAGE105`로 거부한다. 인터넷·메일·문서 안·상대 경로 링크는 허용한다.
- 이미지는 pt-0011의 공통 이미지 처리를 거친다. 파서는 이미지를 만나는 즉시 공통 처리를 호출하고, 실패하면 그 자리의 위치로 `PAGE106`을 진단한다. 그래서 이미지 참조의 위치를 따로 보존하지 않는다.
- fenced code 언어가 대소문자 구분 없이 `page:`로 시작하면 PAGE 블록 후보로 본다. 정확히 `page:gantt`만 Gantt이고 `Page:Gantt` 같은 나머지는 오타를 숨기지 않도록 `PAGE102`로 진단한다.
- 파서와 모델은 스타일을 정하지 않는다. 화면 표현은 pt-0007이 만드는 의미 class·`data-*` 속성과 built-in CSS 또는 사용자 CSS(`--style`)가 결정하고, 파서는 그에 필요한 의미 정보만 보존한다.
- 파서는 오류가 있어도 멈추지 않고 문서 끝까지 진단을 모은다. 오류가 난 요소는 결과에서 빼거나 내용만 남긴다.
- heading `id` 생성은 pt-0007 생성기가 담당한다. pt-0003 모델의 `Heading`에는 `id`가 없고, 생성기가 만드는 `page-data-NNNN` 등 다른 `id`와 함께 중복을 검사해야 하기 때문이다.

## 설계

### 모듈 구성

```text
crates/page/core/src/
├── diagnostic.rs     # Diagnostic, DiagnosticReport, DiagnosticCode, Severity, SourceSpan, SourcePosition
└── markdown/
    ├── mod.rs        # 하위 모듈 선언, parse·ParseOutput·ParsedDocument·ParsedBlock·RawPageBlock 재내보내기
    ├── parser.rs     # parse(): 이벤트를 블록 트리로 조립, PAGE fence 분류, 제목 추출, 진단
    └── position.rs   # LineIndex: byte offset을 line/column으로 변환
```

- 기존 `src/markdown.rs` 스텁은 위 디렉터리 모듈로 교체하고 `lib.rs`의 `pub mod markdown`을 `pub(crate) mod markdown`으로 바꾼다.
- `diagnostic.rs`의 타입은 공개 API 계약이므로 `pub`으로 두고 `lib.rs`에서 다시 내보낸다.
- 사용처는 pt-0005와 pt-0007에서 생긴다. 그 전까지 `markdown` 모듈에 임시 `dead_code` 허용을 두고 pt-0007 제거 항목에 추가한다.

### 파서 라이브러리

- `pulldown-cmark = { version = "0.13", default-features = false }`
  - 기본 기능 `getopts`(CLI 옵션 파싱)와 `html`(HTML 출력)은 쓰지 않는다. HTML은 pt-0007이 직접 만든다.
  - 남는 의존성은 `bitflags`, `memchr`, `unicase` 세 개다.
- 켜는 옵션은 GFM의 `ENABLE_TABLES`, `ENABLE_TASKLISTS`, `ENABLE_STRIKETHROUGH`뿐이다.
- 각주, heading 속성, smart punctuation, 수식, front matter 등 다른 확장은 켜지 않는다. 그 문법은 CommonMark 규칙대로 일반 텍스트로 남는다.
- `into_offset_iter()`로 이벤트마다 원문 byte 범위를 받아 위치를 계산한다.

#### 크기 측정 (2026-10-18, x86_64 Linux, 이벤트를 세는 최소 프로그램)

| 구성 | release 크기 |
|---|---|
| 의존성 없음, 기본 profile | 451 KiB |
| 의존성 없음, 크기 profile | 288 KiB |
| `pulldown-cmark` 기본 기능 켬, 기본 profile | 885 KiB |
| `pulldown-cmark` 기본 기능 끔, 기본 profile | 885 KiB |
| `pulldown-cmark` 기본 기능 끔, 크기 profile (`opt-level = "s"`) | 619 KiB |
| `pulldown-cmark` 기본 기능 끔, 크기 profile (`opt-level = "z"`) | **611 KiB** |
| `comrak` 0.56 기본 기능 끔, 크기 profile | 925 KiB |

- 크기 profile은 `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`다. 이것만으로 약 36% 줄어든다.
- `pulldown-cmark`가 더하는 크기는 약 323 KiB이고 `comrak`의 절반 수준이다.
- 기본 기능은 링크 단계에서 쓰지 않는 코드가 빠져 크기 차이가 없다. 그래도 끄면 `getopts`, `pulldown-cmark-escape`를 내려받거나 빌드하지 않는다.
- `panic = "abort"`는 release 빌드에만 적용된다. `cargo test`는 test profile을 쓰므로 영향이 없다.

### 스타일과의 경계

- 파서와 모델에는 색, 여백, 글꼴 같은 표현 정보를 넣지 않는다. pulldown-cmark의 HTML 출력도 쓰지 않는다.
- 렌더러가 의미 class와 `data-*` 속성을 붙일 수 있도록 다음 정보를 보존한다.

| 정보 | 모델 위치 | pt-0007 렌더링 예 |
|---|---|---|
| 제목 단계 | `HeadingLevel` | `<h2>` |
| 코드 언어 | `CodeBlock.language` | `class="language-rust"` |
| 표 열 정렬 | `TableCell.align` | `data-align="center"` |
| 체크박스 상태 | `ListItem.checked` | `class="page-task" data-checked="true"` |
| 번호 목록 시작값 | `List.start` | `<ol start="3">` |

- 렌더러는 inline `style` 속성을 쓰지 않는다. inline style은 사용자 CSS보다 우선해 override를 막기 때문이다. 표 정렬도 `style="text-align"` 대신 class나 `data-*`로 내보내고 CSS가 정렬한다. 이 규칙은 pt-0007 작업 항목에 기록한다.
- raw HTML(`<div style>`, `<style>`)은 `PAGE101`로 막는다. 사용자는 Markdown 안이 아니라 `--style` CSS 파일로 스타일을 바꾼다.

### 파서 결과

```rust
pub(crate) fn parse(source_name: &str, content: &str, assets: &dyn AssetResolver) -> ParseOutput;

pub(crate) struct ParseOutput {
    pub(crate) document: ParsedDocument,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

pub(crate) struct ParsedDocument {
    pub(crate) title: Option<Vec<Inline>>, // 첫 최상위 H1, blocks에서 제외
    pub(crate) blocks: Vec<ParsedBlock>,   // 원문 순서
}

pub(crate) enum ParsedBlock {
    Markdown(MarkdownBlock), // pt-0003 모델 그대로
    Page(RawPageBlock),
}

pub(crate) struct RawPageBlock {
    pub(crate) kind: PageBlockKind,
    pub(crate) body: String,               // fence 안의 원문
    pub(crate) span: SourceSpan,           // 여는 fence부터 닫는 fence까지
    pub(crate) body_start: SourcePosition, // body 첫 줄 위치, YAML 상대 위치 보정용
}

pub(crate) enum PageBlockKind { Gantt }
```

- Markdown 단계의 오류는 파서가 모두 진단하므로 Markdown 블록에는 위치가 더 필요 없다. 그래서 pt-0003 모델을 그대로 쓴다.
- PAGE 블록은 pt-0005가 YAML 오류를 원본 위치로 알려야 하므로 위치를 보존한다. YAML의 (줄 l, 칸 c)는 원본 (`body_start.line + l - 1`, `body_start.column + c - 1`)이다.
- pt-0005는 `RawPageBlock`을 `Gantt`로 바꾸고 `ParsedDocument`를 `PageDocument`로 조립한다.
- design.md의 "Markdown parser는 모든 node의 원문 span을 유지한다"는 문구를 이 결정에 맞게 고친다.

### 진단 타입

- design.md의 진단 계약을 그대로 구현한다: `Diagnostic`, `DiagnosticReport`, `DiagnosticCode`, `Severity`, `SourceSpan`, `SourcePosition`.
- `DiagnosticReport`는 `has_errors()`, `is_empty()`를 제공한다.
- 진단 정렬과 100개 제한(`PAGE001`)은 `validate`, `compile_html`을 만드는 pt-0007에서 구현한다.
- 위치는 1부터 센다. 칸은 줄 시작부터 센 Unicode 문자 수이고 탭도 한 칸이다. `end`는 포함하지 않는다.
- 파일 맨 앞의 UTF-8 BOM은 파싱과 칸 계산에서 제외한다.

### 변환 규칙

| 원문 | 결과 |
|---|---|
| 최상위 첫 `#` 또는 setext H1 | `title` (blocks에서 제외). 인용구·목록 안의 H1은 제목이 아니다 |
| 그 밖의 heading | `MarkdownBlock::Heading` |
| 문단 | `Paragraph` |
| 목록, 체크박스 목록 | `List`(`start`), `ListItem`(`checked`). tight 목록 항목의 inline은 `Paragraph`로 감싼다 |
| 인용구 | `BlockQuote` |
| fenced·indented code | `CodeBlock`. `language`는 info string의 첫 단어, 없으면 `None` |
| 표 | `Table`. 열 정렬을 각 셀의 `align`으로 복사한다 |
| `---` 등 | `ThematicBreak` |
| text, entity | `Text` (이웃한 text는 합친다) |
| `` `code` `` | `Inline::Code` |
| `*강조*`, `**강조**`, `~~취소~~` | `Emphasis`, `Strong`, `Strikethrough` |
| 링크, autolink | `Link`. 이메일 autolink는 `mailto:`를 붙여 `SafeUrl::parse`에 넘긴다 |
| 이미지 | pt-0011 공통 이미지 처리로 `DataImageUri`를 받아 `Image`. `alt`는 이미지 설명 inline의 평문 |
| soft/hard line break | `SoftBreak`, `HardBreak` |

### PAGE 블록 판별

- fenced code의 info string 첫 단어를 언어로 본다.
- 언어가 대소문자 구분 없이 `page:`로 시작하면 PAGE 블록 후보다.
  - 목록 항목이나 인용구 안에 있으면 `PAGE104`를 내고 버린다.
  - 최상위에서 정확히 `page:gantt`이면 `RawPageBlock { kind: Gantt }`가 된다.
  - 최상위의 그 밖의 후보(`page:chart`, `Page:Gantt` 등)는 `PAGE102`를 내고 버린다.
- `page:`로 시작하지 않는 언어는 일반 `CodeBlock`이다.
- body가 비어 있어도 그대로 넘긴다. 빈 Gantt 오류는 pt-0005가 진단한다.

### 진단 코드

| 코드 | 경우 | 처리 | 위치 |
|---|---|---|---|
| `PAGE101` | raw HTML 블록 또는 inline HTML | 버린다 | HTML 원문 범위 |
| `PAGE102` | 최상위의 알 수 없는 `page:*` fence | 버린다 | fence 전체 |
| `PAGE104` | 목록 항목·인용구 안의 `page:*` fence | 버린다 | fence 전체 |
| `PAGE105` | `SafeUrl::parse`가 거부한 링크 주소 | 링크를 풀고 내용 inline만 남긴다 | 링크 범위 |
| `PAGE106` | 공통 이미지 처리 실패 (pt-0011이 원인별 메시지 정의) | 버린다 | 이미지 범위 |

- 모든 진단은 `Severity::Error`이고 `source_name`과 `span`을 채운다. `block_kind`는 PAGE 블록 진단에 `"page:…"` 언어를 넣는다.

## 구현 체크리스트

### 준비

- [ ] `page-core`에 `pulldown-cmark`(기본 기능 끔)를 추가한다.
- [ ] workspace `Cargo.toml`에 크기 최적화 release profile을 추가하고 release 크기를 기록한다.
- [ ] `src/markdown.rs`를 `markdown/` 디렉터리 모듈로 바꾸고 `pub(crate) mod`로 선언한다.
- [ ] 임시 `dead_code` 허용을 두고 pt-0007 제거 항목에 추가한다.

### 진단과 위치

- [ ] `Diagnostic`, `DiagnosticReport`, `DiagnosticCode`, `Severity`, `SourceSpan`, `SourcePosition`을 구현하고 `lib.rs`에서 다시 내보낸다.
- [ ] `DiagnosticReport::has_errors`, `is_empty`를 구현한다.
- [ ] `LineIndex`로 byte offset을 1부터 시작하는 line/column으로 바꾼다. 칸은 Unicode 문자 수, CRLF와 BOM을 처리한다.
- [ ] 위치 테스트: 첫 줄, 여러 줄, 한글, 탭, CRLF, BOM, 줄 끝 offset

### Markdown 구조화

- [ ] 블록 이벤트를 `MarkdownBlock` 트리로 조립한다: heading, 문단, 목록, 체크박스, 인용구, code, 표, 구분선
- [ ] inline 이벤트를 `Inline`으로 조립한다: text 병합, code, 강조, 취소선, 링크, 이미지, 줄바꿈
- [ ] 첫 최상위 H1을 `title`로 꺼낸다.
- [ ] 이미지는 pt-0011 공통 이미지 처리를 호출해 `DataImageUri`를 받는다.
- [ ] 구조화 테스트: 요소별 변환, 중첩 목록과 인용구, tight 목록, 표 정렬, 이메일 autolink, 메모리 resolver로 이미지 변환, H1 없음, 두 번째 H1, 인용구 안 H1

### PAGE 블록과 진단

- [ ] `page:*` fence를 판별해 `RawPageBlock`을 만들고 `span`, `body_start`를 채운다.
- [ ] `PAGE101`, `PAGE102`, `PAGE104`, `PAGE105`, `PAGE106`을 진단하고 오류 요소를 버리거나 내용만 남긴다.
- [ ] PAGE 블록 테스트: 여러 `page:gantt`와 Markdown의 원문 순서, 들여쓴 fence의 `body_start`, 빈 body, 닫히지 않은 fence
- [ ] 진단 테스트: 코드별 입력과 위치, 한 문서의 여러 오류 수집, PAGE 블록 없는 문서는 진단 없음

### 마무리

- [ ] design.md의 파서 span 문구와 진단 코드 표를 갱신한다.
- [ ] pt-0007에 inline `style` 금지, 표 정렬 `data-align` 출력, 진단 정렬과 100개 제한 항목을 추가한다.
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets`, `cargo test --workspace`가 통과한다.
- [ ] 완료 조건과 `plan.md`의 pt-0004 체크리스트를 갱신하고 검증 결과를 이 문서에 기록한다.

## 검증

- `cargo test -p page-core`

### 진행 기록

- Q5 결정 (2026-10-18): `page:` 접두어는 대소문자 구분 없이 PAGE 블록 후보로 보고, 정확히 `page:gantt`가 아니면 `PAGE102`로 진단한다. 결정 필요 항목을 모두 정리했다.
- Q4 결정 (2026-10-18): 위험한 링크 주소는 `PAGE105`, 이미지 처리 실패는 `PAGE106`이다. `PAGE103`은 예전 "Gantt 둘 이상" 코드라 다시 쓰지 않는다. 이미지는 출처와 관계없이 공통 인터페이스로 처리하기로 하고 pt-0011로 분리했다. 파서가 이미지를 만나는 즉시 공통 처리를 호출하므로 Q2 결정(PAGE 블록만 위치 보존)은 그대로다.
- Q3 결정 (2026-10-18): 진단 타입과 `has_errors`, `is_empty`는 pt-0004에서 공개 타입으로 구현하고, 진단 정렬과 100개 제한은 pt-0007에서 구현한다.
- Q2 결정 (2026-10-18): 위치 정보는 PAGE 블록 원문에만 남기고 Markdown 블록은 위치 없이 pt-0003 모델을 쓴다. design.md의 "모든 node의 원문 span 유지" 문구는 마무리 단계에서 고친다.
- Q1 결정 (2026-10-18): `pulldown-cmark`를 쓰고 크기를 최대한 줄인다. 기본 기능을 끄고 크기 release profile을 추가한다. 측정 결과 `comrak`보다 약 314 KiB 작다. 스타일은 built-in 또는 사용자 CSS가 정하도록 파서는 의미 정보만 보존하고 렌더러는 inline style을 쓰지 않는다.
- 설계 초안 (2026-10-18): 목표, 모듈 구성, 파서 결과 타입, 변환 규칙, PAGE 블록 판별, 진단 코드와 구현 체크리스트를 작성했다. Q1–Q5는 결정 필요로 남겼다.

## 선행 작업

- pt-0002, pt-0003, pt-0011

## 후속 작업

- pt-0005
