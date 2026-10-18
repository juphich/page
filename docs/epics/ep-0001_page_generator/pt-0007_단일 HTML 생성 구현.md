# pt-0007 단일 HTML 생성 구현

## 목표

PAGE 중간 모델과 런타임을 안전한 단일 HTML 문자열로 생성한다.

## 작업 내용

- Gantt 모델을 사람이 읽을 수 있는 JSON으로 직렬화한다.
- `GanttItem` 계층과 task 상태를 직렬화한다.
- 기본 Markdown 요소와 Gantt component를 문서의 원래 순서대로 HTML에 렌더링한다.
- 문서 제목과 JSON을 안전하게 템플릿에 삽입한다.
- heading마다 제목 텍스트 기반의 고유한 `id`를 생성한다.
- Gantt block마다 JSON data block을 생성하고 component와 `data-page-data`로 연결한다.
- 분리된 HTML, component CSS, 테마, 인쇄 CSS와 JavaScript 자산을 인라인한다.
- 사용자 CSS를 지정 순서대로 layered 기본 스타일 뒤의 별도 unlayered style 요소에 안전하게 인라인한다.
- 외부 리소스가 없고 동일 입력에 결정적인 출력을 만든다.
- `src`가 없는 이미지(`Inline::Image { src: None }`)는 `Image not supported yet` 문구와 alt를 담은 빈 박스로 렌더링한다. 박스 모양은 CSS class로 정해 사용자 CSS로 바꿀 수 있게 한다.
- 생성기가 중간 모델을 사용하면 `page-core`의 `document`, `gantt` 모듈에 둔 임시 `#![allow(dead_code)]`와 `document/mod.rs`, `gantt/mod.rs` 재내보내기의 임시 `#[allow(unused_imports)]`를 제거한다.

## 완료 조건

- [ ] 생성 HTML에 모든 데이터와 런타임이 인라인된다.
- [ ] PAGE 블록이 없는 일반 Markdown 문서도 완전한 HTML로 생성된다.
- [ ] 개발 소스에서 분리된 모든 CSS가 결정적인 순서로 인라인된다.
- [ ] 별도 테마 옵션 없이도 기본 테마가 항상 인라인된다.
- [ ] 부분 사용자 CSS가 선언한 속성만 override하고 나머지는 기본 스타일을 유지한다.
- [ ] 여러 사용자 CSS의 지정 순서가 결과에 반영된다.
- [ ] `</script>` 등 특수 입력이 문서 구조를 훼손하지 않는다.
- [ ] 동일 입력의 출력이 바이트 단위로 동일하다.

## 결정 사항

- 파생 진행률은 저장하지 않고 런타임이 task 상태로 계산한다.
- CSS 인라인 순서는 base, Markdown, component, theme, print, user 순서로 고정한다.
- 최종 HTML은 외부 stylesheet를 참조하지 않는다.
- 기본 스타일 누락은 생성 오류로 처리한다.
- 사용자 CSS는 별도 unlayered style 요소에 포함하고 `@import`와 `<style>` 종료 입력을 거부한다.

## 검증

- `cargo test -p page-core`

## 선행 작업

- pt-0003, pt-0004, pt-0005, pt-0006

## 후속 작업

- pt-0008, pt-0010
