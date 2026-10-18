# ep-0001 Rust 문서 생성기 설계

## 설계 목표

Markdown의 `page:gantt` 블록을 확장 가능한 중간 모델로 변환하고, 독립 실행 가능한 단일 HTML에 포함한다. 입력 문법은 [PAGE 입력 문법](../../syntax.md), CLI 계약은 [PAGE CLI 계약](../../cli.md)을 따른다.

생성기는 기본 HTML 셸, CSS와 JavaScript runtime을 컴파일 시점에 내장한 단일 경량 CLI로 제공한다. 기본 실행은 외부 runtime 디렉터리, 설정 파일과 네트워크를 요구하지 않는다.

## 경량 배포 구조

```text
개발 소스
├── shell.html
├── styles/*.css
└── scripts/*.js
        ↓ 컴파일 시점 내장
page 실행 파일 하나
        ↓ page document.md
document.html 하나
```

- 텍스트 runtime 자산은 우선 Rust의 `include_str!`로 내장한다.
- 단순한 자산 내장을 위해 추가 resource framework를 사용하지 않는다.
- 사용자 CSS만 `--style` 지정 시 실행 시점에 읽고 결과 HTML에 인라인한다.
- PDF 브라우저 엔진은 번들하지 않고 로컬 Chromium 계열 브라우저를 사용한다.
- release profile은 크기 최적화, LTO와 symbol 제거를 적용한다.
- 완료 검증에서 release 바이너리 크기와 주요 의존성을 기록하되 근거 없는 고정 용량 제한은 두지 않는다.

## 변환기 계층

변환기는 일반 Markdown 문서 구조와 PAGE 전용 블록을 분리하되 하나의 문서 순서 안에서 함께 다룬다.

```text
Markdown 변환기
├── 기본 Markdown 요소
│   ├── 제목
│   ├── 문단
│   ├── 표
│   ├── 인용구
│   ├── 목록
│   └── 코드 블록
└── PAGE 블록 확장
    ├── page:gantt
    │   ├── 제목
    │   ├── milestone
    │   ├── epic
    │   └── task
    ├── page:chart (향후)
    └── page:report (향후)
```

- 문서 파서는 Markdown 구조와 fenced block을 식별한다.
- PAGE 블록 파서는 식별된 블록을 종류별 모델로 역직렬화하고 검증한다.
- 문서 렌더러는 문서 순서를 유지하면서 기본 요소와 PAGE 블록을 종류별 렌더러에 위임한다.
- 새 PAGE 형식은 기존 Gantt 모델을 변경하지 않고 별도 parser, model과 renderer로 추가한다.
- 공통 추상화는 소스 위치와 진단 정보에 한정한다. 제목과 task처럼 의미가 다른 필드를 하나의 범용 구성 요소로 합치지 않는다.
- 최초 구현은 타입 안전한 enum 분기를 사용하며 동적 플러그인 구조는 현재 범위에 포함하지 않는다.

## 스타일 계층

중간 모델, HTML 구조와 CSS 스타일을 서로 분리한다. renderer는 의미 있는 class와 `data-*` 속성을 출력하고 시각 표현을 직접 소유하지 않는다.

기본 색상, 타이포그래피, 간격, Markdown/Gantt 구성 요소와 인쇄 규칙은 [PAGE 기본 스타일 계약](../../style.md)을 따른다. 기본 테마는 선택 옵션이 아니라 모든 PAGE HTML에 포함되는 필수 자산이다.

```text
runtime/
├── shell.html
├── styles/
│   ├── base.css
│   ├── markdown.css
│   ├── gantt.css
│   ├── theme-default.css
│   └── print.css
└── scripts/
    └── gantt.js
```

