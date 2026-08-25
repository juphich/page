# ep-0001 Rust 문서 생성기 MVP

## 목적

Markdown의 `page:gantt` 블록을 파싱해 독립 실행 가능한 단일 PAGE HTML로 변환하는 Rust CLI를 완성한다.

## 배경

현재 저장소에는 브라우저에서 동작하는 간트 HTML 프로토타입과 Rust workspace 골격만 있다. PAGE 문서를 최초로 생성하는 진입점이 없으므로 문법, 중간 모델, 파서, 생성기와 CLI를 하나의 수직 기능으로 구현해야 한다.

## 범위

- `page:gantt` 블록과 CLI 계약 정의
- PAGE 문서 및 Gantt 중간 모델 정의
- Markdown fenced block 추출과 Gantt 데이터 검증
- 기존 간트 프로토타입의 런타임 템플릿 이관
- 안전하고 결정적인 단일 HTML 생성
- Markdown 파일 입력과 HTML 파일 출력 CLI
- 단위 및 통합 테스트와 예제 문서

## 제외 범위

- 생성된 HTML의 편집·저장 기능 고도화
- 일반 Markdown 본문과 `page:report`, `page:chart` 렌더링
- Mermaid 호환
- 긴 일정의 PDF 페이지 분할

## 완료 조건

- [ ] `page input.md -o output.html` 명령이 성공한다.
- [ ] 유효한 `page:gantt` 입력이 독립 실행 가능한 단일 HTML로 생성된다.
- [ ] 잘못된 입력은 원인을 식별할 수 있는 오류로 거부된다.
- [ ] 동일한 입력은 동일한 HTML을 생성한다.
- [ ] 생성기 단위·통합 테스트가 모두 통과한다.

