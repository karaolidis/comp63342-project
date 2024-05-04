interface A {
  void f();
}

class B implements A {
  public static void f() {
    assert false;
  }
}

class C implements A {
  public static void f() {
  }
}

public class virtual4 {
  public static void test(A b, A c) {
    c.f();
  }
}
