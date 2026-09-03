# pt-0002 Markdown-HTML 변환 파이프라인 정의

## 목표

Markdown 원문을 독립 실행 가능한 PAGE HTML로 변환하는 전체 처리 단계와 확장 지점을 설계한다. 이 task에서는 실제 모델, 파서와 렌더러를 구현하지 않는다.

## 작업 내용

- Markdown 입력부터 HTML 출력까지의 변환 단계를 정의한다.
- 일반 Markdown 요소와 PAGE 전용 블록의 모델 경계를 정의한다.
- 제목, 문단, 표, 인용구, 목록과 코드 블록 등 기본 Markdown 구성 요소를 분류한다.
- `page:gantt`와 향후 `page:chart`, `page:report`가 같은 확장 구조를 사용하도록 PAGE 블록 계층을 정의한다.
- 공통 노드 메타데이터, 소스 위치, 진단 정보와 속성의 역할을 정의한다.
- 문서 파서, PAGE 블록 파서, 검증기와 HTML 렌더러의 책임을 구분한다.
- 중간 모델, HTML 구조, component CSS와 테마의 책임을 구분한다.
- PAGE가 일반 Markdown HTML에 항상 제공할 기본 색상, 타이포그래피, 간격, 제목·본문·표·인용구·목록·코드와 인쇄 스타일을 정의한다.
- `html/body`, 문서 canvas, header, content, footer와 반응형 레이아웃을 포함한 전체 문서 셸을 정의한다.
- 새 PAGE 블록이 독립적인 parser, model, renderer와 CSS를 추가할 수 있는 확장 단위를 정의한다.
- `page-core` 공개 변환 API의 입력, 출력과 오류 계약을 문서로 정의한다.
- HTML 변환과 PDF 출력의 경계를 정의한다.
- 기본 HTML, CSS와 JavaScript를 내장하는 단일 경량 CLI의 배포 경계를 정의한다.
- 후속 구현에서 사용할 최소 정상 입력과 예상 HTML 출력 시나리오를 정의한다.

## 완료 조건

- [x] Markdown-HTML 변환 단계와 각 모듈의 책임이 설계 문서에 정의된다.
- [x] 기본 Markdown 요소와 PAGE 전용 블록의 추상화 및 확장 경계가 정의된다.
- [x] HTML renderer, component CSS, 테마와 인쇄 스타일의 책임 경계가 정의된다.
- [x] 별도 설정 없이 완성된 문서를 생성하는 PAGE 기본 스타일 계약이 문서로 정의된다.
- [x] PAGE 블록 유무와 관계없이 적용되는 전체 문서 셸과 반응형 레이아웃이 정의된다.
- [x] `page-core` 공개 변환 API의 입력, 출력과 오류 계약이 문서로 정의된다.
- [x] 후속 구현의 완료 조건이 되는 최소 인수 테스트 시나리오가 문서로 작성된다.
- [x] 별도 runtime과 필수 설정이 없는 단일 실행 파일의 구현 원칙이 정의된다.

## 결정 사항

- `page-core`는 Markdown 문자열을 받아 독립 실행 가능한 HTML 문자열을 생성한다.
- 파일 입출력, 출력 경로, 덮어쓰기와 프로세스 종료 코드는 `page-cli`가 담당한다.
- PDF는 생성된 HTML을 로컬 브라우저 인쇄 엔진에 전달하는 별도 단계로 취급한다.
- 기본 runtime은 컴파일 시점에 실행 파일에 내장하고 PDF 브라우저 엔진은 번들하지 않는다.
- 문서는 일반 Markdown 요소와 PAGE 전용 블록을 원래 순서대로 함께 보유한다.
- 일반 Markdown 요소는 `MarkdownBlock`, PAGE 전용 요소는 `PageBlock` 계층으로 분리한다.
- 공통 추상화는 소스 위치, 속성과 진단 정보에 한정하고 Gantt 도메인 모델을 범용 구성 요소로 평탄화하지 않는다.
- PAGE 블록은 종류별 파서와 렌더러를 추가할 수 있는 확장 지점을 갖는다.
- HTML renderer는 의미 있는 요소, class와 `data-*` 속성을 생성하고 시각 표현은 CSS에 위임한다.
- 개발 소스의 CSS는 공통, Markdown, PAGE component, 테마와 인쇄 스타일로 분리한다.
- 최종 산출물에서는 분리된 CSS를 인라인하여 단일 HTML 계약을 유지한다.
- 기본 테마는 선택 사항이 아니며 모든 생성 HTML에 포함한다.
- `page:gantt`가 없는 일반 Markdown 문서에도 base, Markdown, 기본 테마와 인쇄 스타일을 적용한다.
- 사용자 CSS는 생성 옵션으로 전달하며 중간 문서 모델에 파일 경로나 CSS 내용을 저장하지 않는다.
- 기본 stylesheet 전체를 cascade layer에 먼저 포함하고 사용자 CSS를 마지막 unlayered style로 배치해 부분 override를 지원한다.
- 중간 모델에는 CSS 문자열이나 구체적인 시각 속성을 포함하지 않는다.
- 최초 구현은 명시적인 enum 분기를 사용하고, 동적 parser registry나 플러그인 시스템은 실제 필요가 생길 때 도입한다.
- 중간 모델은 Markdown 파서와 HTML 생성기 사이의 내부 계약으로 둔다.
- 전체 변환 단계는 Markdown 구조 분석, PAGE 블록 식별, 종류별 역직렬화와 검증, 중간 모델 생성, 종류별 HTML 렌더링 순서로 구성한다.
- 이번 Epic에서는 정의된 기본 Markdown 요소를 실제 HTML로 렌더링한다. raw HTML과 아직 정의되지 않은 PAGE 블록은 허용하지 않는다.

