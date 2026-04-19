# 🦀 Rust 학습 로드맵 (Comprehensive Rust 기준)

이 로드맵은 Google의 [Comprehensive Rust](https://google.github.io/comprehensive-rust/ko/index.html) 커리큘럼 순서를 따르며, 현재 프로젝트(`LearningRust`)를 어떻게 발전시킬지에 대한 구체적인 태스크를 포함합니다.

---

## 📅 1단계: 소유권과 메모리 관리의 심화 (Day 1 ~ 2)
현재 코드에서 기본적인 소유권은 잘 활용하고 계시지만, 가장 어려운 **수명(Lifetimes)**과 **스마트 포인터**에 익숙해질 차례입니다.

*   **학습 내용:**
    *   [참조와 수명(Lifetimes)](https://google.github.io/comprehensive-rust/ko/control-flow/lifetimes.html)
    *   [복합 타입(Structs, Enums, Pattern Matching)](https://google.github.io/comprehensive-rust/ko/structs-and-enums.html)
    *   [스마트 포인터 (`Box`, `Rc`, `RefCell`)](https://google.github.io/comprehensive-rust/ko/smart-pointers.html)
*   **실습 태스크 (LearningRust 프로젝트):**
    *   **Lifetimes 도입**: `findprime`의 결과를 바로 출력하지 않고, 참조를 포함하는 구조체에 저장해보기.
    *   **Enums 활용**: `guessing_game`에서 게임 상태(진행중, 승리, 종료)를 `enum`으로 관리하고 `match`로 처리하기.
    *   **Box 사용**: `Q<T>` 구조체 내부 요소를 `Box`로 감싸서 힙 메모리 할당 연습하기.

---

## 📅 2단계: 트레이트와 제네릭 시스템 (Day 3)
이미 `Q<T>`에서 제네릭을 사용하셨지만, Rust의 강력함은 **표준 트레이트**의 구현에서 나옵니다.

*   **학습 내용:**
    *   [트레이트(Traits)](https://google.github.io/comprehensive-rust/ko/traits.html) 및 [트레이트 바운드](https://google.github.io/comprehensive-rust/ko/traits/trait-bounds.html)
    *   [표준 트레이트 구현 (`Default`, `Clone`, `Drop`, `Iterator`)](https://google.github.io/comprehensive-rust/ko/traits/standard-traits.html)
    *   [트레이트 객체 (`dyn Trait`)](https://google.github.io/comprehensive-rust/ko/traits/trait-objects.html)
*   **실습 태스크 (LearningRust 프로젝트):**
    *   **Queue 고도화**: `Q<T>`에 `Iterator` 트레이트를 구현하여 `for item in queue { ... }`가 가능하게 만들기.
    *   **Drop 구현**: `Q<T>`가 메모리에서 해제될 때 메시지를 출력하는 `Drop` 트레이트 구현하기.
    *   **Default 구현**: `Q::new()` 대신 `Default` 트레이트를 사용하여 기본 인스턴스 생성하기.

---

## 📅 3단계: 에러 핸들링과 함수형 기능 (Day 3 오후)
`match`를 이용한 에러 처리를 넘어, Rust의 에러 전파 방식과 반복자 패턴을 마스터합니다.

*   **학습 내용:**
    *   [에러 핸들링 (`?` 연산자, `Result`, `Option`)](https://google.github.io/comprehensive-rust/ko/error-handling.html)
    *   [반복자와 클로저 (Iterators and Closures)](https://google.github.io/comprehensive-rust/ko/iterators-and-closures.html)
*   **실습 태스크 (LearningRust 프로젝트):**
    *   **Error Propagation**: `fileio.rs`에서 `panic!` 대신 `Result<T, E>`와 `?` 연산자를 사용하여 에러를 상위(main)로 전달하기.
    *   **Iterator 활용**: `findprime`에서 소수를 찾는 로직을 반복자와 `filter`, `collect`를 사용하여 한 줄의 체인으로 작성해보기.

---

## 📅 4단계: 동시성과 비동기 프로그래밍 (Day 4)
이미 `mpsc`를 사용하셨으므로, 이제 공유 상태를 안전하게 관리하는 법과 `Async`를 배울 때입니다.

*   **학습 내용:**
    *   [공유 상태 (`Arc`, `Mutex`)](https://google.github.io/comprehensive-rust/ko/concurrency/shared-state.html)
    *   [비동기 프로그래밍 (Async/Await, Tokio)](https://google.github.io/comprehensive-rust/ko/async.html)
*   **실습 태스크 (LearningRust 프로젝트):**
    *   **Shared State**: 여러 스레드가 하나의 `Arc<Mutex<Vec<i32>>>`에 데이터를 동시에 쓰는 예제 작성하기.
    *   **Async 도입**: `tokio` 크레이트를 추가하고, 비동기 네트워크 요청이나 파일 읽기 시뮬레이션 해보기.

---

## 📅 5단계: 실무 최적화 및 생태계 (Bare Metal & Android)
Google 가이드의 후반부는 실제 환경(Android, Bare Metal)에 맞춘 내용입니다. 일반 개발자라면 생태계 도구에 집중합니다.

*   **학습 내용:**
    *   [Unsafe Rust](https://google.github.io/comprehensive-rust/ko/unsafe.html) (개념만 이해 권장)
    *   [Foreign Function Interface (FFI)](https://google.github.io/comprehensive-rust/ko/ffi.html) (C 라이브러리 호출)
*   **실습 태스크 (LearningRust 프로젝트):**
    *   **프로젝트 구조화**: 현재 파일들을 `src/lib.rs`와 `src/bin/*.rs` 형태로 정리하여 정식 라이브러리 구조 갖추기.
    *   **성능 측정**: `cargo bench`를 사용하여 `findprime` 알고리즘의 성능을 벤치마킹하기.

---

### 💡 학습 팁
1.  **Compiler is your friend**: 에러 메시지를 끝까지 읽는 습관을 들이세요. Rust 에러 메시지는 최고의 선생님입니다.
2.  **`cargo check`를 자주 활용하세요**: 빌드(`build`)보다 훨씬 빠르며, 로직의 문법적 오류를 즉각 잡아줍니다.
3.  **코드 리뷰 요청**: 작성하신 코드를 지금처럼 AI나 커뮤니티에 리뷰 요청하며 "더 Rust다운(Idiomatic) 방식"이 무엇인지 끊임없이 고민해보세요.
