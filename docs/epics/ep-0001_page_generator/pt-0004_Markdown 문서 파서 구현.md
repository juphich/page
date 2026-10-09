# pt-0004 Markdown 문서 파서 구현

## 목표

Markdown을 기본 문서 요소로 구조화하고 PAGE fenced block과 정확한 소스 위치를 추출한다.

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

- 작업 진행 중 기록한다.

## 검증

- `cargo test -p page-core`

## 선행 작업

- pt-0002, pt-0003

## 후속 작업

- pt-0005