## 설계 완료 체크리스트

### 공개 API 계약

- [x] `page-core`의 HTML 변환 진입점 이름과 함수 시그니처를 확정한다.
- [x] 변환과 정합성 검사 API를 하나로 구성할지 `compile`과 `validate`로 분리할지 결정한다.
- [x] `CompileOptions`가 소유할 생성 옵션과 `page-cli`가 소유할 파일 옵션을 구분한다.
- [x] 사용자 CSS를 경로가 아닌 이름과 내용으로 `page-core`에 전달하는 타입을 정의한다.
- [x] 성공 결과, 경고와 실패 오류를 반환하는 타입 및 관계를 정의한다.

### 파이프라인 단계와 타입 경계

- [x] Markdown 원문, Markdown 구조, raw PAGE block, 검증된 중간 모델과 HTML 출력 단계를 명명한다.
- [x] 각 단계의 입력·출력 타입과 담당 모듈을 정의한다.
- [x] 소스 위치가 Markdown 파싱부터 진단 생성까지 어떻게 유지되는지 정의한다.
- [x] 파싱, 정규화, 검증과 렌더링 중 어느 단계가 기본값 및 자동 ID를 결정하는지 확정한다.
- [x] `--dry-run`이 어느 단계까지 실행되고 어떤 산출물을 생략하는지 확정한다.

### 일반 Markdown 처리 범위

- [x] ep-0001에서 일반 Markdown 본문을 실제 HTML로 렌더링할지 확장 구조만 제공할지 최종 확정한다.
- [x] 지원할 기본 Markdown 요소와 그대로 보존하거나 거부할 요소를 구분한다.
- [x] 일반 fenced code block, 알 수 없는 `page:*` 블록과 알 수 없는 언어 식별자의 처리 방식을 정의한다.
- [x] Markdown 문서 제목을 추출하는 규칙과 명시적인 Gantt 제목과의 관계를 정의한다.

### 오류와 진단 계약

- [x] Markdown, PAGE 문법, 모델 검증, HTML 생성, CSS 안전성과 파일 I/O 오류의 진단 코드 영역을 최종 확인한다.
- [x] 즉시 중단하는 오류와 가능한 범위에서 수집하는 오류를 단계별로 구분한다.
- [x] 오류와 경고에 포함할 파일명, line, column, block 종류와 항목 ID 규칙을 정의한다.
- [x] 여러 PAGE 블록에서 발생한 진단의 정렬 순서와 최대 수집 개수 정책을 결정한다.
- [x] `--style` 파일 읽기 오류와 CSS 내용 오류가 어떤 계층에서 생성되는지 확정한다.

### 내장 리소스와 HTML 조립 계약

- [x] 바이너리에 내장할 HTML, CSS와 JavaScript 파일 목록을 확정한다.
- [x] `include_str!` 선언 위치와 runtime 자산의 소유 crate를 확정한다.
- [x] HTML 셸의 필수 삽입 지점과 각 삽입 지점이 한 번만 존재해야 한다는 조건을 정의한다.
- [x] 기본 CSS, component CSS, 테마, print CSS와 사용자 CSS의 인라인 및 cascade layer 순서를 확정한다.
- [x] 제목, JSON, CSS와 JavaScript를 HTML에 삽입할 때 적용할 escaping 및 금지 입력 규칙을 정의한다.
- [x] 동일한 입력과 사용자 CSS에서 바이트 단위로 동일한 HTML을 만드는 정규화 규칙을 확정한다.

### 최소 인수 시나리오

- [x] 일반 Markdown만 포함한 최소 정상 입력과 예상 HTML 구조를 작성한다.
- [x] 평면 task를 가진 `page:gantt` 입력과 예상 HTML·JSON 구조를 작성한다.
- [x] milestone, epic과 task 계층 입력의 예상 정규화 결과를 작성한다.
- [x] 기본 스타일만 사용하는 결과와 부분 사용자 CSS를 적용한 결과를 정의한다.
- [x] 여러 `--style` 파일의 적용 순서와 선언하지 않은 기본 스타일 보존 시나리오를 작성한다.
- [x] `--dry-run` 성공·실패 시 파일이 생성되지 않는 시나리오를 작성한다.
- [x] 잘못된 Markdown, PAGE 데이터와 사용자 CSS의 대표 실패 시나리오를 작성한다.
- [x] HTML과 PDF가 동일한 기본·사용자 스타일을 사용하는 시나리오를 작성한다.

### 완료 정리

- [x] 공개 API, CLI, 문법, 스타일과 Epic 설계 문서 사이의 용어 및 책임이 일치하는지 검토한다.
- [x] 확정된 결정 사항을 이 문서와 `design.md`에 반영한다.
- [x] 완료 조건과 설계 체크리스트를 모두 확인한 뒤 `plan.md`에서 pt-0002를 완료 처리한다.

## 검증

- `docs/syntax.md`, `docs/cli.md`, `docs/style.md`, Epic 설계와 task 경계의 일관성을 검토했다.
- 공개 API, 파이프라인, Markdown 범위, 진단, runtime 조립과 인수 시나리오를 Epic 설계에 반영했다.
- `git diff --check`

## 선행 작업

- pt-0001

## 후속 작업

- pt-0003, pt-0004, pt-0006
