# PAGE 기본 스타일 계약

## 목적

PAGE가 생성하는 모든 문서에 별도 설정 없이 적용되는 기본 시각 체계를 정의한다. 기본 스타일은 선택 기능이 아니라 생성 문서의 가독성, 일관성과 인쇄 품질을 보장하는 필수 산출물이다.

## 원칙

- 생성된 모든 PAGE HTML은 기본 테마를 포함한다.
- 네트워크가 없어도 동일하게 표시되도록 외부 폰트와 stylesheet에 의존하지 않는다.
- 중간 모델은 CSS 문자열이나 구체적인 색상값을 소유하지 않는다.
- HTML은 의미 있는 class와 `data-*` 속성을 제공하고 CSS가 화면 표현을 결정한다.
- 화면과 인쇄에서 정보의 계층, 상태와 표 구조가 식별되어야 한다.
- 색상만으로 상태나 계층을 구분하지 않는다.
- 본문과 주요 UI 텍스트는 WCAG AA 수준의 대비를 목표로 한다.

## 기본 디자인 토큰

기본 테마는 다음 CSS custom property를 기준으로 한다. 구체적인 변수명은 구현 과정에서 세분화할 수 있지만 의미와 기본값은 유지한다.

```css
:root {
  --page-color-canvas: #f0f2f5;
  --page-color-surface: #ffffff;
  --page-color-text: #1c2333;
  --page-color-muted: #6b7280;
  --page-color-border: #e5e7eb;
  --page-color-subtle: #f7f8fa;
  --page-color-accent: #4f7cff;
  --page-color-danger: #d64545;
  --page-color-today: #ff4d4f;

  --page-status-not-started: #64748b;
  --page-status-in-progress: #2563eb;
  --page-status-completed: #16a34a;
  --page-status-on-hold: #d97706;

  --page-font-sans: -apple-system, BlinkMacSystemFont,
    "Apple SD Gothic Neo", "Malgun Gothic", "Segoe UI", sans-serif;
  --page-font-mono: ui-monospace, "SFMono-Regular", Consolas, monospace;

  --page-space-1: 4px;
  --page-space-2: 8px;
  --page-space-3: 12px;
  --page-space-4: 16px;
  --page-space-6: 24px;
  --page-space-8: 32px;

  --page-radius-small: 6px;
  --page-radius-medium: 8px;
  --page-radius-large: 12px;
  --page-content-width: 1200px;
}
```

## 전체 문서 셸

모든 PAGE HTML은 콘텐츠 종류와 관계없이 동일한 최상위 문서 셸과 기본 스타일을 사용한다.

```html
<body class="page-shell">
  <div class="page-document">
    <header class="page-document__header">...</header>
    <main class="page-document__content">...</main>
    <footer class="page-document__footer">...</footer>
  </div>
</body>
```

- `html`과 `body`는 기본 글꼴, 글자색, 배경색과 box sizing 기준을 제공한다.
- `page-shell`은 화면 전체의 canvas, 최소 높이와 문서 배치를 담당한다.
- `page-document`는 최대 콘텐츠 폭, surface, 경계, 모서리와 기본 여백을 담당한다.
- `page-document__header`는 문서 제목과 메타데이터 영역을 제공한다.
- `page-document__content`는 Markdown 블록과 PAGE 전용 블록을 원문 순서대로 배치한다.
- `page-document__footer`는 선택 영역이며 존재하지 않을 때 빈 공간을 만들지 않는다.
- 화면 폭이 좁아지면 외부 여백과 내부 padding을 줄이고 표와 차트처럼 넓은 구성 요소만 내부 스크롤을 허용한다.
- 특정 component가 문서 전체의 배경, 글꼴 또는 콘텐츠 폭을 임의로 덮어쓰지 않는다.
- PAGE 전용 블록이 없어도 전체 문서 셸과 기본 스타일은 동일하게 적용한다.