- 공통 레이아웃, Markdown 요소, Gantt 구성 요소, 테마와 인쇄 스타일을 별도 CSS로 관리한다.
- 일반 Markdown HTML에는 Gantt 존재 여부와 관계없이 base, Markdown, 기본 테마와 인쇄 CSS를 적용한다.
- 모든 생성 HTML은 공통 문서 canvas, header, content와 선택적 footer를 갖는 PAGE 문서 셸을 사용한다.
- Gantt component는 문서 셸 내부에 배치되며 전역 배경, 글꼴과 콘텐츠 폭을 소유하지 않는다.
- 테마의 색상, 글꼴과 크기는 CSS custom property로 정의한다.
- `Gantt`, `Epic`, `Milestone`과 `Task` 모델에는 CSS 문자열이나 구체적인 색상값을 저장하지 않는다.
- 상태와 종류는 class 또는 `data-*` 속성으로 노출하고 CSS가 실제 표현을 결정한다.
- 새 PAGE 블록은 독립적인 component CSS를 추가할 수 있다.
- 개발 시 runtime 자산은 분리하되 `pt-0007`에서 HTML, CSS와 JavaScript를 인라인하여 단일 PAGE HTML로 생성한다.
- 기존 프로토타입의 시각 기준을 기본값으로 사용하고 상태 색상, 문서 요소와 접근성 규칙은 기본 스타일 계약으로 보완한다.
- `--style` 사용자 CSS는 layered 기본 bundle 뒤의 별도 unlayered style 요소에 지정 순서대로 인라인한다.
- 기본 stylesheet 전체를 항상 포함하므로 부분 사용자 CSS에서 선언하지 않은 값은 PAGE 기본 스타일로 유지된다.

## 컴파일 흐름

```text
Markdown 원문
→ Markdown 구조 분석
→ 기본 Markdown 요소와 PAGE fenced block 식별
→ PAGE 블록 종류별 역직렬화
→ 검증·정규화
→ 문서 중간 모델
→ 요소 종류별 HTML 렌더링
→ PAGE 런타임이 포함된 단일 HTML
  ├→ HTML 파일
  └→ 브라우저 인쇄 엔진 → PDF 파일
```

파서와 HTML 생성기는 중간 모델만 공유한다. CLI는 파일 입출력만 담당하고 컴파일 규칙을 소유하지 않는다.

`--dry-run`은 검증·정규화 단계까지만 실행한다. PDF는 동일한 HTML 산출물을 로컬 브라우저 인쇄 엔진으로 변환해 HTML과 시각적 기준을 공유한다.

## `page-core` 공개 API

`page-core`는 문자열 기반 변환과 진단만 담당한다. 경로 해석, 파일 읽기·쓰기, 덮어쓰기, PDF 변환과 프로세스 종료 코드는 `page-cli`의 책임이다.

```rust
pub struct SourceInput<'a> {
    pub name: &'a str,
    pub content: &'a str,
}

pub struct StyleInput<'a> {
    pub name: &'a str,
    pub content: &'a str,
}

#[derive(Default)]
pub struct CompileOptions<'a> {
    pub custom_styles: &'a [StyleInput<'a>],
}

pub struct CompileOutput {
    pub html: String,
    pub diagnostics: DiagnosticReport,
}

pub fn validate(
    source: SourceInput<'_>,
    options: &CompileOptions<'_>,
) -> DiagnosticReport;

pub fn compile_html(
    source: SourceInput<'_>,
    options: &CompileOptions<'_>,
) -> Result<CompileOutput, DiagnosticReport>;
```

- `name`은 진단 표시에만 사용하며 생성 HTML에는 기록하지 않는다.
- `StyleInput`은 파일 경로가 아니라 CLI가 읽은 논리적 이름과 UTF-8 CSS 내용이다.
- `CompileOptions`에는 출력 경로, `force`, PDF 형식과 `dry-run`을 넣지 않는다.
- `validate`는 HTML 렌더링과 runtime 조립 없이 Markdown, PAGE 데이터, 모델과 사용자 CSS를 검사한다.
- `compile_html`은 동일한 내부 분석 함수를 한 번 실행하고 오류가 없을 때만 HTML을 렌더링한다. CLI가 먼저 `validate`를 호출한 뒤 다시 컴파일할 필요는 없다.
- 성공한 `CompileOutput.diagnostics`에는 경고만 존재할 수 있다. 실패한 `DiagnosticReport`에는 발견된 오류와 함께 관련 경고가 포함될 수 있다.

진단 타입의 계약은 다음과 같다.

```rust
pub struct DiagnosticReport {
    pub diagnostics: Vec<Diagnostic>,
}

pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub message: String,
    pub source_name: String,
    pub span: Option<SourceSpan>,
    pub block_kind: Option<String>,
    pub item_id: Option<String>,
}

pub struct DiagnosticCode(&'static str);

pub enum Severity {
    Warning,
    Error,
}

pub struct SourceSpan {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}
```

