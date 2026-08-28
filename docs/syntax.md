# PAGE 입력 문법

## MVP 범위

MVP는 Markdown 문서 안의 `page:gantt` fenced block 하나를 처리한다. 일반 Markdown 본문과 다른 PAGE 블록은 후속 epic에서 지원한다.

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
- milestone, epic과 task를 포함해 문서 전체에서 고유해야 한다.
- 자동 ID는 `milestone-0001`, `epic-0001`, `task-0001` 형식을 사용한다.
- 명시 ID를 먼저 예약한 뒤 깊이 우선 입력 순서로 사용되지 않은 다음 자동 ID를 할당한다.

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

- `page:gantt` 블록 누락 또는 둘 이상 존재
- YAML 구문 오류 또는 지원하지 않는 스키마 버전
- 중복 key, anchor, alias 또는 merge key 사용
- 필수 필드 누락, 알 수 없는 필드 또는 정의되지 않은 상태
- 규칙에 맞지 않거나 중복된 ID
- 비어 있는 최상위 또는 그룹 `items`
- 허용되지 않은 계층 중첩
- 따옴표 없는 날짜, 잘못된 날짜 또는 `start > end`
