# pt-0003 PAGE 중간 모델 정의

## 목표

Markdown 파서와 HTML 생성기 사이의 안정적인 PAGE 중간 모델을 정의한다.

## 작업 내용

- `PageDocument`, `DocumentBlock`, `MarkdownBlock`, `PageBlock`의 문서 계층을 정의한다.
- 제목, 문단, 표, 인용구, 목록과 코드 블록의 기본 모델 경계를 정의한다.
- text, 강조, 링크, 이미지와 inline code 등 inline 요소 모델을 정의한다.
- `Gantt`, `GanttItem`, `Milestone`, `Epic`, `GanttTask`를 PAGE 전용 모델로 정의한다.
- task의 시작일과 종료일을 구조화된 일정 값으로 표현한다.
- task 상태를 네 가지 값의 enum으로 표현한다.
- 기본값, 알 수 없는 필드와 직렬화 정책을 결정한다.
- 모델의 단위 테스트를 작성한다.

## 완료 조건

- [x] 일반 Markdown 요소와 PAGE 전용 블록을 함께 표현하는 typed model이 구현된다.
- [x] 새 PAGE 블록 종류를 독립적으로 추가할 수 있는 모델 경계가 마련된다.
- [x] 모델 생성과 직렬화 테스트가 통과한다.

## 결정 사항

- MVP의 `GanttTask`에는 `progress`를 포함하지 않는다.
- `TaskStatus`는 `미진행`, `진행중`, `완료`, `보류`만 허용한다.
- `GanttItem`은 task, epic, milestone variant를 제공한다.
- Epic은 task를, milestone은 task 또는 epic을 소유한다. 두 그룹은 선택 사항이다.
- Epic과 milestone의 진행률과 일정 범위는 Rust 모델에 저장하거나 계산하지 않는다. 런타임 JS가 Gantt JSON의 하위 task 상태와 날짜로 계산해 그린다.
- `DocumentBlock`은 일반 Markdown과 PAGE 전용 블록을 구분한다.
- `MarkdownBlock`은 기본 Markdown 요소를, `PageBlock`은 PAGE 전용 요소를 나타낸다.
- `PageBlock`을 확장 가능한 enum 경계로 두고 Gantt 세부 모델은 독립 모듈에 둔다.
- 각 도메인의 필드는 구체 타입에 유지한다. 사용자가 지정하는 공통 노드 속성은 입력 문법이 생길 때까지 두지 않는다.
- 중간 모델은 소스 위치를 보존하지 않는다. 진단은 모델 생성 전 단계에서 만들어지므로 위치는 pt-0004 파서 내부 타입과 pt-0005 입력 DTO가 보존한다.
- 미래 기능을 미리 필드로 추가하지 않고, 필요할 때 선택 필드나 새 모델로 확장한다.
- 중간 모델의 모든 타입은 `pub(crate)`로 둔다. 외부 공개 API는 `compile_html`과 `validate`뿐이다.
- 데이터를 가진 enum은 variant마다 허용되는 자식만 담아 잘못된 계층을 타입으로 표현할 수 없게 한다.
- 형식·범위 규칙은 값 타입의 생성자가 보장하고, 문서 전체를 봐야 하는 규칙만 pt-0005 validator에 둔다.
- 모델에는 `Serialize`만 구현하고 `Deserialize`는 구현하지 않는다. YAML 입력과 알 수 없는 필드 정책은 pt-0005 입력 DTO가 담당한다.
- 날짜는 외부 crate 없이 자체 `CalendarDate`로 구현한다. 요일과 날짜 간격 계산은 런타임 JS가 담당한다.
- 문서는 Markdown 섹션과 종류별 PAGE 블록을 원문 순서대로 0개 이상 가진다. 같은 종류의 PAGE 블록도 여러 개 올 수 있다.
- PAGE 블록은 문서 최상위에만 위치한다. 목록 항목이나 인용구 안의 `page:*` 블록은 pt-0004 파서가 `PAGE104` 오류로 처리한다.
- 문서 제목은 첫 level-1 heading으로만 결정한다. PAGE 블록의 제목은 문서 제목에 쓰지 않고 각 블록이 소유한다.