line과 column은 사용자 표시와 동일한 1부터 시작하는 값이며 `end`는 exclusive이다. `DiagnosticCode`는 고정된 ASCII 코드의 newtype이며 문자열 조회 동작을 제공한다. `DiagnosticReport`는 최소한 `has_errors()`와 `is_empty()` 동작을 제공한다. 구체적인 Rust 필드 공개 범위와 문자열 소유 최적화는 구현 task에서 조정할 수 있으나 의미 계약은 유지한다.

## 파이프라인 단계와 책임

| 단계 | 내부 입력 → 출력 | 담당 모듈 | 책임 |
|---|---|---|---|
| 소스 준비 | `SourceInput`, `StyleInput[]` | `page-cli` | UTF-8 파일 읽기와 논리적 이름 부여 |
| Markdown 구조 분석 | `SourceInput` → `ParsedDocument` | `markdown` | 기본 Markdown node와 `RawPageBlock` 식별, span 보존 |
| PAGE 역직렬화 | `RawPageBlock` → block 입력 DTO | `page::parse` → `gantt::parse` | 종류별 parser dispatch, YAML 문법, 필드와 스키마 버전 해석 |
| 정규화 | block 입력 DTO → 정규화 후보 | `page::normalize` → `gantt::normalize` | 공백 제거, 기본 상태·버전, 날짜와 자동 ID 결정 |
| 모델 검증 | 정규화 후보 → `PageDocument` | `document::validate` → 종류별 validator | 문서 전체 개수·ID와 날짜 순서, 계층 등 block 제약 검사 |
| 본문 렌더링 | `PageDocument` → `RenderedBody` | `generator::render` | Markdown 의미 HTML과 PAGE component markup 생성 |
| 문서 조립 | body, data, 내장 runtime, CSS → `String` | `generator::assemble` | 안전한 삽입, 고정 순서 인라인과 최종 LF 정규화 |
| 출력 및 PDF | HTML → 파일 또는 PDF | `page-cli` | 원자적 파일 교체와 로컬 브라우저 실행 |

`ParsedDocument`, `RawPageBlock`, 입력 DTO와 `RenderedBody`는 crate 내부 타입으로 두고 외부 API에 노출하지 않는다. `PageDocument`와 Gantt 도메인 타입의 공개 범위는 `pt-0003`에서 결정한다.

- Markdown parser는 모든 node의 원문 span을 유지한다.
- YAML parser가 제공하는 상대 위치는 fenced block의 시작 위치와 합쳐 원본 Markdown 위치로 변환한다.
- 명시 ID의 형식과 중복을 먼저 검사하고 문서 전체의 명시 ID를 예약한 뒤 깊이 우선 입력 순서로 자동 ID를 할당한다.
- `version`과 `status` 기본값, 이름 공백 제거와 날짜 구조화는 정규화 단계가 담당한다.
- 검증 단계는 정규화된 값을 변경하지 않는다.
- `--dry-run`은 사용자 CSS 안전성 검사와 `PageDocument` 검증까지 실행하고 body 렌더링, runtime 조립, 브라우저 탐지와 파일 쓰기를 생략한다.

## 일반 Markdown 계약

- 기본 Markdown과 GFM table, task list, strikethrough를 지원한다.
- 첫 번째 level-1 heading은 문서 제목이며 원래 inline 표현을 유지해 header에 한 번만 렌더링한다. HTML `<title>`에는 inline 요소를 plain text로 평탄화한 값을 사용한다.
- level-1 heading이 없으면 `PAGE Document`를 문서 제목으로 사용한다. PAGE 블록의 제목은 문서 제목에 사용하지 않고 각 component 안에 렌더링한다.
- 일반 code fence와 알 수 없는 비 PAGE 언어는 언어 class를 가진 code block으로 렌더링한다.
- 알 수 없는 `page:*` fence는 오류로 처리한다.
- 모든 heading에는 제목 텍스트로 만든 `id`를 붙여 문서 내 fragment 링크의 대상이 되게 한다.
  - inline 요소를 plain text로 평탄화하고 영문자는 소문자로 바꾼다.
  - 문자(한글 포함), 숫자, `-`, `_`는 유지하고 공백은 `-`로 바꾸며 그 밖의 문장 부호는 제거한다.
  - 결과가 비면 `section`을 사용한다.
  - 이미 사용된 `id`이면 문서 순서대로 `-1`, `-2`를 붙인다. 생성기가 만드는 `page-data-NNNN` 등 다른 `id`와도 겹치지 않게 같은 사용 목록으로 검사한다.
