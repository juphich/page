# 백로그 이미지 asset 처리

## 배경

ep-0001은 이미지를 구현하지 않는다. `![로고](./images/logo.png)` 등 모든 이미지는 `Image not supported yet` 문구와 alt를 담은 빈 박스로 표시된다.

## 목표

Markdown 이미지가 어떤 주소로 작성되어도 하나의 공통 처리 경로를 거쳐 단일 HTML 안에 `data:image` URI로 들어가게 한다.

## 범위

- 이미지 내용을 가져오는 공통 `AssetResolver` 인터페이스. `page-core`는 파일이나 네트워크를 직접 읽지 않고 요청만 한다.
- `data:image` URI는 그대로 통과시키고 그 밖의 주소는 resolver로 받아 처리하는 공통 이미지 처리
- 받은 바이트로 이미지 형식(PNG, JPEG, GIF, WebP, SVG)을 확인하고 `DataImageUri`로 변환
- 가져오기·형식 확인 실패의 진단 코드와 원인별 메시지
- `CompileOptions`에 resolver를 넘기는 공개 API 확장
- `page-cli`의 md 파일 폴더 기준 상대 파일 resolver와 테스트용 메모리 resolver
- `docs/syntax.md`, ep-0001 design.md의 이미지 규칙 갱신

## 결정 필요

- 인터넷 이미지 처리: 오류와 파일 저장 안내 / 주소 유지 / 변환 시 내려받기
- 상대 경로의 범위: md 파일 폴더 밖(`../`)과 절대 경로 허용 여부
- 이미지 크기 상한과 SVG 허용 범위
- 사용자 CSS의 `url()` 이미지·폰트, 첨부 파일 등 다른 asset까지 같은 인터페이스로 다룰지

## 기대 결과

- 상대 파일 이미지가 단일 HTML 안에 들어가고 HTML 파일 하나만 보내도 이미지가 보인다.
- 이미지 출처가 늘어나도 resolver 구현만 추가하면 된다.
- 처리에 성공한 이미지는 `Inline::Image.src`에 `DataImageUri`가 들어가고, 미구현 빈 박스는 실패나 미지원 출처에만 남는다.

## 참고

- ep-0001 pt-0004 파서는 모든 이미지를 `Inline::Image { src: None }`으로 만든다. 이 기능을 구현하면 파서가 이미지를 만나는 즉시 공통 이미지 처리를 호출해 `src`를 채우도록 바꾼다. 실패 진단에 그 자리의 원본 위치를 바로 붙일 수 있으므로 이미지 참조의 위치를 따로 보존하지 않아도 된다.
- 2026-10-18 pt-0004 설계 중 pt-0011로 분리했다가 ep-0001 범위에서 제외해 backlog로 옮겼다.
