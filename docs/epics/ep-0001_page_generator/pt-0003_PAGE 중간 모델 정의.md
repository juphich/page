# pt-0003 PAGE 중간 모델 정의

## 목표

Markdown 파서와 HTML 생성기 사이의 안정적인 PAGE 중간 모델을 정의한다.

## 작업 내용

- `PageDocument`, `DocumentBlock`, `MarkdownBlock`, `PageBlock`의 문서 계층을 정의한다.
- 제목, 문단, 표, 인용구, 목록과 코드 블록의 기본 모델 경계를 정의한다.
- text, 강조, 링크, 이미지와 inline code 등 inline 요소 모델을 정의한다.
- 공통 소스 위치와 노드 메타데이터를 정의한다.
- `Gantt`, `GanttItem`, `Milestone`, `Epic`, `GanttTask`를 PAGE 전용 모델로 정의한다.
- task의 시작일과 종료일을 구조화된 일정 값으로 표현한다.
- task 상태를 네 가지 값의 enum으로 표현한다.
- Epic과 milestone의 완료 task 기반 진행률 계산을 모델 동작으로 제공한다.
- 기본값, 알 수 없는 필드와 직렬화 정책을 결정한다.
- 모델의 단위 테스트를 작성한다.

## 완료 조건

- [ ] 일반 Markdown 요소와 PAGE 전용 블록을 함께 표현하는 typed model이 구현된다.
- [ ] 새 PAGE 블록 종류를 독립적으로 추가할 수 있는 모델 경계가 마련된다.
- [ ] 모델 생성과 직렬화 테스트가 통과한다.

## 결정 사항

- MVP의 `GanttTask`에는 `progress`를 포함하지 않는다.
- `TaskStatus`는 `미진행`, `진행중`, `완료`, `보류`만 허용한다.
- `GanttItem`은 task, epic, milestone variant를 제공한다.
- Epic은 task를, milestone은 task 또는 epic을 소유한다. 두 그룹은 선택 사항이다.
- 진행률은 저장 필드가 아닌 하위 task 상태의 파생 값이다.
- `DocumentBlock`은 일반 Markdown과 PAGE 전용 블록을 구분한다.
- `MarkdownBlock`은 기본 Markdown 요소를, `PageBlock`은 PAGE 전용 요소를 나타낸다.
- `PageBlock`을 확장 가능한 enum 경계로 두고 Gantt 세부 모델은 독립 모듈에 둔다.
- 공통 노드 추상화에는 소스 위치와 속성만 두고 각 도메인의 필드는 구체 타입에 유지한다.
- 미래 기능을 미리 필드로 추가하지 않고, 필요할 때 선택 필드나 새 모델로 확장한다.

## 검증

- `cargo test --workspace`

## 선행 작업

- pt-0002

## 후속 작업

- pt-0005, pt-0007
