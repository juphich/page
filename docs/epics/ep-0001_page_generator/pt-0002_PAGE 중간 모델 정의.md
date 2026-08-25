# pt-0002 PAGE 중간 모델 정의

## 목표

Markdown 파서와 HTML 생성기 사이의 안정적인 PAGE 중간 모델을 정의한다.

## 작업 내용

- `PageDocument`, `PageBlock`, `Gantt`, `GanttTask`를 정의한다.
- 기본값, 알 수 없는 필드와 직렬화 정책을 결정한다.
- 모델의 단위 테스트를 작성한다.

## 완료 조건

- [ ] Gantt 문서를 표현하는 typed model이 구현된다.
- [ ] 모델 생성과 직렬화 테스트가 통과한다.

## 결정 사항

- 작업 진행 중 기록한다.

## 검증

- `cargo test --workspace`

## 후속 작업

- pt-0004, pt-0006