- PAGE block은 문서 최상위에만 둘 수 있다. 목록 항목이나 인용구 안의 `page:*` fence는 오류로 처리한다.
- raw HTML은 오류로 처리하며 그대로 통과시키거나 자동 sanitization하지 않는다.
- 링크는 `http`, `https`, `mailto`, 문서 내 fragment와 상대 경로를 허용하고 `javascript`, `data` 등 실행 가능한 scheme은 거부한다.
- 이미지는 아직 구현하지 않는다. 주소 종류와 관계없이 `Image not supported yet` 문구와 alt를 담은 빈 박스로 렌더링하고 진단은 내지 않는다. 이미지를 단일 HTML에 넣는 처리는 backlog에서 설계한다.
- 문서는 `page:gantt`를 0개 이상 포함할 수 있다. 없는 문서는 일반 Markdown 문서로 생성하고, 여러 개인 문서는 각 블록을 독립된 Gantt component로 생성한다.
- Markdown block과 Gantt component는 원문 순서대로 content 영역에 배치한다. 문서 제목으로 소비한 첫 level-1 heading만 header로 이동한다.

## 중간 모델

```text
PageDocument
└── DocumentBlock[]
    ├── MarkdownBlock
    │   ├── Heading
    │   ├── Paragraph
    │   ├── Table
    │   ├── BlockQuote
    │   ├── List
    │   └── CodeBlock
    └── PageBlock::Gantt
        └── Gantt
            └── GanttItem[]
                ├── Task
                ├── Epic
                │   └── Task[]
                └── Milestone
                    └── (Epic | Task)[]
```

- 모든 문서 블록은 원문 순서를 보존한다.
- 소스 위치는 진단을 만드는 파싱·정규화 단계의 내부 타입이 보존하고 중간 모델에는 두지 않는다.
- 일반 Markdown과 PAGE 블록의 모델 및 렌더링 책임은 서로 분리한다.
- 이번 Epic은 정의된 기본 Markdown 요소를 실제 HTML로 렌더링한다. raw HTML, `page:chart`, `page:report`와 Mermaid는 후속 epic에서 다룬다.

- 차트는 task만으로 구성할 수 있다.
- Epic과 milestone은 선택적인 그룹이다.
- Epic은 task만, milestone은 task 또는 epic을 포함한다.
- milestone, epic과 task를 나타내는 ID는 문서 전체에서 고유하다.
- 입력 날짜는 검증 후 `DateRange`로 정규화한다.
- task 상태는 `TaskStatus` enum으로 표현한다.

## 진행률과 일정 파생

Epic과 milestone의 진행률과 일정 범위는 저장하지 않는다. Gantt runtime JS만 Gantt JSON에서 계산하며 Rust 중간 모델은 계산하지 않는다.

```text
Epic 진행률 = Epic의 완료 task 수 / Epic의 전체 task 수 × 100
Milestone 진행률 = Milestone 하위의 완료 task 수 / Milestone 하위의 전체 task 수 × 100
```

- `completed` 상태만 완료 수에 포함한다.
- 진행률은 정수 백분율로 반올림한다.
- 그룹 일정은 모든 하위 task의 최소 시작일과 최대 종료일이다.
- 그룹이 없는 평면 차트에는 그룹 진행률을 표시하지 않는다.

## 직렬화와 확장

- 입력 스키마에 없는 필드는 오류로 처리한다.
- 자동 ID는 항목 종류와 입력 순서로 결정적으로 생성한다.
- 파생 진행률은 JSON에 저장하지 않고 런타임에서 계산한다.
- 새 문서 유형은 `PageBlock` variant로 추가한다.
- Gantt 확장은 기존 필드의 의미를 바꾸지 않는 선택 필드나 별도 타입으로 추가한다.

## 오류와 진단

진단 코드는 계층별로 분리한다.

