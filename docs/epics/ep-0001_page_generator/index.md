# ep-0001 Rust 문서 생성기 MVP

## 목적

Markdown의 `page:gantt` 블록을 파싱해 독립 실행 가능한 PAGE HTML 또는 PDF로 변환하고 입력 정합성을 검사하는 Rust CLI를 완성한다.

CLI는 기본 HTML 셸, CSS와 JavaScript runtime을 내장한 단일 경량 실행 파일로 제공하며 별도 runtime이나 필수 설정 없이 기본 변환을 수행한다.

## 배경

현재 저장소에는 브라우저에서 동작하는 간트 HTML 프로토타입과 Rust workspace 골격만 있다. PAGE 문서를 최초로 생성하는 진입점이 없으므로 문법, 중간 모델, 파서, 생성기와 CLI를 하나의 수직 기능으로 구현해야 한다.

## 범위

- `page:gantt` 블록과 CLI 계약 정의
- 평면 task와 선택적 milestone·epic 계층을 지원하는 PAGE 및 Gantt 중간 모델 정의
- 기본 Markdown 문서 구조화, PAGE fenced block 추출과 Gantt 데이터 검증
- 기본 Markdown 요소의 안전한 HTML 렌더링
- 기존 간트 프로토타입의 런타임 템플릿 이관
- 안전하고 결정적인 단일 HTML 생성
- Markdown 정합성 검사와 HTML/PDF 출력 CLI
- 단위 및 통합 테스트와 예제 문서

## 제외 범위

- 생성된 HTML의 편집·저장 기능 고도화
- `page:report`, `page:chart`와 Mermaid 렌더링
- raw HTML 허용과 sanitization
- 긴 일정의 PDF 페이지 분할
- 파일·인터넷 이미지를 HTML에 넣는 이미지 asset 처리 (`docs/backlogs/백로그_이미지 asset 처리.md`)

## 완료 조건

- [ ] `page input.md -o output.html` 명령이 성공한다.
- [ ] `page input.md --format pdf -o output.pdf` 명령이 성공한다.
- [ ] `page input.md --dry-run`이 파일 생성 없이 정합성을 검사한다.
- [ ] `page input.md --style custom.css`가 기본 스타일을 유지하면서 사용자 CSS의 선언만 override한다.
- [ ] 유효한 `page:gantt` 입력이 독립 실행 가능한 단일 HTML로 생성된다.
- [ ] 일반 Markdown만 있는 입력이 기본 스타일을 갖는 단일 HTML로 생성된다.
- [ ] 차트가 평면 task 목록과 선택적인 milestone·epic 계층을 표시한다.
- [ ] 존재하는 Epic과 milestone의 진행률이 하위 완료 task 수로 계산된다.
- [ ] 잘못된 입력은 원인을 식별할 수 있는 오류로 거부된다.
- [ ] 동일한 입력은 동일한 HTML을 생성한다.
- [ ] 생성기 단위·통합 테스트가 모두 통과한다.
- [ ] release 실행 파일 하나만으로 기본 HTML 생성이 동작하며 별도 runtime 파일을 요구하지 않는다.
- [ ] release 바이너리 크기와 주요 의존성이 측정·기록되고 불필요한 대형 runtime을 포함하지 않는다.
