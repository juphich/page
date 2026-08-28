# pt-0002 PAGE 중간 모델 정의

## 목표

Markdown 파서와 HTML 생성기 사이의 안정적인 PAGE 중간 모델을 정의한다.

## 작업 내용

- `PageDocument`, `PageBlock`, `Gantt`, `GanttItem`, `Milestone`, `Epic`, `GanttTask`를 정의한다.
- task의 시작일과 종료일을 구조화된 일정 값으로 표현한다.
- task 상태를 네 가지 값의 enum으로 표현한다.
- Epic과 milestone의 완료 task 기반 진행률 계산을 모델 동작으로 제공한다.
- 기본값, 알 수 없는 필드와 직렬화 정책을 결정한다.
- 모델의 단위 테스트를 작성한다.

## 완료 조건

- [ ] Gantt 문서를 표현하는 typed model이 구현된다.
- [ ] 모델 생성과 직렬화 테스트가 통과한다.

## 결정 사항

- MVP의 `GanttTask`에는 `progress`를 포함하지 않는다.
- `TaskStatus`는 `미진행`, `진행중`, `완료`, `보류`만 허용한다.
- `GanttItem`은 task, epic, milestone variant를 제공한다.
- Epic은 task를, milestone은 task 또는 epic을 소유한다. 두 그룹은 선택 사항이다.
- 진행률은 저장 필드가 아닌 하위 task 상태의 파생 값이다.
- `PageBlock`을 확장 가능한 enum 경계로 두고 Gantt 세부 모델은 독립 모듈에 둔다.
- 미래 기능을 미리 필드로 추가하지 않고, 필요할 때 선택 필드나 새 모델로 확장한다.

## 검증

- `cargo test --workspace`

## 후속 작업

- pt-0004, pt-0006