## 설계

### 모듈 구성

```text
crates/page/core/src/
├── document/
│   ├── mod.rs        # 하위 모듈 선언, PageDocument·DocumentBlock·PageBlock 재내보내기
│   ├── page.rs       # PageDocument, DocumentBlock, PageBlock
│   ├── block.rs      # MarkdownBlock, HeadingLevel, List, ListItem, Table, TableRow, TableCell, Alignment
│   ├── inline.rs     # Inline
│   └── url.rs        # SafeUrl, DataImageUri
└── gantt/
    ├── mod.rs        # 하위 모듈 선언, 타입 재내보내기
    ├── chart.rs      # Gantt, SchemaVersion, GanttItem, MilestoneItem, Milestone, Epic, GanttTask, EmptyItems
    ├── id.rs         # ItemId
    ├── date.rs       # CalendarDate, DateRange
    └── status.rs     # TaskStatus
```

- 기존 `src/document.rs`, `src/gantt.rs` 스텁은 위 모듈로 교체한다.
- `src/markdown.rs`(pt-0004 파서)와 `src/generator.rs`(pt-0007)는 이 task에서 변경하지 않는다. 모델 파일은 파서와 이름이 겹치지 않도록 `block.rs`, `inline.rs`로 둔다.
- 문서 최상위 타입은 `page.rs`에 두고, `page` 모듈은 비공개로 둔 채 `document/mod.rs`에서 `pub(crate) use page::{DocumentBlock, PageBlock, PageDocument}`로 다시 내보낸다. 사용처는 `crate::document::PageDocument`로 접근한다. 나머지 하위 모듈은 `pub(crate) mod`로 선언한다.
- `gantt`의 `chart`, `id`, `date`, `status` 모듈도 비공개로 두고 `gantt/mod.rs`에서 구조·값 타입과 오류 타입을 `pub(crate) use`로 다시 내보낸다. 사용처는 `crate::gantt::ItemId`처럼 접근한다.
- 공개 API 오류 타입 `CompileError`는 `src/errors.rs`에 두고 `lib.rs`에서 `pub use`로 다시 내보낸다.
- `document`는 `PageBlock::Gantt`로 `gantt`를 참조하고, `gantt`는 `document`를 참조하지 않는다.

### 문서 모델

```rust
pub(crate) struct PageDocument {
    pub title: Option<Vec<Inline>>, // 첫 H1, 없으면 렌더러가 "PAGE Document"
    pub blocks: Vec<DocumentBlock>, // 원문 순서, 문서 제목으로 쓴 첫 H1은 제외
}
// title_text(&self) -> String: <title>용으로 inline을 평탄화하고 없으면 "PAGE Document"를 반환한다.

pub(crate) enum DocumentBlock { Markdown(MarkdownBlock), Page(PageBlock) }

pub(crate) enum PageBlock { Gantt(Gantt) }

pub(crate) enum MarkdownBlock {
    Heading { level: HeadingLevel, content: Vec<Inline> },
    Paragraph(Vec<Inline>),
    List(List),
    BlockQuote(Vec<MarkdownBlock>),
    CodeBlock { language: Option<String>, code: String },
    Table(Table),
    ThematicBreak,
}

pub(crate) enum HeadingLevel { H1, H2, H3, H4, H5, H6 }

pub(crate) struct List {
    pub start: Option<u64>,   // None: 글머리표 목록, Some(n): n부터 번호
    pub items: Vec<ListItem>, // 1개 이상
}
pub(crate) struct ListItem {
    pub checked: Option<bool>, // task list가 아니면 None
    pub blocks: Vec<MarkdownBlock>,
}

pub(crate) struct Table { pub header: TableRow, pub rows: Vec<TableRow> }
pub(crate) struct TableRow { pub cells: Vec<TableCell> }
pub(crate) struct TableCell { pub align: Alignment, pub content: Vec<Inline> }
pub(crate) enum Alignment { None, Left, Center, Right }

pub(crate) enum Inline {
    Text(String),
    Code(String),
    Emphasis(Vec<Inline>),
    Strong(Vec<Inline>),
    Strikethrough(Vec<Inline>),
    Link { href: SafeUrl, title: Option<String>, content: Vec<Inline> },
    Image { src: Option<DataImageUri>, alt: String, title: Option<String> }, // None: 이미지 미구현 빈 박스
    SoftBreak,
    HardBreak,
}

pub(crate) struct SafeUrl(String);      // http, https, mailto, #fragment, 상대 경로
pub(crate) struct DataImageUri(String); // data:image/*
```