## 기본 문서 스타일

기본 Markdown 변환기가 생성하는 HTML에는 `markdown.css`와 기본 테마를 항상 적용한다. 이 규칙은 문서에 `page:gantt` 등 PAGE 전용 블록이 하나도 없는 경우에도 동일하다.

- 본문은 시스템 sans-serif 글꼴, `16px` 기준 크기와 `1.6` 행간을 사용한다.
- 문서 영역은 최대 `1200px`이며 좁은 화면에서는 좌우 `16px` 여백을 유지한다.
- 제목은 단계별 크기와 굵기로 문서 구조를 명확하게 표현한다.
- 문단 사이에는 일관된 수직 간격을 적용한다.
- 링크는 본문과 구분되는 accent 색과 밑줄 또는 동등한 비색상 표지를 사용한다.
- 표는 헤더 배경, 셀 경계와 충분한 셀 여백을 제공하며 좁은 화면에서는 가로 스크롤을 허용한다.
- 인용구는 왼쪽 경계선과 들여쓰기를 사용하고 본문 대비를 유지한다.
- 인라인 코드와 코드 블록은 monospace 글꼴 및 본문과 구분되는 배경을 사용한다.
- 키보드 포커스는 모든 조작 요소에서 명확하게 보여야 한다.

기본 Markdown 요소와 HTML 의미 요소의 대응은 다음을 기준으로 한다.

| Markdown 요소 | HTML 의미 요소 | 기본 스타일 책임 |
|---|---|---|
| 문서 제목과 제목 | `h1`–`h6` | 단계별 크기, 굵기와 여백 |
| 본문 | `p` | 읽기 폭, 행간과 문단 간격 |
| 링크 | `a` | accent 색, 밑줄과 focus 표시 |
| 목록 | `ul`, `ol`, `li` | 들여쓰기와 항목 간격 |
| 인용구 | `blockquote` | 경계선, 들여쓰기와 muted 표현 |
| 표 | `table`, `thead`, `tbody`, `th`, `td` | 헤더, 경계, 셀 간격과 overflow |
| 인라인 코드 | `code` | monospace 글꼴과 subtle 배경 |
| 코드 블록 | `pre > code` | 여백, overflow와 가독성 |
| 구분선 | `hr` | 문서 구획을 나타내는 경계 |
| 이미지 | `img` | 콘텐츠 폭 제한과 비율 유지 |

## 기본 Gantt 스타일

- timeline의 기본 하루 폭은 `30px`, task 행 높이는 `38px`, 헤더 높이는 `46px`로 한다.
- 사이드바 기본 폭은 `200px`로 하고 긴 이름은 말줄임 표시하되 전체 값을 확인할 수 있어야 한다.
- 주말은 subtle 배경으로, 오늘은 별도의 선과 텍스트로 표시한다.
- task 상태는 색상과 함께 class, 레이블 또는 패턴 등 비색상 정보로도 구분한다.
- milestone, epic과 task는 들여쓰기, 글꼴 굵기와 도형을 조합해 계층을 표현한다.
- 막대와 표의 상태 표현은 동일한 상태 토큰을 사용한다.
- 편집 UI는 보기 콘텐츠와 시각적으로 구분하고 인쇄 결과에서는 제외한다.

## 인쇄 및 PDF 스타일

- 기본 용지는 가로 방향이며 여백은 `12mm`로 한다.
- 인쇄 배경은 흰색으로 정규화한다.
- toolbar, 편집 입력, resize handle과 toast 등 조작 UI를 숨긴다.
- 차트 색상은 가능한 범위에서 정확히 인쇄되도록 `print-color-adjust`를 사용한다.
- 표의 헤더, 경계, 차트 행과 상태 표지는 흑백 출력에서도 식별 가능해야 한다.
- 행과 task 막대는 가능한 범위에서 페이지 중간 분할을 피한다.

## 스타일 자산과 조립 순서

