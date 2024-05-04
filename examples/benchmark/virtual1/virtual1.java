class A {
  public static void f() {
  }
}

class B extends A {
  public static void f() {
    assert false;
  }
}

class virtual1 {
  public static void test(A a, B b) {
    a.f();
  }
}