- 첫 H1 추출은 pt-0004 파서가 한다. 모델은 결과만 담고 PAGE 블록에서 제목을 가져오지 않는다.
- `blocks`는 `Page(Gantt)`를 여러 개 포함할 수 있다. 각 Gantt는 자기 `title`을 component 안에 렌더링한다.
- `BlockQuote`와 `ListItem`의 자식은 `MarkdownBlock`이므로 PAGE 블록은 문서 최상위에만 위치한다.
- `SafeUrl`과 `DataImageUri`는 `parse`를 통과한 값만 존재하므로 렌더러는 scheme을 다시 검사하지 않는다.

### Gantt 모델

```rust
#[derive(Serialize)]
pub(crate) struct Gantt {
    pub version: SchemaVersion,
    pub title: String,
    items: Vec<GanttItem>, // new()가 1개 이상을 보장
}

pub(crate) enum SchemaVersion { V1 } // 정수 1로 직렬화

#[derive(Serialize)]
#[serde(untagged)] // type 키는 각 구조체가 쓴다
pub(crate) enum GanttItem { Task(GanttTask), Epic(Epic), Milestone(Milestone) }

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum MilestoneItem { Task(GanttTask), Epic(Epic) }

#[derive(Serialize)]
#[serde(tag = "type", rename = "milestone")]
pub(crate) struct Milestone {
    pub id: ItemId,
    pub name: String,
    items: Vec<MilestoneItem>, // new()가 1개 이상을 보장
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "epic")]
pub(crate) struct Epic {
    pub id: ItemId,
    pub name: String,
    #[serde(rename = "items")]
    tasks: Vec<GanttTask>, // new()가 1개 이상을 보장
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "task")]
pub(crate) struct GanttTask {
    pub id: ItemId,
    pub name: String,
    #[serde(flatten)]
    pub schedule: DateRange, // "start", "end"
    pub status: TaskStatus,
}
```

- `Gantt::new`, `Milestone::new`, `Epic::new`는 자식 목록이 비어 있으면 `EmptyItems`를 반환한다.
- 자식 목록은 비공개 필드이며 `Gantt::items()`, `Milestone::items()`, `Epic::tasks()` 조회 메서드로 읽는다.
- `type` 키는 enum이 아니라 `Milestone`, `Epic`, `GanttTask`가 직접 쓴다. `Epic`은 task를 enum 없이 담으므로 enum에 태그를 두면 epic 안의 task에 `type`이 빠진다.
- `MilestoneItem`에는 Milestone이 없고 `Epic`은 `GanttTask`만 소유하므로 milestone·epic 중첩은 표현할 수 없다.

### 값 타입

| 타입 | 표현 | 생성 | 거부하는 값 |
|---|---|---|---|
| `ItemId` | `String` newtype | `ItemId::parse(&str)` | 영문자로 시작하지 않음, 허용 외 문자, 0자 또는 65자 이상 |
| `CalendarDate` | `year: u16`, `month: u8`, `day: u8`, `Copy`·`Ord` | `CalendarDate::parse(&str)` | `YYYY-MM-DD`가 아닌 형식, 존재하지 않는 날짜 |
| `DateRange` | 비공개 `start`, `end`, 종료일 포함 | `DateRange::new(start, end)` | `start > end` |
| `TaskStatus` | `NotStarted`, `InProgress`, `Completed`, `OnHold` | `Default`는 `NotStarted` | 입력 문자열 해석은 pt-0005 DTO |

