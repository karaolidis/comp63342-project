class A {
}

class B extends A {
}

class C extends B {
}

public class ClassCastException2 {
  public void test(A c) {
    B b = (B) c;
    assert b != null;
  }
}