| 영역 | 코드 범위 | 예시 |
|---|---|---|
| 공통 보고 제한 | `PAGE0xx` | 최대 진단 수 초과 |
| Markdown·PAGE 입력 문법 | `PAGE1xx` | raw HTML, 알 수 없는 PAGE block, YAML 구문 |
| 정규화·모델 검증 | `PAGE2xx` | 필수 필드, ID, 날짜, 상태와 계층 오류 |
| HTML·CSS·PDF 산출물 생성 | `PAGE3xx` | 금지 CSS, template marker, 직렬화와 브라우저 오류 |
| CLI 파일 I/O | `PAGE4xx` | 입력·CSS 읽기, 출력 경로와 원자적 쓰기 오류 |

- 입력 Markdown 진단을 먼저, 사용자 CSS 진단을 `--style` 지정 순서대로 배치한다.
- 같은 source 안에서는 시작 line, column, severity, code 순서로 안정적으로 정렬하며 같은 위치에서는 error를 warning보다 먼저 둔다.
- 오류와 경고가 100개를 넘으면 앞의 99개와 생략 사실을 알리는 마지막 진단 하나만 유지한다.
- Markdown 구조 분석이 불가능한 치명 오류는 해당 source의 후속 분석을 중단한다.
- Gantt YAML 구문 오류는 해당 block의 역직렬화만 중단하고 이미 발견한 독립적인 Markdown 진단은 유지한다.
- 모델 검증은 서로 독립적으로 확인할 수 있는 오류를 가능한 범위에서 모두 수집한다.
- template 무결성이나 HTML 조립 오류는 내부 생성 오류로 즉시 중단한다.
- `page-cli`는 입력 또는 CSS 파일을 읽을 수 없으면 core를 호출하지 않고 실패한다.
- `page-core`는 전달받은 CSS 내용의 금지 구문을 `PAGE3xx`로 진단한다. CSS 전체 문법의 유효성은 별도 parser를 추가하지 않고 브라우저 표준 처리에 맡긴다.
- 진단에는 가능한 경우 source name과 span을 반드시 포함하고 PAGE block 종류와 Gantt item ID는 식별 가능한 단계부터 추가한다.

대표 코드의 최초 할당은 다음을 기준으로 하며 세부 코드는 해당 구현 task에서 이 범위 안에 추가한다.

| 코드 | 의미 |
|---|---|
| `PAGE001` | 추가 진단이 생략됨 |
| `PAGE101` | raw HTML은 지원하지 않음 |
| `PAGE102` | 알 수 없는 `page:*` block |
| `PAGE104` | 목록 항목이나 인용구 안에 놓인 `page:*` block |
| `PAGE120` | Gantt YAML 구문 또는 금지 YAML 기능 |
| `PAGE201` | 필수·알 수 없는 필드 또는 지원하지 않는 값 |
| `PAGE202` | 날짜 형식 또는 날짜 범위 오류 |
| `PAGE203` | ID 형식 또는 중복 오류 |
| `PAGE204` | 허용하지 않는 Gantt 계층 |
| `PAGE301` | runtime template 또는 marker 무결성 오류 |
| `PAGE302` | 금지된 사용자 CSS 내용 |
| `PAGE303` | HTML 데이터 직렬화 또는 조립 오류 |
| `PAGE320` | PDF용 Chromium 계열 브라우저를 찾지 못함 |
| `PAGE321` | 브라우저 실행 또는 PDF 결과 검증 실패 |
| `PAGE401` | Markdown 입력 파일 읽기 또는 UTF-8 오류 |
| `PAGE402` | 사용자 CSS 파일 읽기 또는 UTF-8 오류 |
| `PAGE403` | 출력 경로, 확장자 또는 기존 파일 정책 오류 |
| `PAGE404` | 임시 파일 쓰기 또는 최종 파일 교체 오류 |

## 내장 리소스와 HTML 조립

`page-core`가 소유하고 컴파일 시점에 내장할 MVP runtime 자산은 다음과 같다.

```text
runtime/
├── shell.html
├── styles/
│   ├── base.css
│   ├── markdown.css
│   ├── gantt.css
│   ├── theme-default.css
│   └── print.css
└── scripts/
    └── gantt.js
```