- `CalendarDate`는 `Display`로 `YYYY-MM-DD`를 출력하고, `TaskStatus::as_str`는 `not_started` 등 입력 식별자를 반환한다.
- 상태의 표시 문구와 색은 모델이 아니라 렌더러와 CSS가 결정한다.
- 각 생성자는 `ItemIdError`, `DateError`, `DateRangeError`를 반환하고 pt-0005가 이를 `PAGE202`, `PAGE203` 진단으로 변환한다.

### 직렬화

Gantt 블록마다 JSON data block 하나를 만든다. 내용은 정규화된 입력 YAML과 같은 구조다.

```json
{"version":1,"title":"출시 일정","items":[
  {"type":"epic","id":"epic-0001","name":"기획","items":[
    {"type":"task","id":"requirements","name":"요구사항",
     "start":"2026-09-01","end":"2026-09-05","status":"completed"}]}]}
```

- 키 순서는 구조체 필드 순서로 고정한다.
- 진행률과 그룹 일정은 JSON에 넣지 않는다. 런타임이 계산한다.
- 문서 모델(`PageDocument`, `MarkdownBlock`, `Inline`)은 JSON으로 직렬화하지 않는다.

### 불변식 보장 위치

| 불변식 | 보장 위치 |
|---|---|
| epic 안 epic, milestone 안 milestone 금지 | 타입 (`MilestoneItem`, `Epic.tasks`) |
| PAGE 블록은 문서 최상위에만 위치 | 타입 (`BlockQuote`, `ListItem`의 자식이 `MarkdownBlock`) |
| 실존 날짜, `start ≤ end` | `CalendarDate::parse`, `DateRange::new` |
| ID 형식 | `ItemId::parse` |
| Gantt·Milestone·Epic의 items 1개 이상 | 각 `new()` |
| 링크 scheme, 이미지 data URI | `SafeUrl::parse`, `DataImageUri::parse` |
| ID가 여러 Gantt 블록을 통틀어 고유, 자동 ID, 이름 공백 제거, 기본값 | pt-0005 정규화·검증 |

### 의존성

- `page-core`에 `serde`(`derive`)를 추가하고 `serde_json`은 dev-dependency로 추가한다. pt-0007에서 일반 의존성으로 전환한다.
- 모든 모델 타입은 `Debug`, `Clone`, `PartialEq`, `Eq`를 구현하고 날짜·상태 값 타입은 `Copy`를 구현한다.

## 구현 체크리스트

### 준비

- [x] `page-core`에 `serde`(`derive`)와 dev-dependency `serde_json`을 추가한다.
- [x] 기존 `src/document.rs`, `src/gantt.rs` 스텁을 삭제하고 `document/`, `gantt/` 디렉터리 모듈을 만든다.
- [x] `lib.rs`의 `document`, `gantt` 모듈 선언을 `pub(crate) mod`로 바꾼다.
- [x] 사용처가 생기기 전의 `dead_code` 경고를 두 모듈 단위 `allow`로 처리하고, 제거 시점을 pt-0007 후속 작업으로 기록한다.

### 문서 모델 (Markdown)

- [x] `SafeUrl::parse`를 구현한다. `http`, `https`, `mailto`, `#fragment`와 scheme 없는 상대 경로만 허용한다.
- [x] scheme 판정은 대소문자를 구분하지 않고, 앞뒤 공백과 제어 문자를 포함한 우회 입력(`" javascript:"`, `"java\tscript:"`)을 거부한다.
- [x] `DataImageUri::parse`를 구현한다. `data:image/` 접두어만 허용한다.
- [x] URL 테스트: 허용 형식 각각 통과, `javascript:`·`JAVASCRIPT:`·`data:text/html`·`file:` 거부, 우회 입력 거부
- [x] `MarkdownBlock`, `HeadingLevel`, `List`, `ListItem`, `Table`, `TableRow`, `TableCell`, `Alignment`, `Inline`을 구현한다.
- [x] `PageDocument`, `DocumentBlock::Markdown`과 `PageDocument::title_text`를 구현한다. `PageBlock`은 Gantt 구조 단계에서 연결한다.
- [x] `title_text` 테스트: 강조·링크·inline code·줄바꿈 평탄화, 제목이 없으면 `PAGE Document`

