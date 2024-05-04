class A extends RuntimeException {
}

class B extends A {
}

class C extends B {
}

public class exceptions14 {
  public static void test() {
    C c = new C();
    A a = new A();
  }
}
