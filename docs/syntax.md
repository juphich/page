# PAGE 입력 문법

## MVP 범위

MVP는 일반 Markdown 본문과 0개 이상의 `page:gantt` fenced block을 처리한다. 각 블록은 문서 안의 독립된 Gantt 섹션이며 Markdown 섹션과 원문 순서대로 섞일 수 있다. `page:chart`, `page:report`와 Mermaid 호환은 후속 epic에서 지원한다.

지원하는 일반 Markdown 요소는 다음과 같다.

- 제목 `h1`–`h6`, 문단과 줄바꿈
- 강조, 굵게, 취소선과 인라인 코드
- 순서 있는 목록, 순서 없는 목록과 task list
- 링크와 이미지
- 인용구, 구분선과 fenced/indented code block
- GFM 표

raw HTML은 단일 문서의 구조와 스크립트 안전성을 보장하기 위해 MVP에서 허용하지 않는다. raw HTML이 필요하면 후속 버전에서 별도의 신뢰 입력 옵션과 sanitization 정책을 정의한다.

링크는 `http`, `https`, `mailto`, 문서 안의 fragment와 상대 경로를 허용한다. 각 제목은 제목 텍스트로 만든 `id`를 가지므로 `[일정](#일정)`처럼 제목으로 이동하는 링크를 쓸 수 있다. `javascript:`와 실행 가능한 `data:` 링크는 오류다. 이미지는 단일 파일 원칙에 따라 `data:image/*` URI만 허용하며 상대 파일과 원격 이미지는 asset embedding을 설계할 때까지 지원하지 않는다.

첫 번째 level-1 제목을 문서 제목으로 사용하고 원래 inline 표현을 유지해 문서 header에 한 번만 렌더링한다. HTML `<title>`에는 이를 plain text로 평탄화한 값을 사용한다. 이후 제목은 content 영역에서 원래 순서대로 렌더링한다. level-1 제목이 없으면 `PAGE Document`를 사용한다. `page:gantt`의 `title`은 문서 제목에 사용하지 않으며 항상 해당 Gantt component 안에 렌더링한다.

일반 fenced code block은 언어 식별자를 class로 보존해 코드로 렌더링한다. 정확히 `page:gantt`인 fenced block만 PAGE 블록으로 처리한다. 알려지지 않은 `page:*` 식별자는 오타를 숨기지 않도록 오류로 처리하고, 그 밖의 알려지지 않은 언어 식별자는 일반 코드 블록으로 렌더링한다. PAGE 블록은 문서 최상위에만 둘 수 있으며 목록 항목이나 인용구 안의 `page:*` fenced block은 오류다.

````markdown
```page:gantt
version: 1
title: 출시 일정
items:
  - type: task
    name: 일정 준비
    start: "2026-08-29"
    end: "2026-08-31"
    status: completed
  - type: epic
    id: planning
    name: 기획
    items:
      - type: task
        id: requirements
        name: 요구사항 분석
        start: "2026-09-01"
        end: "2026-09-05"
        status: in_progress
```
````

## Gantt 필드

| 필드 | 필수 | 기본값 | 규칙 |
|---|---|---|---|
| `version` | 아니요 | `1` | 지원하는 스키마 버전이어야 한다. |
| `title` | 예 | 없음 | 공백이 아닌 문자열이어야 한다. |
| `items` | 예 | 없음 | 하나 이상의 Gantt 항목을 포함한다. |

## Gantt 항목

| `type` | 허용 위치 | 하위 항목 |
|---|---|---|
| `task` | 차트, milestone, epic | 없음 |
| `epic` | 차트, milestone | task만 허용 |
| `milestone` | 차트 | task 또는 epic 허용 |

차트는 task만 나열할 수 있다. Epic과 milestone은 그룹과 진행률 표시가 필요할 때만 사용한다. milestone 안에 milestone을 넣거나 epic 안에 epic을 넣는 중첩은 허용하지 않는다.

