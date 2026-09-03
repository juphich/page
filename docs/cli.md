# PAGE CLI 계약

## 명령 형식

```text
page [OPTIONS] <INPUT>
```

```bash
page project.md
page project.md -o project.html
page project.md -o project.html --style ./company.css
page project.md -o project.html --style ./brand.css --style ./report.css
page project.md --format pdf -o project.pdf
page project.md --dry-run --style ./company.css
```

## 인자와 옵션

| 항목 | 설명 |
|---|---|
| `<INPUT>` | UTF-8 Markdown 입력 파일 |
| `-o, --output <PATH>` | 출력 파일 경로 |
| `-F, --format <FORMAT>` | 출력 형식: `html`, `pdf` (기본값: `html`) |
| `-s, --style <PATH>` | 기본 스타일 뒤에 적용할 사용자 CSS 파일. 여러 번 지정 가능 |
| `-f, --force` | 기존 출력 파일 덮어쓰기 |
| `--dry-run` | 입력 문법, 모델 정합성과 사용자 CSS를 검사하고 출력하지 않음 |
| `-h, --help` | 도움말 출력 |
| `-V, --version` | 버전 출력 |

출력 경로를 생략하면 입력 파일의 마지막 확장자를 선택한 형식의 `.html` 또는 `.pdf`로 바꾼다. 확장자가 없는 입력에는 해당 확장자를 붙인다. 명시한 출력 경로의 확장자는 선택한 형식과 일치해야 한다. 입력 확장자는 제한하지 않는다.

입력과 출력 경로가 같으면 오류로 처리한다. 출력의 상위 디렉토리는 미리 존재해야 하며 CLI가 생성하지 않는다. stdin과 stdout 출력은 MVP 범위에서 제외한다.

`--style`은 지정 순서대로 처리한다. PAGE의 완전한 layered 기본 스타일은 항상 포함하며 사용자 CSS는 마지막의 별도 unlayered style 요소에서 자신이 선언한 속성만 override한다. 따라서 사용자 CSS가 전체 PAGE 스타일을 다시 정의할 필요가 없다. 사용자 CSS의 경로나 파일명은 결과 HTML에 기록하지 않고 CSS 내용만 인라인한다.

상대 CSS 경로는 CLI를 실행한 현재 작업 디렉토리를 기준으로 해석한다. 읽을 수 없거나 UTF-8이 아닌 CSS 파일은 오류로 처리한다. 사용자 CSS에서 외부 stylesheet를 다시 불러오는 `@import`는 단일 파일·오프라인 계약을 위반하므로 허용하지 않는다. HTML의 `<style>` 요소를 종료할 수 있는 입력도 오류로 처리한다.

`--dry-run`은 `--output`, `--format`, `--force`와 함께 사용할 수 없지만 `--style`과는 함께 사용할 수 있다. 입력과 CSS를 읽어 Markdown·YAML 구문, PAGE 모델 정합성과 사용자 CSS 안전성까지 검사하지만 HTML/PDF 생성, 브라우저 탐지와 파일 쓰기는 수행하지 않는다.

## 실행 규칙

1. 입력 파일 전체를 UTF-8로 읽는다.
2. `--style`로 지정한 CSS 파일을 순서대로 읽고 안전성을 검사한다.
3. Markdown을 파싱하고 PAGE 모델을 검증한다.
4. `--dry-run`이면 진단을 출력하고 종료한다.
5. 기본 스타일과 사용자 CSS가 포함된 완전한 단일 HTML을 메모리에서 생성한다.
6. PDF 형식이면 동일한 HTML을 브라우저 인쇄 엔진으로 PDF 변환한다.
7. 모든 생성 단계가 성공한 뒤 출력 파일과 같은 디렉토리에 임시 파일을 쓴다.
8. 임시 파일 쓰기가 완료되면 최종 출력 경로로 교체한다.

기존 출력 파일은 `--force` 없이 덮어쓰지 않는다. `--force`를 사용해도 새 HTML이 완성되기 전에는 기존 파일을 변경하지 않는다. 실패 시 임시 파일을 정리하고 기존 출력 파일을 보존한다.

HTML 출력은 UTF-8, LF 줄바꿈과 마지막 개행 하나를 사용한다. 생성 시각, 절대 입력 경로와 임의 값을 넣지 않아 동일한 Markdown, runtime, 사용자 CSS 내용과 지정 순서는 바이트 단위로 동일한 HTML을 생성한다. 생성된 HTML에는 외부 네트워크 런타임 의존성을 포함하지 않는다.

PDF 출력은 로컬의 Chromium 계열 브라우저 인쇄 엔진을 사용한다. 지원 브라우저를 찾을 수 없거나 변환에 실패하면 `PAGE3xx` 오류로 종료하며 HTML이나 PDF를 남기지 않는다. PDF의 바이트 동일성은 브라우저 버전에 따라 달라질 수 있으므로 보장하지 않는다.

## 종료 코드

| 코드 | 의미 |
|---:|---|
| `0` | 성공 |
| `1` | 입력, 파싱, 검증, 생성 또는 파일 I/O 실패 |
| `2` | 잘못된 CLI 사용법 |

## 진단 형식

진단은 stderr에 출력한다.

```text
project.md:8:5: error[PAGE202]: end must not be earlier than start
```

- MVP 진단 메시지는 영어로 고정한다.
- `PAGE0xx`는 공통 보고 제한, `PAGE1xx`는 Markdown·YAML 구문, `PAGE2xx`는 모델 검증, `PAGE3xx`는 HTML 생성, `PAGE4xx`는 파일 I/O 오류에 사용한다.
- 사용자 CSS의 금지된 내용은 `PAGE3xx`, CSS 파일 읽기와 UTF-8 오류는 `PAGE4xx`로 진단한다.
- Markdown 입력 진단을 먼저 표시하고 사용자 CSS 진단은 `--style` 지정 순서대로 표시한다. 같은 source 안에서는 line, column, severity와 code 순서로 안정적으로 정렬한다.
- 진단은 최대 100개이며 제한을 넘으면 앞의 99개와 생략 사실을 알리는 마지막 진단을 표시한다.
- YAML을 해석할 수 없는 오류는 즉시 중단한다. 모델 검증 오류는 독립적으로 확인 가능한 항목을 소스 순서로 모두 수집한다.
- 복구할 수 없거나 출력 의미가 달라지는 문제는 오류로 처리하고 HTML을 생성하지 않는다.
- 안전한 기본값을 적용할 수 있는 정보성 문제만 경고로 처리한다. 경고만 있으면 요청한 출력을 생성하고 종료 코드 `0`을 반환한다.
- `--dry-run`은 정합성 오류가 없으면 `0`, 오류가 있으면 `1`을 반환한다.
- 사용자 취소나 대화형 입력은 MVP CLI에 없다.