개발 소스에서는 역할별 CSS 파일로 관리하고 최종 HTML에서는 다음 순서로 인라인한다.

```text
base.css
→ markdown.css
→ component CSS (gantt.css 등)
→ theme-default.css
→ print.css
→ 사용자 CSS (`--style` 지정 순서)
```

기본 테마는 항상 포함한다. 향후 다른 테마를 지원하더라도 기본 테마 없이 문서를 생성하지 않는다.

## 사용자 CSS override

사용자는 문서를 생성할 때 `--style <PATH>`로 CSS 파일을 추가할 수 있다. 이 옵션은 반복 가능하며 지정한 순서대로 최종 HTML에 인라인한다.

```bash
page project.md -o project.html --style ./company.css
page project.md -o project.html --style ./brand.css --style ./report.css
```

PAGE 기본 stylesheet는 다음 cascade layer 순서를 고정한다.

```css
@layer page-reset, page-base, page-markdown, page-components, page-theme, page-print;
```

기본 stylesheet는 자신의 layer에 배치하고 사용자 CSS는 생성기가 마지막의 별도 unlayered style 요소에 배치한다. 일반적인 author style에서 unlayered 선언은 layered 기본 선언보다 우선한다. 기본 stylesheet에서는 `!important`를 사용하지 않는다. 이에 따라 사용자 CSS는 전체 스타일을 복사하지 않고 필요한 변수나 속성만 선언해 override할 수 있으며, 선언하지 않은 모든 값은 PAGE 기본 스타일을 유지한다.

```css
/* company.css */
:root {
  --page-color-accent: #7c3aed;
  --page-content-width: 1440px;
}

.page-document h1 {
  letter-spacing: -0.025em;
}
```

여러 사용자 CSS가 같은 속성을 같은 조건으로 선언하면 나중에 지정한 파일의 선언이 우선한다. 일반적인 CSS cascade 규칙인 origin, importance와 선언 순서는 유지된다.

- base, Markdown, 기본 테마와 print로 구성된 문서 기본 bundle은 사용자 CSS 사용 여부와 관계없이 항상 전부 포함한다.
- Gantt 등 component CSS는 해당 component가 문서에 있을 때만 기본 bundle과 사용자 CSS 사이에 포함한다.
- 사용자 CSS의 파일 경로는 생성 결과에 남기지 않고 내용만 인라인한다.
- `@import`와 `<style>` 종료 문자열은 허용하지 않는다.
- 외부 파일을 참조하는 asset URL은 단일 파일 보장을 위해 MVP에서 지원하지 않는다.
- 사용자 CSS의 `@media print` 규칙은 기본 인쇄 스타일을 override할 수 있다.
- 동일한 Markdown, 기본 runtime과 사용자 CSS 내용 및 순서는 동일한 HTML을 생성해야 한다.

## 검증 기준

- 생성 문서는 `html`, `body`, 문서 canvas와 콘텐츠 영역에 정의된 기본 스타일을 갖는다.
- 기본 옵션으로 생성된 HTML에 기본 테마와 필요한 component CSS가 포함된다.
- PAGE 전용 블록이 없는 일반 Markdown 문서에도 `base.css`, `markdown.css`, `theme-default.css`와 `print.css`가 포함된다.
- 부분 사용자 CSS를 적용해도 선언하지 않은 문서 및 component 스타일은 기본값을 유지한다.
- 여러 `--style` 파일은 지정 순서대로 override된다.
- 생성 HTML에 외부 stylesheet나 웹 폰트 요청이 없다.
- 기본 Markdown 요소와 Gantt 구성 요소에 정의되지 않은 무스타일 상태가 없다.
- 좁은 화면에서 본문과 표를 읽을 수 있으며 차트는 데이터가 잘리지 않도록 스크롤할 수 있다.
- 인쇄 미리보기와 PDF에서 편집 UI가 제외되고 문서 구조와 상태가 식별된다.
