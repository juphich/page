# PAGE

**Portable Authoring for Graphs & Exports**

Markdown을 Rust 기반 문서 생성기로 단일 PAGE HTML로 변환하고, 설치·서버·빌드 없이 브라우저에서 차트와 보고서를 편집·재저장하는 독립형 리포트 캔버스입니다.

현재 첫 번째 보고서 유형은 **간트차트**입니다. 산출물은 언제나 단일 `.html` 파일 하나이며, 그 파일 안에 데이터·뷰어·에디터가 모두 들어 있어 별도 도구 없이 다시 열어 편집하고 저장할 수 있습니다.

> A standalone, self-editing report canvas.

## 핵심 아이디어

내보낸 HTML이 **자기 자신을 편집·재저장**한다. 파일 안에 편집 대상(데이터 JSON) + 편집 도구(뷰어·에디터 JS)가 함께 들어 있어, 파일을 열면 곧 편집기이자 결과물이 된다. 서버·런타임·설치가 필요 없다.

## 빠른 시작

### 최초 문서 생성

Rust 기반 생성기가 Markdown 원본의 page:gantt, page:chart, page:report 블록과 Mermaid 블록을 해석해 최초 PAGE HTML 문서를 생성합니다.

    Markdown(.md) → Rust 문서 생성기 → PAGE 문서(.html)

생성된 HTML은 생성기 없이 브라우저에서 열어 편집하고 다시 저장할 수 있습니다.

```bash
open src/gantt.html          # macOS. 그 외에는 브라우저로 파일을 열면 됨
```

1. **✏️ 편집모드** — 막대를 드래그해 이동, 양끝을 잡아 기간 조절. 아래 표에서 이름·날짜·진행률·색상 편집, 태스크 추가/삭제. 제목도 바로 수정.
2. **💾 HTML로 저장** — 현재 상태로 완전한 HTML을 재조립해 저장. Chromium 계열은 같은 파일 덮어쓰기, 그 외는 새 파일 다운로드로 자동 폴백.
3. **🖨 PDF 출력** — 인쇄 대화상자에서 "PDF로 저장"(가로).

저장한 파일을 다시 열면 편집 내용이 그대로 살아 있고 다시 편집·저장할 수 있다.

## 주요 요구사항

1. Markdown 파일을 Rust 생성기에 전달해 최초 PAGE HTML 문서를 생성한다.
2. page:gantt, page:chart, page:report와 Mermaid 블록을 파싱한다.
3. 생성된 문서는 JavaScript를 자체 내장해 편집모드로 전환·편집할 수 있다.
4. 편집은 별도 전용 툴 없이 브라우저 로컬에서 가능하다.
5. 온전한 PAGE 문서는 PDF로 출력할 수 있다.

## 문서

- [docs/requirements.md](docs/requirements.md) — 요구사항 상세·범위·수용 기준
- [docs/design.md](docs/design.md) — 아키텍처·자기 편집 문서 패턴·저장 전략·데이터 모델·로드맵

## 저장소 구조

```
page-canvas/
├── README.md
├── docs/
│   ├── requirements.md
│   └── design.md
└── src/
    └── gantt.html        # 단일 파일 프로토타입 (배포 산출물이자 편집기)
```

## 현재 상태

동작하는 브라우저 편집기 프로토타입 단계. Rust 기반 Markdown 문서 생성기와 page:* 블록 파서는 설계 단계이다. 간트 막대 렌더링·드래그/리사이즈 편집·표 편집·자기 재저장·PDF 인쇄까지 동작한다. 의존관계 화살표, 마일스톤, 긴 일정의 PDF 페이지 분할, undo는 미구현 — [docs/design.md](docs/design.md)의 로드맵 참고.
