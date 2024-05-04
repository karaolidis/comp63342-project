class A {
  public void f() {}
}

class B extends A {
  public void f() {
    assert false;
  }
}

class virtual1 {
  public static void test(A a, B b) {
    a.f();
  }
}