- `page-core::runtime` 모듈이 `include_str!` 상수와 고정 자산 순서를 소유한다.
- `page-cli`는 내장 runtime 경로나 설치 위치를 알지 못한다.
- `base.css`, `markdown.css`, `theme-default.css`와 `print.css`는 모든 문서에 포함한다.
- `gantt.css`와 `gantt.js`는 문서에 Gantt block이 하나 이상 있을 때 한 번만 포함한다. Gantt JSON data block은 Gantt block마다 하나씩 생성한다. 향후 component도 실제 사용된 종류의 자산만 첫 등장 기준의 고정 순서로 포함한다.
- `shell.html`에는 `__PAGE_DOCUMENT_TITLE__`, `__PAGE_STYLES__`, `__PAGE_DOCUMENT_BODY__`, `__PAGE_DATA__`, `__PAGE_SCRIPTS__` marker가 각각 정확히 한 번 존재해야 한다.
- `__PAGE_DATA__`에는 PAGE block의 data block을 문서 순서대로 0개 이상 넣는다. 각 data block은 `<script type="application/json" id="page-data-NNNN" data-page-block="gantt">`이며 `NNNN`은 문서 안 PAGE block 등장 순서로 `0001`부터 붙인다.
- 각 component 요소는 `data-page-data="page-data-NNNN"`으로 자신의 data block을 가리키고, runtime은 component마다 독립된 인스턴스와 상태를 만든다.
- 사용하지 않는 component의 data와 script marker도 빈 문자열로 교체해 marker가 최종 HTML에 남지 않게 한다.
- marker 누락이나 중복은 `PAGE301`이며 부분 HTML을 반환하지 않는다.
- 기본 stylesheet는 `page-reset`, `page-base`, `page-markdown`, `page-components`, `page-theme`, `page-print` layer 순서로 조립한다.
- 사용자 CSS는 기본 layered stylesheet 뒤의 별도 `<style data-page-style="user">` 요소에 지정 순서대로 넣는다. unlayered author style이 layered 기본값보다 우선하므로 부분 override가 가능하다.
- 기본 stylesheet는 `!important`와 사용자별 inline style을 사용하지 않는다. 계산된 Gantt 위치처럼 runtime 값이 필요한 경우에는 전용 CSS custom property를 사용한다.
- 내장 JavaScript만 script 영역에 넣으며 Markdown이나 CLI를 통한 사용자 JavaScript 주입은 허용하지 않는다.

삽입 값은 문맥별로 처리한다.

- 문서 제목과 Markdown text는 HTML text escaping을 적용한다.
- link의 속성값은 허용 scheme을 검증한 뒤 HTML attribute escaping을 적용한다.
- Gantt JSON은 구조체 필드 순서로 직렬화하고 `<`, `>`, `&`, U+2028과 U+2029를 Unicode escape로 바꿔 `application/json` script 종료를 방지한다.
- 사용자 CSS는 내용을 변경해 escape하지 않고 ASCII case-insensitive `</style`, `@import`와 외부 asset URL을 검사해 거부한다.
- CSS `url()`은 `data:` URI와 문서 fragment만 허용한다.
- 사용자 CSS 파일마다 별도 style 요소를 사용하고 source 경로 대신 `data-page-style="user"`와 1부터 시작하는 순번만 기록한다.

결정적 HTML을 위해 다음을 고정한다.

- Markdown과 사용자 CSS의 UTF-8 BOM을 제거하고 CRLF/CR을 LF로 정규화한다.
- 기본 runtime 자산, component와 사용자 CSS의 조립 순서를 고정한다.
- JSON은 구조체와 `Vec`을 사용해 필드·항목 순서를 보존하고 출력에 `HashMap` 반복 순서를 사용하지 않는다.
- 자동 ID는 문서 입력 순서에만 의존한다.
- 생성 시각, 절대 경로, 임의 값과 환경별 메타데이터를 기록하지 않는다.
- 각 인라인 자산 사이에는 LF 하나를 사용하고 최종 HTML은 마지막 LF 하나로 끝낸다.

## 최소 인수 시나리오

