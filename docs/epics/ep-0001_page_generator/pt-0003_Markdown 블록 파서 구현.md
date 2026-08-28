# pt-0003 Markdown 블록 파서 구현

## 목표

Markdown에서 PAGE fenced block과 정확한 소스 위치를 추출한다.

## 작업 내용

- Markdown AST 또는 이벤트 기반 파서를 선정한다.
- `page:gantt` fenced block을 식별한다.
- 누락·중복 블록과 구문 위치를 진단한다.

## 완료 조건

- [ ] 정상 fenced block을 내용과 위치 정보로 추출한다.
- [ ] 경계 및 오류 입력에 대한 단위 테스트가 통과한다.

## 결정 사항

- 작업 진행 중 기록한다.

## 검증

- `cargo test -p page-core`

## 후속 작업

- pt-0004