### Gantt 값 타입

- [x] `ItemId::parse`, `as_str`, `ItemIdError`를 구현한다.
- [x] `ItemId` 테스트: `a`와 64자 허용, 65자·`1abc`·`a.b`·빈 문자열 거부
- [x] `CalendarDate::parse`를 구현한다. 정확히 `YYYY-MM-DD` 10자, 숫자 자리, 월 1–12, 월별 일수와 윤년을 검사한다.
- [x] `CalendarDate`에 `Ord`, `Copy`, `Display`, 문자열 `Serialize`를 구현한다.
- [x] `CalendarDate` 테스트: 정상 파싱, `2028-02-29` 허용, `2026-02-29`·`2026-13-01`·`2026-9-1` 거부, 순서 비교, `Display` 왕복
- [x] `DateRange::new`, `start`, `end`, `DateRangeError`를 구현하고 `start`/`end` 키로 직렬화한다.
- [x] `DateRange` 테스트: 같은 날 허용, `start > end` 거부
- [x] `TaskStatus`에 `Default`(`NotStarted`), `as_str`와 `as_str` 문자열로 쓰는 `Serialize`를 구현한다.
- [x] `SchemaVersion::V1`을 정수 `1`로 직렬화한다.

### Gantt 구조

- [x] `GanttTask`, `Epic`, `MilestoneItem`, `Milestone`, `GanttItem`, `Gantt`와 `EmptyItems`를 구현한다.
- [x] `Gantt::new`, `Milestone::new`, `Epic::new`가 빈 자식 목록을 거부하고 `items()`, `tasks()` 조회 메서드를 제공한다.
- [x] 그룹 생성자 테스트: 빈 목록이면 `EmptyItems`
- [x] `PageBlock::Gantt`와 `DocumentBlock::Page`를 추가해 Gantt를 문서 모델에 연결한다.
- [x] JSON 직렬화 테스트: milestone·epic·task 예시의 전체 JSON 일치, `type` 태그, `Epic.tasks`의 `items` 키, `start`/`end` 평탄화, 키 순서, `progress` 키 없음

### 마무리

- [x] 모든 모델 타입에 `Debug`, `Clone`, `PartialEq`, `Eq`를 구현하고 값 타입에 `Copy`를 구현한다.
- [x] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets`, `cargo test --workspace`가 통과한다.
- [x] 완료 조건과 `plan.md`의 pt-0003 체크리스트를 갱신하고 검증 결과를 이 문서에 기록한다.

## 검증

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets`: 경고 없음
- `cargo test --workspace`: 25개 통과 (문서 모델 8, Gantt 값 타입 14, Gantt 구조 3)
- 모든 모델 타입의 `Debug`, `Clone`, `PartialEq`, `Eq`와 값·오류 타입의 `Copy`를 확인했다.
- 완료 조건 대조: Markdown과 PAGE 블록을 함께 담는 `DocumentBlock`, 새 PAGE 블록 종류를 variant로 추가하는 `PageBlock` 경계, 생성자·직렬화 테스트

### 진행 기록