여기서 milestone은 여러 task 또는 epic을 묶는 일정 그룹이다. 특정 날짜를 나타내는 다이아몬드 milestone은 MVP 범위에 포함하지 않는다. 모든 항목은 입력 순서를 유지해 렌더링한다.

## Epic과 Milestone 필드

| 필드 | 필수 | 기본값 | 규칙 |
|---|---|---|---|
| `type` | 예 | 없음 | `epic` 또는 `milestone`이다. |
| `id` | 아니요 | 순서 기반 자동 생성 | 문서 안에서 고유해야 한다. |
| `name` | 예 | 없음 | 공백이 아닌 문자열이어야 한다. |
| `items` | 예 | 없음 | 허용된 하위 항목을 하나 이상 포함한다. |

## Task 필드

| 필드 | 필수 | 기본값 | 규칙 |
|---|---|---|---|
| `type` | 예 | 없음 | `task`여야 한다. |
| `id` | 아니요 | 순서 기반 자동 생성 | 문서 안에서 고유해야 한다. |
| `name` | 예 | 없음 | 공백이 아닌 문자열이어야 한다. |
| `start` | 예 | 없음 | `YYYY-MM-DD` 형식의 유효한 날짜다. |
| `end` | 예 | 없음 | 포함 날짜이며 `start`보다 빠를 수 없다. |
| `status` | 아니요 | `not_started` | 정의된 상태 식별자만 허용한다. |

## ID 규칙

- ID는 영문자로 시작하고 영문자, 숫자, `_`, `-`만 사용한다.
- 길이는 1자 이상 64자 이하다.
- milestone, epic과 task를 포함해 문서의 모든 `page:gantt` 블록을 통틀어 고유해야 한다.
- 자동 ID는 `milestone-0001`, `epic-0001`, `task-0001` 형식을 사용한다.
- 문서 전체의 명시 ID를 먼저 예약한 뒤 블록의 문서 순서와 블록 안의 깊이 우선 입력 순서로 사용되지 않은 다음 자동 ID를 할당한다. 자동 ID 번호는 블록 사이에서 이어진다.

## Task 상태

| 입력값 | 표시 | 의미 |
|---|---|---|
| `not_started` | 미진행 | 아직 시작하지 않음 |
| `in_progress` | 진행중 | 현재 진행 중 |
| `completed` | 완료 | 작업 완료 |
| `on_hold` | 보류 | 작업을 일시 중단함 |

`progress`와 사용자 지정 `color`는 MVP에서 지원하지 않는다. 렌더러가 상태에 따라 기본 표현을 결정하며, 필요한 경우 후속 스키마에서 선택 필드로 확장한다.

Epic과 milestone의 진행률은 입력하지 않는다. 하위 task 상태로부터 계산하는 방법은 [ep-0001 설계](epics/ep-0001_page_generator/design.md#진행률과-일정-파생)에 정의한다.

## YAML 제한

- `version`은 정수 `1`만 허용한다.
- 날짜는 자동 타입 변환을 피하기 위해 따옴표로 감싼 문자열로 작성한다.
- 중복 key, anchor, alias와 merge key는 허용하지 않는다.
- `title`과 `name`은 앞뒤 공백을 제거해 정규화하며 결과가 비어 있으면 오류다.
- 정의되지 않은 필드는 향후 버전 필드도 포함해 현재 버전에서 오류로 처리한다.

## 오류 조건

- raw HTML 또는 알 수 없는 `page:*` fenced block 사용
- 목록 항목이나 인용구 안에 놓인 `page:*` fenced block
- YAML 구문 오류 또는 지원하지 않는 스키마 버전
- 중복 key, anchor, alias 또는 merge key 사용
- 필수 필드 누락, 알 수 없는 필드 또는 정의되지 않은 상태
- 규칙에 맞지 않거나 중복된 ID
- 비어 있는 최상위 또는 그룹 `items`
- 허용되지 않은 계층 중첩
- 따옴표 없는 날짜, 잘못된 날짜 또는 `start > end`