| ID | 입력·옵션 | 예상 결과 |
|---|---|---|
| `A01` | `# Release plan`과 일반 문단만 포함 | title과 header는 `Release plan`, 문단은 content에 렌더링되고 기본 CSS 전체가 인라인됨 |
| `A02` | 제목 없는 일반 Markdown 문단 | title은 `PAGE Document`, 일반 문서가 오류 없이 생성됨 |
| `A03` | H1 없이 평면 task 두 개의 Gantt, 일부 ID·status 생략 | 문서 title은 `PAGE Document`, Gantt 제목은 component 안에 렌더링하고 `task-0001`부터 ID와 `not_started`를 결정적으로 할당함 |
| `A04` | milestone 아래 epic과 task | 원래 계층과 순서를 JSON·HTML에 보존하고 그룹 진행률은 하위 completed task로 계산함 |
| `A05` | 일반 Markdown 사이에 Gantt 하나 | 첫 H1만 header로 이동하고 나머지 Markdown과 Gantt는 원문 순서대로 content에 렌더링됨 |
| `A06` | `--style accent.css`가 accent 변수만 선언 | 나머지 기본 CSS는 유지되고 accent 변수만 사용자 값으로 override됨 |
| `A07` | `--style brand.css --style report.css`가 같은 속성 선언 | 두 CSS를 순서대로 인라인하며 동일 조건에서는 `report.css` 선언이 적용됨 |
| `A08` | 유효 입력과 `--dry-run` | 모델·CSS 검증은 성공하지만 HTML/PDF 및 출력 파일을 생성하지 않음 |
| `A09` | raw HTML, 알 수 없는 `page:chart`, 잘못된 Gantt 날짜 | 각각 `PAGE101`, `PAGE102`, `PAGE202` 영역의 원본 위치 진단을 반환함 |
| `A10` | 사용자 CSS에 `@import` 또는 `</style>` 포함 | `PAGE302`로 실패하고 HTML/PDF를 생성하지 않음 |
| `A11` | 같은 Markdown, runtime과 CSS 내용·순서로 두 번 HTML 생성 | 두 결과가 바이트 단위로 동일함 |
| `A12` | 같은 입력과 사용자 CSS로 HTML 및 PDF 생성 | PDF가 HTML과 같은 기본·사용자 스타일을 사용하고 편집 UI는 인쇄에서 제외됨 |
| `A13` | Markdown 섹션 사이에 Gantt 두 개, 일부 ID 생략 | 두 component와 data block이 원문 순서로 생성되고, ID는 두 블록을 통틀어 고유하며 자동 ID 번호가 블록 사이에서 이어짐 |

인수 테스트는 전체 HTML snapshot에만 의존하지 않는다. title, 문서 셸, block 순서, 내장 JSON, style/script 존재와 진단을 구조적으로 검사하고 결정성 시나리오에서만 전체 바이트를 비교한다.

## 런타임 구조

런타임은 HTML/CSS 절대배치로 사이드바와 타임라인을 구성한다.

```text
Gantt
├── Sidebar
│   └── Milestone | Epic | Task 행
└── Timeline
    ├── 날짜 헤더와 주말 음영
    ├── 그룹 요약 막대
    └── Task 일정 막대
```

- 한 문서의 여러 Gantt component는 각자의 data block으로 독립 초기화하며 상태를 공유하지 않는다.
- 평면 task와 계층형 항목을 같은 렌더링 파이프라인에서 처리한다.
- Epic과 milestone 행은 하위 task의 일정 범위와 계산 진행률을 표시한다.
- task 막대는 상태별 기본 표현을 사용한다.
- 기존 프로토타입의 편집·저장 기능은 보존하되 고도화는 후속 epic에서 진행한다.
- 편집 버튼은 Gantt component마다, HTML 저장과 PDF 출력 버튼은 문서에 하나만 둔다. 저장 시 각 Gantt가 자기 data block을 갱신한 뒤 문서 전체를 저장한다.
- 런타임 HTML은 의미 구조를, component CSS는 화면 표현을, print CSS는 PDF 표현을 담당한다.

## Task 경계

- `pt-0002`: Markdown-HTML 변환 파이프라인과 공개 API
- `pt-0003`: 중간 모델
- `pt-0004`: 기본 Markdown 구조화와 PAGE fenced block 추출
- `pt-0005`: Gantt 역직렬화, 검증과 정규화
- `pt-0006`: 계층형 Gantt 런타임
- `pt-0007`: 안전한 JSON 삽입과 단일 HTML 생성
- `pt-0008`: CLI 파일 입출력
- `pt-0009`: 브라우저 인쇄 엔진을 이용한 PDF 출력
- `pt-0010`: 전체 컴파일 흐름 검증