- 이미지 미구현 반영 (2026-10-18): pt-0004 설계에서 이미지를 미구현으로 두기로 해 `Inline::Image.src`를 `Option<DataImageUri>`로 바꾼다. 코드 변경은 pt-0004 1단계에서 한다.
- 5단계 마무리 (2026-10-18): 모델 타입의 trait 구현을 점검하고 다른 오류 타입과 맞춰 `ItemIdError`에 `Copy`를 추가했다. 완료 조건과 `plan.md`를 갱신하고 검증 결과를 기록했다. `version`은 형식 변경에 대비해 유지하기로 했다.
- 4단계 Gantt 구조 (2026-10-18): `chart.rs`에 `Gantt`, `SchemaVersion`, `GanttItem`, `MilestoneItem`, `Milestone`, `Epic`, `GanttTask`, `EmptyItems`를 구현하고 `page.rs`에 `PageBlock::Gantt`, `DocumentBlock::Page`를 추가했다. 설계와 다른 점: `gantt/mod.rs`는 `document/mod.rs`처럼 선언과 재내보내기만 두고 구조 타입과 `SchemaVersion`을 `chart.rs`로 옮겼다. 전체 JSON 테스트에서 epic 안의 task에 `type`이 빠지는 문제가 드러나, `type` 태그를 enum 대신 `Milestone`, `Epic`, `GanttTask` 구조체에 두고 enum은 `untagged`로 바꿨다. `cargo fmt --check`, `cargo clippy`, `cargo test`(25개) 통과.
- 3단계 Gantt 값 타입 (2026-10-15): `id.rs`(`ItemId`), `date.rs`(`CalendarDate`, `DateRange`), `status.rs`(`TaskStatus`), `mod.rs`(`SchemaVersion`)를 구현했다. 설계와 다른 점: `ItemIdError`는 `Empty`, `TooLong`, `InvalidStart`, `InvalidCharacter`로, `DateError`는 `InvalidFormat`, `NonexistentDate`로 나누고 0년을 거부한다. pt-0005 중복 ID 검사를 위해 `ItemId`, `CalendarDate`에 `Hash`를 둔다. `TaskStatus`의 JSON 문자열은 serde 이름 규칙 대신 `as_str`를 써서 상태 이름을 한 곳에서만 정의한다. `gantt` 하위 모듈은 비공개로 두고 `gantt/mod.rs`에서 다시 내보내며, 임시 `#[allow(unused_imports)]` 제거를 pt-0007에 기록했다. `cargo fmt --check`, `cargo clippy`, `cargo test`(22개) 통과.
- 2단계 문서 모델 (2026-10-11): `url.rs`(`SafeUrl`, `DataImageUri`), `block.rs`(Markdown 블록), `inline.rs`(`Inline`, `plain_text`), `page.rs`(`PageDocument`, `DocumentBlock::Markdown`, `title_text`)를 구현했다. 설계와 다른 점: `UrlError`는 `Empty`, `UnsafeCharacter`, `DisallowedScheme`, `NotDataImage`로 나누고 빈 링크 주소도 거부한다. scheme은 URL 표준대로 `/`, `?`, `#` 앞의 `:`로만 판정해 `docs/a:b.md` 같은 상대 경로를 허용한다. 평탄화 함수 `plain_text`는 heading id에서도 쓰도록 `inline.rs`에 두고 이미지는 alt 텍스트로 바꾼다. `page` 모듈은 비공개로 두고 `document`에서 `PageDocument`, `DocumentBlock`을 다시 내보내며, 임시 `#[allow(unused_imports)]` 제거를 pt-0007에 기록했다. `cargo fmt --check`, `cargo clippy`, `cargo test`(8개) 통과.
- 단계 순서 변경 (2026-10-10): 사용자 요청으로 Markdown 문서 모델을 2단계로 앞당기고 Gantt 값 타입·구조를 3·4단계로 옮겼다. `PageBlock`과 `DocumentBlock::Page`는 Gantt 구조 단계에서 추가한다.
- 1단계 준비 (2026-10-10): serde 의존성, `document/`·`gantt/` 모듈 골격, `pub(crate)` 선언, 임시 `dead_code` 허용을 적용했다. 모델 파일은 파서 `src/markdown.rs`와 이름이 겹치지 않도록 `block.rs`, `inline.rs`로 정했고 `CompileError`를 `src/errors.rs`로 분리했다. `cargo fmt --check`, `cargo clippy`, `cargo test` 통과.

## 선행 작업

- pt-0002

## 후속 작업

- pt-0005, pt-0007
