# ep-0001 Rust 문서 생성기 설계

## 설계 목표

Markdown의 `page:gantt` 블록을 확장 가능한 중간 모델로 변환하고, 독립 실행 가능한 단일 HTML에 포함한다. 입력 문법은 [PAGE 입력 문법](../../syntax.md), CLI 계약은 [PAGE CLI 계약](../../cli.md)을 따른다.

## 컴파일 흐름

```text
Markdown
→ fenced block 추출
→ page:gantt 역직렬화
→ 검증·정규화
→ PAGE 중간 모델
→ Gantt 런타임이 포함된 단일 HTML
  ├→ HTML 파일
  └→ 브라우저 인쇄 엔진 → PDF 파일
```

파서와 HTML 생성기는 중간 모델만 공유한다. CLI는 파일 입출력만 담당하고 컴파일 규칙을 소유하지 않는다.

`--dry-run`은 검증·정규화 단계까지만 실행한다. PDF는 동일한 HTML 산출물을 로컬 브라우저 인쇄 엔진으로 변환해 HTML과 시각적 기준을 공유한다.

## 중간 모델

```text
PageDocument
└── PageBlock::Gantt
    └── Gantt
        └── GanttItem[]
            ├── Task
            ├── Epic
            │   └── Task[]
            └── Milestone
                └── (Epic | Task)[]
```

- 차트는 task만으로 구성할 수 있다.
- Epic과 milestone은 선택적인 그룹이다.
- Epic은 task만, milestone은 task 또는 epic을 포함한다.
- milestone, epic과 task를 나타내는 ID는 문서 전체에서 고유하다.
- 입력 날짜는 검증 후 `DateRange`로 정규화한다.
- task 상태는 `TaskStatus` enum으로 표현한다.

## 진행률과 일정 파생

Epic과 milestone의 진행률과 일정 범위는 저장하지 않는다.

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

- 평면 task와 계층형 항목을 같은 렌더링 파이프라인에서 처리한다.
- Epic과 milestone 행은 하위 task의 일정 범위와 계산 진행률을 표시한다.
- task 막대는 상태별 기본 표현을 사용한다.
- 기존 프로토타입의 편집·저장 기능은 보존하되 고도화는 후속 epic에서 진행한다.

## Task 경계

- `pt-0002`: 중간 모델과 파생 값 API
- `pt-0003`: Markdown fenced block 추출
- `pt-0004`: Gantt 역직렬화, 검증과 정규화
- `pt-0005`: 계층형 Gantt 런타임
- `pt-0006`: 안전한 JSON 삽입과 단일 HTML 생성
- `pt-0007`: CLI 파일 입출력
- `pt-0009`: 브라우저 인쇄 엔진을 이용한 PDF 출력
- `pt-0008`: 전체 컴파일 흐름 검증
