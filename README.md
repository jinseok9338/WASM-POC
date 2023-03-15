# WASM- POC
`WASM-POC` 는 몇가지 개념 증명(Proof of Concept)을 하고 싶어서 만든 레포(Repository)이다. 

## 배경 
1. Web Assembly 를 이용하여 웹 앱을 만드는 방식은 크게 2가지로 분류 된다. 기존의 javascript 로 쓰여진 앱에 웹 어셈블리로 컴파일 된 타 언어 코드( 여기서는 Rust 이다) 를 주입하는 방식, 혹은 처음부터 Yew 혹은 Leptos 를 이용하여 전체 코드를 러스트로 작성 하는 방식이 있다. 
Yew 는 특히 React 와 같은 Class Component( Struct Component), Functional Component, use_state 와 같은 hook 등을 제공 하기 때문에 React 환경에 익숙한 개발자면 쉽게 넘어 올 수 가 있다. (물론 Rust 언어의 장벽은 그대로 존재 한다.)
2. 혹자는 여기서 Yew 를 이용하여 Web App 을 만들려고 하였다.
3. 그러나 Javascript 에 의존성이 있는 기능을 구현할 수 있을까 하는 의문이 생겼다. -> 예를 들어 Kakao 소셜 로그인은 웹에서 구현 하려면 Kakao 가 제공하는 Javascript SDK 에 의존 하는 방법 밖에 없다. 
4. 또한 Rust 에서도 강력한 crate 라는 패키지 매니저가 있지만 과연 npm package 를 써야 하는 경우 npm 모듈을 사용할 수 있을까라는 의문이 생겼다.

## 목표
1. Javascript SDK 를 Rust 코드 베이스에서 쓸 수 있을까?
2. Npm 모듈을 Rust 코드 베이스에서 쓸 수 있을까?

## 결론 
결론부터 말하자면 두가지 다 가능 하다.

